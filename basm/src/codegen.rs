use crate::*;
use colored::*;

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
            let map = LABEL_MAP.lock().unwrap();
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
                "asciiz" | "word" => 0,
                _ => return Err((line_num, "Directive not recognized".to_string())),
            };
            Ok(label_val)
        }
        Some(Token::MemPointer(mem)) => Ok((1 << 7) | mem),
        Some(Token::RegPointer(reg)) => Ok((1 << 6) | reg),
        Some(Token::Ident(ident)) => {
            let map = LABEL_MAP.lock().unwrap();
            let vmap = VARIABLE_MAP.lock().unwrap();
            if let Some(&address) = map.get(ident) {
                Ok(address as i16)
            } else if let Some(&value) = vmap.get(ident) {
                Ok(value as i16)
            } else {
                return Err((
                    line_num,
                    format!("label/variable \"{}\" not declared.", ident.magenta()),
                ));
            }
        }
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
            _ => Err((
                line_num,
                format!("Instruction \"{}\" not recognized", instruction.magenta()),
            )),
        },
        Token::Directive(s) => {
            match s.as_str() {
                "asciiz" => ins_type = "ascii",
                "word" => ins_type = "word",
                "data" => ins_type = "data",
                "dataword" => ins_type = "dataword",
                "pad" => ins_type = "pad",
                _ => ins_type = "directive",
            }

            Ok(HLT_OP)
        }
        _ => {
            let inst = match ins {
                Token::EqualSign => "equals".to_string(),
                Token::Ident(_) => "identifier".to_string(),
                Token::Register(_) => "register".to_string(),
                Token::Comma => "comma".to_string(),
                Token::Literal(_) => "literal".to_string(),
                Token::NewLine => "newline".to_string(),
                Token::Eol => "eol".to_string(),
                Token::SRCall(_) => "label reference".to_string(),
                Token::MemAddr(_) => "memory address".to_string(),
                Token::Directive(_) => "directive".to_string(),
                Token::RegPointer(_) => "register indirect".to_string(),
                Token::MemPointer(_) => "memory indirect".to_string(),
                Token::Asciiz(_) => "ascii string".to_string(),
            };
            return Err((
                line_num,
                format!("expected ident, found {}", inst.bright_green()),
            ));
        }
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
        "directive" => {
            let arg_bin = argument_to_binary(Some(ins), line_num)?;
            if arg_bin != 1 {
                Ok(Some(vec![
                    (instruction_bin << 12) | (arg_bin << 9) | argument_to_binary(arg1, line_num)?,
                ]))
            } else {
                Ok(None)
            }
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
        "data" => {
            if arg1.is_none() {
                return Err((line_num, ".data argument is empty".to_string()));
            }
            let mut collected: Vec<i16> = Vec::new();
            for character in arg1.unwrap().get_raw().chars() {
                collected.push((1 << 8) | (character as i16));
            }
            Ok(Some(collected))
        }
        "pad" => {
            if arg1.is_none() {
                return Err((line_num, ".pad argument is empty".to_string()));
            }
            let collected: Vec<i16> = vec![0; arg1.unwrap().get_num() as usize];
            Ok(Some(collected))
        }
        "dataword" => {
            if arg1.is_none() {
                return Err((line_num, "DataWord argument is empty".to_string()));
            }
            Ok(Some(vec![(1 << 8) | arg1.unwrap().get_num()]))
        }
        _ => Err((line_num, "Instruction type not recognized".to_string())),
    }
}
