use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub static SUBROUTINE_MAP: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static MEMORY_COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
static START_LOCATION: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

pub fn argument_to_binary(arg: Option<&Token>, line_num: u32) -> Result<i16, String> {
    match arg {
        Some(Token::Register(num)) => {
            if *num > 7 {
                return Err(format!(
                    "Register number cannot be greater than 7 at line {}",
                    line_num
                ));
            }
            Ok(*num)
        }
        Some(Token::Literal(literal)) => Ok((1 << 8) | *literal),
        Some(Token::SR(sr) | Token::SRCall(sr)) => {
            let map = SUBROUTINE_MAP.lock().unwrap();
            if let Some(&address) = map.get(sr) {
                Ok(address as i16)
            } else {
                Err(format!(
                    "Subroutine \"{}\" does not exist at line {}",
                    sr, line_num
                ))
            }
        }
        Some(Token::MemAddr(n)) => Ok(*n),
        Some(Token::Label(keyword)) => {
            let label_val: i16 = match keyword.as_str() {
                "start" => 1,
                "ssp" => 2,
                "sbp" => 3,
                _ => {
                    return Err(format!(
                        "Label not recognized after '.' at line {}",
                        line_num
                    ))
                }
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
    line_num: u32,
) -> Result<Option<i16>, String> {
    let mut ins_type = "default";
    let instruction_bin = match ins {
        Token::Ident(ref instruction) => match instruction.to_uppercase().as_str() {
            "SSP" => {
                ins_type = "sp";
                Ok(RET_OP << 1 | 1)
            }
            "SBP" => {
                ins_type = "sp";
                Ok(NOP_OP << 1 | 1)
            }
            "HLT" => Ok(HLT_OP), // 0
            "ADD" => Ok(ADD_OP), // 1
            "JO" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    ins_type = "call";
                } else if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "jwr";
                }
                Ok(JO_OP) // 2
            }
            "POP" => {
                ins_type = "one_arg";
                Ok(POP_OP) // 3
            }
            "DIV" => Ok(DIV_OP), // 4
            "RET" | "ET" => Ok(RET_OP), // 5
            "LD" => Ok(LD_OP),   // 6
            "ST" => {
                if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "sti";
                } else {
                    ins_type = "st";
                }
                Ok(ST_OP) // 7
            }
            "JMP" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    ins_type = "call";
                } else if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "jwr";
                }
                Ok(JMP_OP)
            }
            "JZ" => {
                ins_type = "one_arg";
                if let Some(&Token::SRCall(_)) = arg1.or(arg2) {
                    ins_type = "call";
                } else if let Some(&Token::RegPointer(_)) = arg1.or(arg2) {
                    ins_type = "jwr";
                }
                Ok(JZ_OP) // 9
            }
            "CMP" => Ok(CMP_OP), // 10
            "MUL" => Ok(MUL_OP), // 11
            "PUSH" => {
                ins_type = "one_arg";
                Ok(PUSH_OP) // 12
            }
            "INT" => {
                ins_type = "one_arg";
                Ok(INT_OP) // 13
            }
            "NOP" => Ok(NOP_OP),
            "MOV" => Ok(MOV_OP), // 14
            _ => Err(format!("Instruction not recognized at line {}", line_num)),
        },
        Token::SR(_) => {
            ins_type = "subr";
            Ok(0)
        }
        Token::Label(_) => {
            ins_type = "label";
            Ok(HLT_OP)
        }
        _ => Err(format!("Invalid instruction type at line {}", line_num)),
    }?;

    match ins_type.trim().to_lowercase().as_str() {
        "one_arg" => {
            let arg_bin = argument_to_binary(arg1, line_num)?;
            Ok(Some((instruction_bin << 12) | arg_bin))
        }
        "st" => {
            let arg1_bin = argument_to_binary(arg1, line_num)?;
            let arg2_bin = argument_to_binary(arg2, line_num)?;
            Ok(Some((instruction_bin << 12) | (arg1_bin << 3) | arg2_bin))
        }
        "sti" => {
            let raw = arg1
                .ok_or_else(|| format!("Missing argument for STI at line {}", line_num))?
                .get_raw();
            let parsed_int = raw
                .trim()
                .parse::<i16>()
                .map_err(|_| format!("Failed to parse integer at line {}", line_num))?;
            Ok(Some(
                (instruction_bin << 12)
                    | (1 << 11)
                    | (argument_to_binary(Some(&Token::Register(parsed_int)), line_num)? << 7)
                    | argument_to_binary(arg2, line_num)?,
            ))
        }
        "label" => {
            let arg_bin = argument_to_binary(Some(ins), line_num)?;
            Ok(Some(
                (instruction_bin << 12) | (arg_bin << 9) | argument_to_binary(arg1, line_num)?,
            ))
        }
        "default" => {
            let arg1_bin = argument_to_binary(arg1, line_num)?;
            let arg2_bin = argument_to_binary(arg2, line_num)?;
            Ok(Some((instruction_bin << 12) | (arg1_bin << 9) | arg2_bin))
        }
        "call" => {
            let address = argument_to_binary(arg1, line_num)?;
            Ok(Some((instruction_bin << 12) | address))
        }
        "jwr" => {
            let raw_str = arg1
                .ok_or_else(|| format!("Missing argument for JWR at line {}", line_num))?
                .get_raw();
            let parsed_int = raw_str
                .trim()
                .parse::<i16>()
                .map_err(|_| format!("Failed to parse integer for JWR at line {}", line_num))?;
            Ok(Some(
                (instruction_bin << 12)
                    | 1 << 11
                    | argument_to_binary(Some(&Token::Register(parsed_int)), line_num)?,
            ))
        }
        "sp" => {
            Ok(Some((instruction_bin << 11) | arg1.unwrap().get_num())) // this was verified in verify.rs
                                                                        // unwrapping is safe
        }
        _ => Err(format!(
            "Instruction type not recognized at line {}",
            line_num
        )),
    }
}

pub fn process_start(lines: &[String]) -> Result<(), String> {
    let mut start_number: Option<i32> = None;

    for line in lines {
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
                if stripped.starts_with('[') && stripped.ends_with(']') {
                    stripped[1..stripped.len() - 1].parse::<i32>().ok()
                } else {
                    stripped.parse::<i32>().ok()
                }
            });
        }
    }

    if let Some(num) = start_number {
        let mut start_location = START_LOCATION
            .lock()
            .map_err(|_| "Failed to lock START_LOCATION")?;
        *start_location = num;
    }

    Ok(())
}

pub fn load_subroutines(lines: &[String]) -> Result<(), String> {
    let mut subroutine_counter = *START_LOCATION
        .lock()
        .map_err(|_| "Failed to lock START_LOCATION")? as u32;
    let mut subroutine_map = SUBROUTINE_MAP
        .lock()
        .map_err(|_| "Failed to lock SUBROUTINE_MAP")?;

    for line in lines {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with(';') || trimmed_line.starts_with('.')
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
            subroutine_counter -= 1;
        }

        subroutine_counter += 1;
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
