use crate::InstructionArgument::*;
use crate::*;
use colored::*;
type CodeGenError = ParserError;
type CodeGenResult = Result<i16, (Box<CodeGenError>, Vec<(String, Range<usize>)>)>;
use std::ops::Range;
pub fn encode_instruction(
    fname: &String,
    opcode: &i16,
    class: &u8,
    args: &[(InstructionArgument, Range<usize>)],
) -> CodeGenResult {
    let lhs = args.first();
    let rhs = args.get(1);
    let mut encoded;
    let l_map = LABEL_MAP.lock().unwrap();
    match *class {
        HLT_TYPE => {
            encoded = opcode << 12;
        }
        MOV_TYPE => {
            encoded = opcode << 12;
            if let Some((Reg(r), _)) = lhs {
                encoded |= (*r as i16) << 9;
            } else {
                gen_ice!("CANNOT RETRIEVE REGISTER OPERAND FROM MOV TYPE INS");
            }
            if let Some((arg, f)) = rhs {
                match arg {
                    Reg(r) => encoded |= *r as i16,
                    IReg(r) => encoded = encoded | (1 << 6) | (*r as i16),
                    Mem(m) => {
                        if let Some(v) = m.data.first() {
                            encoded = encoded | (1 << 7) | (v.0.get_value() as i16);
                        } else {
                            return Err((
                                Box::new(CodeGenError {
                                    file: fname.to_string(),
                                    help: None,
                                    input: read_file(fname),
                                    message: String::from(
                                        "MOV type instruction appears to have empty memory",
                                    ),
                                    start_pos: f.start,
                                    last_pos: f.end,
                                }),
                                vec![],
                            ));
                        }
                    }
                    Imm(_) => encoded = encoded | (1 << 8) | arg.get_imm(),
                    _ => gen_ice!("MOV TYPE INSTRUCTION HAS EMPTY/INVALID RHS"),
                }
            }
        }
        B_TYPE => {
            encoded = opcode << 11;
            match &lhs.unwrap().0 {
                // safe to unwrap - checked earlier
                Ident(i) => {
                    if let Some((name, span, value)) = l_map.get(i) {
                        if *value >= 1024 {
                            return Err((
                                Box::new(CodeGenError {
                                    file: name.to_string(),
                                    help: None,
                                    input: read_file(name),
                                    message: format!(
                                        "the address of label \"{i}\" cannot fit within 10 bits"
                                    ),
                                    start_pos: span.start,
                                    last_pos: span.end,
                                }),
                                vec![],
                            ));
                        } else {
                            encoded |= *value as i16;
                        }
                    } else {
                        std::mem::drop(l_map);
                        let info = if let (Some(s), _) = find_similar_entries(i) {
                            Some(format!("{} {s}", "╮".bright_red()))
                        } else {
                            None
                        };
                        return Err((
                            Box::new(CodeGenError {
                                file: fname.to_string(),
                                help: info,
                                input: read_file(fname),
                                message: format!("cannot find label \"{i}\""),
                                start_pos: args.first().unwrap().1.start,
                                last_pos: args.first().unwrap().1.end,
                            }),
                            find_similar_entries(i).1,
                        ));
                    }
                }
                Mem(m) => {
                    if let Some(v) = m.data.first() {
                        encoded |= v.0.get_value() as i16; // value limits are checked earlier
                    } else {
                        return Err((
                            Box::new(CodeGenError {
                                file: fname.to_string(),
                                help: Some(String::from(
                                    "branch instructions expect memory or register indirects",
                                )),
                                input: read_file(fname),
                                message: String::from("branch instruction memory appears empty"),
                                start_pos: args.first().unwrap().1.start,
                                last_pos: args.first().unwrap().1.end,
                            }),
                            vec![],
                        ));
                    }
                }
                IReg(r) => {
                    encoded = encoded | (1 << 10) | (*r as i16);
                }
                _ => gen_ice!("BRANCH INSTRUCTION HAS INVALID LHS"),
            }
        }
        POP_TYPE => {
            encoded = opcode << 12;
            match &lhs.unwrap().0 {
                Ident(i) => {
                    if let Some((name, span, value)) = l_map.get(i) {
                        if *value >= 2048 {
                            return Err((
                                Box::new(CodeGenError {
                                    file: name.to_string(),
                                    help: None,
                                    input: read_file(name),
                                    message: format!(
                                        "the address of label \"{i}\" cannot fit within 11 bits"
                                    ),
                                    start_pos: span.start,
                                    last_pos: span.end,
                                }),
                                vec![],
                            ));
                        } else {
                            encoded = encoded | (1 << 11) | (*value as i16);
                        }
                    } else {
                        return Err((
                            Box::new(CodeGenError {
                                file: fname.to_string(),
                                help: None,
                                input: read_file(fname),
                                message: format!("cannot find label \"{i}\""),
                                start_pos: args.first().unwrap().1.start,
                                last_pos: args.first().unwrap().1.end,
                            }),
                            vec![],
                        ));
                    }
                }

                Mem(m) => {
                    if let Some(v) = m.data.first() {
                        encoded = encoded | (1 << 11) | v.0.get_value() as i16;
                    } else {
                        return Err((
                            Box::new(CodeGenError {
                                file: fname.to_string(),
                                help: None,
                                input: read_file(fname),
                                message: String::from("POP instruction memory appears empty"),
                                start_pos: args.first().unwrap().1.start,
                                last_pos: args.first().unwrap().1.end,
                            }),
                            vec![],
                        ));
                    }
                }

                Reg(r) => encoded |= *r as i16,
                _ => {
                    gen_ice!("POP INSTRUCTION HAS INVALID LHS - THIS SHOULD'VE BEEN CAUGHT EARLIER")
                }
            }
        }
        LD_TYPE => {
            encoded = opcode << 12;
            if let Some((Reg(r), _)) = lhs {
                encoded |= (*r as i16) << 9;
            } else {
                gen_ice!("LD INSTRUCTION LHS DOES NOT APPEAR TO BE REGISTER");
            }
            match &rhs.unwrap().0 {
                Ident(i) => {
                    if let Some((name, span, value)) = l_map.get(i) {
                        if *value >= 512 {
                            return Err((
                                Box::new(CodeGenError {
                                    file: name.to_string(),
                                    help: None,
                                    input: read_file(name),
                                    message: format!(
                                        "the address of label \"{i}\" cannot fit within 9 bits"
                                    ),
                                    start_pos: span.start,
                                    last_pos: span.end,
                                }),
                                vec![],
                            ));
                        } else {
                            encoded |= *value as i16;
                        }
                    } else {
                        std::mem::drop(l_map);
                        let info = if let (Some(s), _) = find_similar_entries(i) {
                            Some(format!("{} {s}", "╮".bright_red()))
                        } else {
                            None
                        };
                        return Err((
                            Box::new(CodeGenError {
                                file: fname.to_string(),
                                help: info,
                                input: read_file(fname),
                                message: format!("cannot find label \"{i}\""),
                                start_pos: args.get(1).unwrap().1.start,
                                last_pos: args.get(1).unwrap().1.end,
                            }),
                            find_similar_entries(i).1,
                        ));
                    }
                }

                Mem(m) => {
                    if let Some(v) = m.data.first() {
                        encoded |= v.0.get_value() as i16;
                    } else {
                        return Err((
                            Box::new(CodeGenError {
                                file: fname.to_string(),
                                help: None,
                                input: read_file(fname),
                                message: String::from("LD/LEA instruction memory appears empty"),
                                start_pos: args.get(1).unwrap().1.start,
                                last_pos: args.get(1).unwrap().1.end,
                            }),
                            vec![],
                        ));
                    }
                }
                _ => gen_ice!(
                    "LEA/LD INSTRUCTION HAS INVALID RHS - THIS SHOULD'VE BEEN CAUGHT EARLIER"
                ),
            }
        }
        MOV_TYPE_ONE => {
            encoded = opcode << 12;
            match &lhs.unwrap().0 {
                Imm(_) => encoded = encoded | (1 << 8) | lhs.unwrap().0.get_imm(),
                Reg(r) => encoded |= *r as i16,
                _ => gen_ice!("MOV TYPE ONE DID NOT HAVE ARG"),
            }
        }
        ST_TYPE => {
            encoded = opcode << 12;
            match &lhs.unwrap().0 {
                Ident(i) => {
                    if let Some((name, span, value)) = l_map.get(i) {
                        if *value >= 256 {
                            return Err((
                                Box::new(CodeGenError {
                                    file: name.to_string(),
                                    help: None,
                                    input: read_file(name),
                                    message: format!(
                                        "the address of label \"{i}\" cannot fit within 8 bits"
                                    ),
                                    start_pos: span.start,
                                    last_pos: span.end,
                                }),
                                vec![],
                            ));
                        } else {
                            encoded |= (*value as i16) << 3;
                        }
                    } else {
                        std::mem::drop(l_map);
                        let info = if let (Some(s), _) = find_similar_entries(i) {
                            Some(format!("{} {s}", "╮".bright_red()))
                        } else {
                            None
                        };
                        return Err((
                            Box::new(CodeGenError {
                                file: fname.to_string(),
                                help: info,
                                input: read_file(fname),
                                message: format!("cannot find label \"{i}\""),
                                start_pos: args.first().unwrap().1.start,
                                last_pos: args.first().unwrap().1.end,
                            }),
                            find_similar_entries(i).1,
                        ));
                    }
                }

                Mem(m) => {
                    encoded |= (m.data.first().unwrap().0.get_value() as i16) << 3;
                }

                IReg(r) => {
                    encoded = encoded | (1 << 11) | ((*r as i16) << 7);
                }
                _ => {
                    gen_ice!("ST INSTRUCTION HAS INVALID LHS - THIS SHOULD'VE BEEN CAUGHT EARLIER")
                }
            }
            match &rhs.unwrap().0 {
                Reg(r) => encoded |= *r as i16,
                _ => gen_ice!("Right hand side of ST is not a register"),
            }
        }
        _ => gen_ice!("INVALID INSTRUCTION TYPE IN CODEGEN"),
    }
    Ok(encoded)
}
