use crate::*;
use colored::*;
type TempErr = (String, String);
type CodeGenResult = Result<Option<Vec<i16>>, (usize, Option<Vec<usize>>, TempErr)>;
pub fn argument_to_binary(
    arg: Option<&Token>,
    line_num: usize,
) -> Result<i16, (usize, Option<Vec<usize>>, TempErr)> {
    match arg {
        Some(Token::Register(num)) => {
            if *num > 9 {
                return Err((
                    line_num,
                    None,
                    (
                        format!("invalid register number {num}"),
                        format!("valid registers are {}", "r0-r9".magenta()),
                    ),
                ));
            }
            Ok(*num)
        }
        Some(Token::Literal(literal)) => Ok((1 << 8) | *literal),
        Some(Token::SRCall(sr)) => {
            let map = LABEL_MAP.lock().unwrap();
            if let Some((_, address)) = map.get(sr) {
                Ok(*address as i16)
            } else {
                let similars = find_closest_matches(&map, sr, 2);
                let mut founds = String::from("");
                let mut found_lines: Vec<usize> = Vec::new();
                for (line, element) in similars {
                    found_lines.push(line + 1);
                    if !founds.is_empty() {
                        founds = format!("{founds}, ");
                    }
                    founds = format!("{founds}{}:{}", element.green(), line + 1);
                }
                founds = if founds.is_empty() {
                    String::from("")
                } else {
                    format!("similar labels exist: {founds}")
                };
                Err((
                    line_num,
                    Some(found_lines),
                    (format!("label \"{}\" does not exist", sr.magenta()), founds),
                ))
            }
        }
        Some(Token::MemAddr(n)) => Ok(*n),
        Some(Token::Directive(keyword)) => {
            let label_val: i16 = match keyword.as_str() {
                "start" => 1,
                "asciiz" | "word" => 0,
                _ => {
                    return Err((
                        line_num,
                        None,
                        ("directive not recognized".to_string(), "".to_string()),
                    ))
                }
            };
            Ok(label_val)
        }
        Some(Token::MemPointer(mem)) => Ok((1 << 7) | mem),
        Some(Token::RegPointer(reg)) => Ok((1 << 6) | reg),
        Some(Token::Ident(ident)) => {
            let map = LABEL_MAP.lock().unwrap();
            let vmap = VARIABLE_MAP.lock().unwrap();
            if let Some((_, address)) = map.get(ident) {
                Ok(*address as i16)
            } else if let Some((_, value)) = vmap.get(ident) {
                Ok(*value as i16)
            } else {
                let similars = find_closest_matches(&map, ident, 2);
                let mut founds = String::from("");
                let mut found_lines: Vec<usize> = Vec::new();
                for (line, element) in similars {
                    found_lines.push(line + 1);
                    if !founds.is_empty() {
                        founds = format!("{founds}, ");
                    }
                    founds = format!("{founds}{}:{}", element.green(), line + 1);
                }
                founds = if founds.is_empty() {
                    String::from("")
                } else {
                    format!("similar labels exist: {founds}")
                };
                let mut total_founds = founds;
                let similars = find_closest_matches_i32(&vmap, ident, 2);
                let mut founds = String::from("");
                for (line, element) in similars {
                    found_lines.push(line + 1);
                    if !founds.is_empty() {
                        founds = format!("{founds}, ");
                    }
                    founds = format!("{founds}{}:{}", element.green(), line + 1);
                }
                founds = if founds.is_empty() {
                    String::from("")
                } else {
                    format!("similar variables exist: {founds}")
                };
                total_founds = if !founds.is_empty() {
                    format!("{}, {}", total_founds, founds)
                } else {
                    total_founds
                };

                return Err((
                    line_num,
                    Some(found_lines),
                    (
                        format!("label/variable \"{}\" not declared", ident.magenta(),),
                        total_founds,
                    ),
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
) -> CodeGenResult {
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
                    _ => {
                        let instructions = [
                            "BO", "BNO", "JMP", "J", "BZ", "BE", "BEQ", "BNZ", "BNE", "BL", "BG",
                        ];
                        let mut matches: Vec<(String, usize)> = instructions
                            .iter()
                            .map(|instructions| {
                                (
                                    instructions.to_string(),
                                    levenshtein_distance(
                                        instruction.to_uppercase().as_str(),
                                        instructions,
                                    ),
                                )
                            })
                            .filter(|(_, dist)| *dist <= 1)
                            .collect();
                        matches.sort_by_key(|&(_, dist)| dist);

                        let closest_matches: Vec<String> =
                            matches.into_iter().map(|(word, _)| word).collect();
                        let result = if closest_matches.is_empty() {
                            "".to_string()
                        } else {
                            format!("maybe you meant: {}", closest_matches.join(", ").green())
                        };

                        return Err((
                            line_num,
                            None,
                            (
                                format!(
                                    "invalid jump/branch instruction \"{}\"",
                                    instruction.magenta()
                                ),
                                result,
                            ),
                        ));
                    }
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
            _ => {
                let instructions = [
                    "ADD", "HLT", "BO", "POP", "DIV", "RET", "LD", "ST", "JMP", "BZ", "PUSH",
                    "CMP", "NAND", "INT", "MOV", "LEA", "BE", "BNE", "BNZ", "BNO", "BG", "BL",
                ];
                let mut matches: Vec<(String, usize)> = instructions
                    .iter()
                    .map(|instructions| {
                        (
                            instructions.to_string(),
                            levenshtein_distance(instruction.to_uppercase().as_str(), instructions),
                        )
                    })
                    .filter(|(_, dist)| *dist <= 2)
                    .collect();
                matches.sort_by_key(|&(_, dist)| dist);

                let closest_matches: Vec<String> =
                    matches.into_iter().map(|(word, _)| word).collect();
                let result = if closest_matches.is_empty() {
                    "".to_string()
                } else {
                    format!("maybe you meant: {}", closest_matches.join(", ").green())
                };

                return Err((
                    line_num,
                    None,
                    (
                        format!("instruction \"{}\" not recognized", instruction.magenta(),),
                        result,
                    ),
                ));
            }
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
                None,
                (
                    format!("expected ident, found {}", inst.bright_green()),
                    "please provide a directive or identifier".to_string(),
                ),
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
                .ok_or_else(|| {
                    (
                        line_num,
                        None,
                        (
                            "missing argument for POP".to_string(),
                            "please provide a memory address or register argument".to_string(),
                        ),
                    )
                })?
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
                .ok_or_else(|| {
                    (
                        line_num,
                        None,
                        (
                            "missing argument for store indirect".to_string(),
                            "provide a memory address LHS and register indirect RHS".to_string(),
                        ),
                    )
                })?
                .get_raw();
            let parsed_int = raw.trim().parse::<i16>().map_err(|_| {
                (
                    line_num,
                    None,
                    (
                        "failed to parse integer".to_string(),
                        "please provide a proper integer".to_string(),
                    ),
                )
            })?;
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
                    None,
                    (
                        "label memory address too large on instruction on line".to_string(),
                        "".to_string(),
                    ),
                ));
            }
            Ok(Some(vec![(instruction_bin << 11) | address]))
        }
        "jwr" => {
            let raw_str = arg1
                .ok_or_else(|| {
                    (
                        line_num,
                        None,
                        (
                            "missing argument for indirect branch/jump".to_string(),
                            "please provide a register indirect argument".to_string(),
                        ),
                    )
                })?
                .get_raw();
            let parsed_int = raw_str.trim().parse::<i16>().map_err(|_| {
                (
                    line_num,
                    None,
                    (
                        "failed to parse integer for indirect jump".to_string(),
                        "please provide a register indirect argument".to_string(),
                    ),
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
                return Err((
                    line_num,
                    None,
                    (
                        "asciiz argument is empty".to_string(),
                        "please provide an ASCII string argument".to_string(),
                    ),
                ));
            }
            let mut collected: Vec<i16> = Vec::new();
            for character in arg1.unwrap().get_raw().chars() {
                collected.push(character as i16);
            }
            Ok(Some(collected))
        }
        "word" => {
            if arg1.is_none() {
                return Err((
                    line_num,
                    None,
                    (
                        "word argument is empty".to_string(),
                        "please provide a 16-bit argument".to_string(),
                    ),
                ));
            }
            Ok(Some(vec![arg1.unwrap().get_num()]))
        }
        "ld" => {
            if arg1.is_none() || arg2.is_none() {
                return Err((
                    line_num,
                    None,
                    (
                        "LEA/LD argument is empty".to_string(),
                        "please provide a register LHS and address RHS".to_string(),
                    ),
                ));
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
                return Err((
                    line_num,
                    None,
                    (
                        ".data argument is empty".to_string(),
                        "please provide an ASCII string argument".to_string(),
                    ),
                ));
            }
            let mut collected: Vec<i16> = Vec::new();
            for character in arg1.unwrap().get_raw().chars() {
                collected.push((1 << 8) | (character as i16));
            }
            Ok(Some(collected))
        }
        "pad" => {
            if arg1.is_none() {
                return Err((
                    line_num,
                    None,
                    (
                        ".pad argument is empty".to_string(),
                        "please provide an argument".to_string(),
                    ),
                ));
            }
            let collected: Vec<i16> = vec![0; arg1.unwrap().get_num() as usize];
            Ok(Some(collected))
        }
        "dataword" => {
            if arg1.is_none() {
                return Err((
                    line_num,
                    None,
                    (
                        "dataword argument is empty".to_string(),
                        "please provide a 16-bit argument".to_string(),
                    ),
                ));
            }
            Ok(Some(vec![(1 << 8) | arg1.unwrap().get_num()]))
        }
        _ => unsafe { std::hint::unreachable_unchecked() },
    }
}
