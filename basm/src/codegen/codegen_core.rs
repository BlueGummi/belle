use crate::*;
use colored::*;
pub const HLT_OP: i16 = 0b0000;
pub const ADD_OP: i16 = 0b0001;
pub const BO_OP: i16 = 0b00100;
pub const BNO_OP: i16 = 0b00101;
pub const POP_OP: i16 = 0b0011;
pub const DIV_OP: i16 = 0b0100;
pub const RET_OP: i16 = 0b0101;
pub const BL_OP: i16 = 0b01010;
pub const BG_OP: i16 = 0b01011;
pub const LD_OP: i16 = 0b0110;
pub const ST_OP: i16 = 0b0111;
pub const JMP_OP: i16 = 0b10000;
pub const BZ_OP: i16 = 0b10010;
pub const BNZ_OP: i16 = 0b10011;
pub const CMP_OP: i16 = 0b1010;
pub const NAND_OP: i16 = 0b1011;
pub const PUSH_OP: i16 = 0b1100;
pub const INT_OP: i16 = 0b1101;
pub const MOV_OP: i16 = 0b1110;
pub const LEA_OP: i16 = 0b1111;

pub const HLT_TYPE: u8 = 0;
pub const MOV_TYPE: u8 = 1;
pub const B_TYPE: u8 = 2;
pub const POP_TYPE: u8 = 3;
pub const LD_TYPE: u8 = 4;
pub const ST_TYPE: u8 = 5;
pub const MOV_TYPE_ONE: u8 = 6;
type CodeGenError = ParserError;
use std::ops::Range;
type CodeGenResult = Result<Vec<i16>, (Box<CodeGenError>, Vec<(String, Range<usize>)>)>;
pub fn encode(
    ins: (&String, &TokenKind, &Range<usize>),
    fname: &String,
    next_ins: &Option<&(String, TokenKind, Range<usize>)>,
) -> CodeGenResult {
    let mut encoded_tokens = Vec::new();
    match &ins.1 {
        TokenKind::Instruction(ins) => {
            let (opcode, ins_class) = match ins.name.to_lowercase().as_str() {
                // all instructions should be valid when this is reached, as it is validated in
                // validator/validator_ins.rs. therefore, it is fine to use MOV_TYPE for PUSH and INT
                // because the argument types and counts are validated.
                //
                //
                // But, I do need to check labels as they are not detected in previous instances
                //
                // I should also allow it to reference CONSTs too, and see if it is talking about a label
                // or a CONST (check labels first)
                //
                // ^^^^ I need to validate that there are no duplicate names across symbol tabels for
                // labels and constants
                "hlt" => (HLT_OP, HLT_TYPE),
                "add" => (ADD_OP, MOV_TYPE),
                "bo" => (BO_OP, B_TYPE),
                "bno" => (BNO_OP, B_TYPE),
                "pop" => (POP_OP, POP_TYPE),
                "div" => (DIV_OP, MOV_TYPE),
                "ret" => (RET_OP, HLT_TYPE),
                "bl" => (BL_OP, B_TYPE),
                "bg" => (BG_OP, B_TYPE),
                "ld" => (LD_OP, LD_TYPE),
                "st" => (ST_OP, ST_TYPE),
                "jmp" => (JMP_OP, B_TYPE),
                "bz" => (BZ_OP, B_TYPE),
                "bnz" => (BNZ_OP, B_TYPE),
                "cmp" => (CMP_OP, MOV_TYPE),
                "nand" => (NAND_OP, MOV_TYPE),
                "push" => (PUSH_OP, MOV_TYPE_ONE),
                "int" => (INT_OP, MOV_TYPE_ONE),
                "mov" => (MOV_OP, MOV_TYPE),
                "lea" => (LEA_OP, LD_TYPE),
                _ => gen_ice!(
                    "INSTRUCTION MATCH FAILED: {} WAS NOT RECOGNIZED.",
                    ins.name.to_uppercase().magenta()
                ),
            };
            encoded_tokens.push(encode_instruction(
                fname,
                &opcode,
                &ins_class,
                &ins.operands,
            )?);
        }
        TokenKind::Directive(name) => match name.to_lowercase().as_str() {
            "asciiz" => {
                let stri = match next_ins {
                    Some(thing) => thing.1.to_string(),
                    None => "no".to_string(),
                };
                if let Some((_, TokenKind::StringLit(_), _)) = next_ins {
                    for letter in next_ins.unwrap().1.get_str().chars() {
                        encoded_tokens.push(letter as i16);
                    }
                } else {
                    return Err((
                        Box::new(CodeGenError {
                            file: fname.to_string(),
                            help: Some(format!("found {} argument", stri.magenta())),
                            input: read_file(fname),
                            message: String::from(
                                "ASCIIZ directive must be succeeded by string literal",
                            ),
                            start_pos: ins.2.start,
                            last_pos: ins.2.end,
                        }),
                        vec![],
                    ));
                }
            }
            "pad" => {
                let stri = match next_ins {
                    Some(thing) => thing.1.to_string(),
                    None => "no".to_string(),
                };
                if let Some((_, TokenKind::IntLit(num), _)) = next_ins {
                    encoded_tokens.extend(vec![0; *num as usize]);
                } else {
                    return Err((
                        Box::new(CodeGenError {
                            file: fname.to_string(),
                            help: Some(format!("found {} argument", stri.magenta())),
                            input: read_file(fname),
                            message: String::from(
                                "PAD directive must be succeeded by integer literal",
                            ),
                            start_pos: ins.2.start,
                            last_pos: ins.2.end,
                        }),
                        vec![],
                    ));
                }
            }
            "word" => {
                encoded_tokens.push(next_ins.unwrap().1.get_value() as i16);
            }
            "start" => (),
            _ => gen_ice!("DIRECTIVE MATCH FAILED: {name} NOT RECOGNIZED"),
        },
        _ => {}
    }

    Ok(encoded_tokens)
}
