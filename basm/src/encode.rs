use crate::*;
use colored::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;
pub static SUBROUTINE_MAP: Lazy<Mutex<HashMap<String, usize>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static VARIABLE_MAP: Lazy<Mutex<HashMap<String, i32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MEMORY_COUNTER: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
static START_LOCATION: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

pub fn argument_to_binary(arg: Option<&Token>, line_num: usize) -> Result<i16, (usize, String)> {
    match arg {
        Some(Token::Register(num)) => {
            if *num > 9 {
                return Err((line_num, "Invalid register number".to_string()));
            }
            Ok(*num)
        }
        Some(Token::Literal(literal)) => Ok((1 << 8) | *literal),
        Some(Token::SRCall(sr)) => {
            let map = SUBROUTINE_MAP.lock().unwrap();
            if let Some(&address) = map.get(sr) {
                Ok(address as i16)
            } else {
                Err((line_num, format!("Label \"{}\" does not exist", sr)))
            }
        }
        Some(Token::MemAddr(n)) => Ok(*n),
        Some(Token::Directive(keyword)) => {
            let label_val: i16 = match keyword.as_str() {
                "start" => 1,
                "ssp" => 2,
                "sbp" => 3,
                "asciiz" | "word" => 0,
                _ => return Err((line_num, "Directive not recognized".to_string())),
            };
            Ok(label_val)
        }
        Some(Token::MemPointer(mem)) => Ok((1 << 7) | mem),
        Some(Token::RegPointer(reg)) => Ok((1 << 6) | reg),
        _ => Ok(0),
    }
}

pub fn encode_instruction(
    ins: &Token,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<Option<Vec<i16>>, (usize, String)> {
    let mut ins_type = "default";
    let instruction_bin = match ins {
        Token::Ident(ref instruction) => match instruction.to_uppercase().as_str() {
            "HLT" => Ok(HLT_OP), // 0
            "ADD" => Ok(ADD_OP), // 1
            instruction
                if instruction.to_uppercase().starts_with('J')
                    || instruction.to_uppercase().starts_with('B') =>
            {
                ins_type = "jump";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    ins_type = "call";
                } else if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "jwr";
                }
                match instruction.to_uppercase().as_str() {
                    "BO" => Ok(BO_OP),
                    "BNO" => Ok(BNO_OP),
                    "JMP" | "J" => Ok(JMP_OP),
                    "BZ" | "BE" | "BEQ" => Ok(BZ_OP),
                    "BNZ" | "BNE" => Ok(BNZ_OP),
                    "BL" => Ok(BL_OP),
                    "BG" => Ok(BG_OP),
                    _ => Err((line_num, "Invalid jump instruction".to_string())),
                }
            }
            "POP" => {
                ins_type = "one_arg";
                if let Some(&Token::MemAddr(_)) = arg1 {
                    ins_type = "popmem";
                }
                Ok(POP_OP) // 3
            }
            "DIV" => Ok(DIV_OP),        // 4
            "RET" | "ET" => Ok(RET_OP), // 5
            "LD" => {
                ins_type = "ld";
                Ok(LD_OP) // 6
            }
            "ST" => {
                if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "sti";
                } else {
                    ins_type = "st";
                }
                Ok(ST_OP) // 7
            }
            "CMP" => Ok(CMP_OP),   // 10
            "NAND" => Ok(NAND_OP), // 11
            "PUSH" => {
                ins_type = "one_arg";
                Ok(PUSH_OP) // 12
            }
            "INT" => {
                ins_type = "one_arg";
                Ok(INT_OP) // 13
            }
            "LEA" => {
                ins_type = "ld";
                Ok(LEA_OP)
            }
            "MOV" => Ok(MOV_OP), // 14
            _ => Err((line_num, "Instruction not recognized".to_string())),
        },
        Token::Directive(s) => {
            match s.as_str() {
                "asciiz" => ins_type = "ascii",
                "word" => ins_type = "word",
                _ => ins_type = "label",
            }

            Ok(HLT_OP)
        }
        _ => Err((line_num, "Invalid instruction type".to_string())),
    }?;

    match ins_type.trim().to_lowercase().as_str() {
        "one_arg" => {
            let arg_bin = argument_to_binary(arg1, line_num)?;
            Ok(Some(vec![(instruction_bin << 12) | arg_bin]))
        }
        "popmem" => {
            let arg_bin = arg1
                .ok_or_else(|| (line_num, "Missing argument for POP".to_string()))?
                .get_num();
            Ok(Some(vec![instruction_bin << 12 | 1 << 11 | arg_bin]))
        }
        "st" => {
            let arg1_bin = argument_to_binary(arg1, line_num)?;
            let arg2_bin = argument_to_binary(arg2, line_num)?;
            Ok(Some(vec![
                (instruction_bin << 12) | (arg1_bin << 3) | arg2_bin,
            ]))
        }
        "sti" => {
            let raw = arg1
                .ok_or_else(|| (line_num, "Missing argument for STI".to_string()))?
                .get_raw();
            let parsed_int = raw
                .trim()
                .parse::<i16>()
                .map_err(|_| (line_num, "Failed to parse integer".to_string()))?;
            Ok(Some(vec![
                (instruction_bin << 12)
                    | (1 << 11)
                    | (argument_to_binary(Some(&Token::Register(parsed_int)), line_num)? << 7)
                    | argument_to_binary(arg2, line_num)?,
            ]))
        }
        "label" => {
            let arg_bin = argument_to_binary(Some(ins), line_num)?;
            Ok(Some(vec![
                (instruction_bin << 12) | (arg_bin << 9) | argument_to_binary(arg1, line_num)?,
            ]))
        }
        "default" => {
            let arg1_bin = argument_to_binary(arg1, line_num)?;
            let arg2_bin = argument_to_binary(arg2, line_num)?;
            Ok(Some(vec![
                (instruction_bin << 12) | (arg1_bin << 9) | arg2_bin,
            ]))
        }
        "call" => {
            let address = argument_to_binary(arg1, line_num)?;
            if address > 1023 {
                return Err((
                    line_num,
                    "Label memory address too large on instruction on line".to_string(),
                ));
            }
            Ok(Some(vec![(instruction_bin << 11) | address]))
        }
        "jwr" => {
            let raw_str = arg1
                .ok_or_else(|| {
                    (
                        line_num,
                        "Missing argument for indirect branch/jump".to_string(),
                    )
                })?
                .get_raw();
            let parsed_int = raw_str.trim().parse::<i16>().map_err(|_| {
                (
                    line_num,
                    "Failed to parse integer for indirect jump".to_string(),
                )
            })?;
            Ok(Some(vec![
                (instruction_bin << 11)
                    | 1 << 10
                    | argument_to_binary(Some(&Token::Register(parsed_int)), line_num)?,
            ]))
        }
        "ascii" => {
            if arg1.is_none() {
                return Err((line_num, "Asciiz argument is empty".to_string()));
            }
            let mut collected: Vec<i16> = Vec::new();
            for character in arg1.unwrap().get_raw().chars() {
                collected.push(character as i16);
            }
            Ok(Some(collected))
        }
        "word" => {
            if arg1.is_none() {
                return Err((line_num, "Word argument is empty".to_string()));
            }
            Ok(Some(vec![arg1.unwrap().get_num()]))
        }
        "ld" => {
            if arg1.is_none() || arg2.is_none() {
                return Err((line_num, "LEA/LD argument is empty".to_string()));
            }
            let arg2_bin = argument_to_binary(arg2, line_num)?;
            let arg1_bin = argument_to_binary(arg1, line_num)?;
            Ok(Some(vec![
                (instruction_bin << 12) | (arg1_bin << 9) | arg2_bin,
            ]))
        }
        "jump" => {
            let arg_bin = argument_to_binary(arg1, line_num)?;
            Ok(Some(vec![(instruction_bin << 11) | arg_bin]))
        }
        _ => Err((line_num, "Instruction type not recognized".to_string())),
    }
}

pub fn process_start(lines: &[String]) -> Result<(), (usize, String)> {
    let mut start_number: Option<i32> = None;

    for (index, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with(';') {
            continue;
        }

        let line_before_comment = if trimmed_line.contains(';') {
            trimmed_line.split(';').next().unwrap_or(trimmed_line)
        } else {
            trimmed_line
        };

        if line_before_comment.starts_with(".start") {
            start_number = line_before_comment.split_whitespace().nth(1).and_then(|s| {
                let stripped = s.strip_prefix('$').unwrap_or(s);
                let stripped = s.strip_prefix('[').unwrap_or(stripped);
                let stripped = if stripped.ends_with(']') {
                    &stripped[0..stripped.len() - 1]
                } else {
                    stripped
                };
                if let Some(value) = stripped.strip_prefix("0b") {
                    i32::from_str_radix(value, 2)
                        .map_err(|_| {
                            (
                                index,
                                format!("Invalid .start directive binary number: {}", stripped),
                            )
                        })
                        .ok()
                } else if let Some(value) = stripped.strip_prefix("0x") {
                    i32::from_str_radix(value, 16)
                        .map_err(|_| {
                            (
                                index,
                                format!(
                                    "Invalid .start directive hexadecimal number: {}",
                                    stripped
                                ),
                            )
                        })
                        .ok()
                } else {
                    match stripped.parse::<i32>() {
                        Ok(n) => Some(n),
                        Err(_) => {
                            let vmap = VARIABLE_MAP.lock().unwrap();
                            if let Some(&replacement) = vmap.get(stripped.trim()) {
                                Some(replacement)
                            } else {
                                None
                            }
                        }
                    }
                }
            });
        }
    }

    if let Some(val) = start_number {
        if val > 511 {
            return Err((0, String::from("Start location must not exceed 511")));
        }
    }
    let mut start_location = START_LOCATION
        .lock()
        .map_err(|_| (0, "Failed to lock START_LOCATION".to_string()))?;

    *start_location = start_number.unwrap_or(100);

    Ok(())
}
pub fn load_subroutines(lines: &[String]) -> Result<(), String> {
    let mut subroutine_counter = *START_LOCATION
        .lock()
        .map_err(|_| "Failed to lock START_LOCATION")? as usize;
    let mut subroutine_map = SUBROUTINE_MAP
        .lock()
        .map_err(|_| "Failed to lock SUBROUTINE_MAP")?;

    for line in lines {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty()
            || trimmed_line.starts_with(';')
            || trimmed_line.starts_with(".ssp")
            || trimmed_line.starts_with(".sbp")
            || trimmed_line.starts_with(".start")
        {
            continue;
        }
        let line_before_comment = if trimmed_line.contains(';') {
            trimmed_line.split(';').next().unwrap_or(trimmed_line)
        } else {
            trimmed_line
        };
        if line_before_comment.trim().ends_with(':') && !line_before_comment.trim().contains(' ') {
            let subroutine_name = line_before_comment
                .trim()
                .trim_end_matches(':')
                .trim()
                .to_string();
            subroutine_map.insert(subroutine_name.trim().to_string(), subroutine_counter);
            continue;
        }
        if line_before_comment.starts_with(".asciiz") {
            if let Some(start) = line_before_comment.find('\"') {
                if let Some(end) = line_before_comment[start + 1..].find('\"') {
                    subroutine_counter += line_before_comment[start + 1..start + 1 + end].len();
                }
            }
            continue;
        }
        if line_before_comment.starts_with(".word") {
            subroutine_counter += 1;
            continue;
        }
        if !line_before_comment.trim().contains('=') {
            subroutine_counter += 1;
        }
    }

    Ok(())
}
pub fn process_variables(lines: &[String]) -> Result<(), (usize, String)> {
    let mut variable_map = VARIABLE_MAP
        .lock()
        .map_err(|_| (0, "Failed to lock VARIABLE_MAP".to_string()))?;

    for (index, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with(';') {
            continue;
        }

        let line_before_comment = if trimmed_line.contains(';') {
            trimmed_line.split(';').next().unwrap_or(trimmed_line)
        } else {
            trimmed_line
        };

        if let Some(eq_index) = line_before_comment.find('=') {
            let variable_name = line_before_comment[..eq_index].trim();
            let variable_value = line_before_comment[eq_index + 1..].trim();

            let parsed_value = if let Some(stripped) = variable_value.strip_prefix("0b") {
                i32::from_str_radix(stripped, 2)
                    .map_err(|_| (index, format!("Invalid binary number: {}", variable_value)))?
            } else if let Some(stripped) = variable_value.strip_prefix("0x") {
                i32::from_str_radix(stripped, 16).map_err(|_| {
                    (
                        index,
                        format!("Invalid hexadecimal number: {}", variable_value),
                    )
                })?
            } else {
                variable_value.parse::<i32>().map_err(|_| {
                    (
                        index,
                        format!("Invalid variable assignment: {}", line_before_comment),
                    )
                })?
            };

            variable_map.insert(variable_name.to_string(), parsed_value);
        }
    }

    Ok(())
}
pub fn update_memory_counter() -> Result<(), String> {
    let mut counter = MEMORY_COUNTER
        .lock()
        .map_err(|_| "Failed to lock MEMORY_COUNTER")?;
    *counter += 1;
    Ok(())
}

pub fn print_line(line_number: usize) -> io::Result<()> {
    let path = Path::new(&CONFIG.source);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for (current_line, line) in reader.lines().enumerate() {
        if current_line + 1 == line_number {
            match line {
                Ok(content) => {
                    println!(
                        "{:^6} {} {}\n",
                        (current_line + 1).to_string().blue(),
                        "|".blue(),
                        content
                    );
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    return Err(e);
                }
            }
        }
    }

    eprintln!("Line number {} is out of bounds.", line_number);
    Err(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "Line not found",
    ))
}
