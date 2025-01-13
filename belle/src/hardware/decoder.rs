use crate::{Argument::*, Instruction::*, *};
use std::fmt;

pub const HLT_OP: i16 = 0b0000; // we need this
pub const ADD_OP: i16 = 0b0001; // we also need this
pub const JO_OP: i16 = 0b0010; // maybe optional ?
pub const POP_OP: i16 = 0b0011; // we need this
pub const DIV_OP: i16 = 0b0100; // we need this
pub const RET_OP: i16 = 0b0101; // we need this
pub const LD_OP: i16 = 0b0110; // we need this
pub const ST_OP: i16 = 0b0111; // we need this
pub const JMP_OP: i16 = 0b1000; // maybe optional?
pub const JZ_OP: i16 = 0b1001; // we need this
pub const CMP_OP: i16 = 0b1010; // we need this
pub const MUL_OP: i16 = 0b1011; // we need this
pub const PUSH_OP: i16 = 0b1100; // we need this
pub const INT_OP: i16 = 0b1101; // we need this
pub const MOV_OP: i16 = 0b1110; // we need this
pub const LEA_OP: i16 = 0b1111; // funny

pub enum Argument {
    Register(i16),
    MemAddr(i16),
    Literal(i16),
    RegPtr(i16),
    MemPtr(i16),
    SR(i16),
    Flag(i16),
    Nothing,
}

pub enum Instruction {
    HLT,
    ADD(Argument, Argument),
    JO(Argument),
    JNO(Argument),
    POP(Argument),
    DIV(Argument, Argument),
    RET,
    LD(Argument, Argument),
    ST(Argument, Argument),
    JMP(Argument),
    JR(Argument),
    JZ(Argument),
    JNZ(Argument),
    JL(Argument),
    JG(Argument),
    CMP(Argument, Argument),
    MUL(Argument, Argument),
    PUSH(Argument),
    INT(Argument),
    MOV(Argument, Argument),
    LEA(Argument, Argument),
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Register(val) => write!(f, "r{val}"),
            Argument::MemAddr(val) => write!(f, "[{val}]"),
            Argument::Literal(val) => write!(f, "{val}"),
            Argument::RegPtr(val) => write!(f, "&r{val}"),
            Argument::MemPtr(val) => write!(f, "&{val}"),
            Argument::SR(val) => write!(f, "SR({val})"),
            Argument::Flag(val) => write!(f, "Flag({val})"),
            Argument::Nothing => write!(f, "Nothing"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::HLT => write!(f, "HLT"),
            Instruction::ADD(arg1, arg2) => write!(f, "ADD {arg1}, {arg2}"),
            Instruction::JO(arg) => write!(f, "JO {arg}"),
            Instruction::POP(arg) => write!(f, "POP {arg}"),
            Instruction::DIV(arg1, arg2) => write!(f, "DIV {arg1}, {arg2}"),
            Instruction::RET => write!(f, "RET"),
            Instruction::LD(arg1, arg2) => write!(f, "LD {arg1}, {arg2}"),
            Instruction::ST(arg1, arg2) => write!(f, "ST {arg1}, {arg2}"),
            Instruction::JMP(arg) => write!(f, "JMP {arg}"),
            Instruction::JR(arg) => write!(f, "JR {arg}"),
            Instruction::JNZ(arg) => write!(f, "JNZ {arg}"),
            Instruction::JL(arg) => write!(f, "JL {arg}"),
            Instruction::JG(arg) => write!(f, "JG {arg}"),
            Instruction::JNO(arg) => write!(f, "JNO {arg}"),
            Instruction::JZ(arg) => write!(f, "JZ {arg}"),
            Instruction::CMP(arg1, arg2) => write!(f, "CMP {arg1}, {arg2}"),
            Instruction::MUL(arg1, arg2) => write!(f, "MUL {arg1}, {arg2}"),
            Instruction::PUSH(arg) => write!(f, "PUSH {arg}"),
            Instruction::INT(arg) => write!(f, "INT {arg}"),
            Instruction::MOV(arg1, arg2) => write!(f, "MOV {arg1}, {arg2}"),
            Instruction::LEA(arg1, arg2) => write!(f, "LEA {arg1}, {arg2}"),
        }
    }
}

impl Argument {
    pub fn is_ptr(&self) -> bool {
        matches!(self, Argument::RegPtr(_) | Argument::MemPtr(_))
    }
}

impl CPU {
    pub fn get_value(&mut self, arg: &Argument) -> Result<f32, UnrecoverableError> {
        match arg {
            Register(n) => match n {
                4 => Ok(self.uint_reg[0] as f32),
                5 => Ok(self.uint_reg[1] as f32),
                6 => Ok(self.float_reg[0]),
                7 => Ok(self.float_reg[1]),
                n if *n > 3 => {
                    self.err = true;
                    Err(UnrecoverableError::IllegalInstruction(
                        self.ir,
                        self.pc,
                        Some("Illegal register".to_string()),
                    ))
                }
                n if *n < 0 => {
                    self.err = true;
                    Err(UnrecoverableError::IllegalInstruction(
                        self.ir,
                        self.pc,
                        Some("Illegal register".to_string()),
                    ))
                }

                _ => Ok(self.int_reg[*n as usize] as f32),
            },
            Literal(n) => Ok((*n) as f32),
            MemPtr(n) => {
                if self.memory[*n as usize].is_none() {
                    return Err(self.generate_segfault(
                        "Segmentation fault while dereferencing pointer.\nThe pointer's location is empty.",
                    ));
                }
                let tmp = self.memory[*n as usize].unwrap() as usize;
                if tmp > MEMORY_SIZE {
                    self.err = true;
                    return Err(UnrecoverableError::IllegalInstruction(
            self.ir,
                        self.pc,
                        Some("Segmentation fault whilst processing pointer.\nMemory address invalid (too large).".to_string()),
                    ));
                }
                if self.memory[tmp].is_none() {
                    return Err(self.generate_segfault(
                        "Segmentation fault while dereferencing pointer.\nThe address the pointer references is empty.",
                    ));
                }
                Ok(self.memory[tmp].unwrap() as f32)
            }
            RegPtr(n) => {
                let tmp = match n {
                    4 => self.uint_reg[0] as f32,
                    5 => self.uint_reg[1] as f32,
                    6 => self.float_reg[0],
                    7 => self.float_reg[1],
                    n if *n > 3 => {
                        self.err = true;
                        self.running = false;
                        return Err(UnrecoverableError::IllegalInstruction(
                            self.ir,
                            self.pc,
                            Some("Illegal register pointer".to_string()),
                        ));
                    }
                    n if *n < 0 => {
                        self.err = true;
                        self.running = false;
                        return Err(UnrecoverableError::IllegalInstruction(
                            self.ir,
                            self.pc,
                            Some("Illegal register pointer".to_string()),
                        ));
                    }
                    _ => self.int_reg[*n as usize] as f32,
                };
                let memloc = tmp as usize;
                if memloc >= self.memory.len() || tmp < 0.0 {
                    self.running = false;
                    return Err(self
                        .generate_segfault("Segmentation fault handling pointer.\nAddress OOB."));
                }
                if self.memory[memloc].is_none() {
                    self.running = false;
                    return Err(self.generate_segfault(
                        "Segmentation fault while dereferencing pointer.\nThe address the pointer references is empty.",
                    ));
                }
                Ok(self.memory[memloc].unwrap() as f32)
            }
            MemAddr(n) => {
                if self.memory[*n as usize].is_none() {
                    self.running = false;
                    return Err(self.generate_segfault(
                        "Segmentation fault while loading from memory.\nMemory address is empty.",
                    ));
                }
                Ok(self.memory[*n as usize].unwrap() as f32)
            }
            _ => unreachable!("Argument types are invalid (how did you get here?)"),
        }
    }

    pub fn decode_instruction(&self) -> Instruction {
        let opcode = (self.ir >> 12) & 0b1111u16 as i16;
        let mut ins_type = if ((self.ir >> 8) & 1) == 1 {
            1
        } else if ((self.ir >> 7) & 1) == 1 {
            2
        } else if ((self.ir >> 6) & 1) == 1 {
            3
        } else {
            0
        };
        let it_is_bouncy =
            opcode == JZ_OP || opcode == JO_OP || opcode == JMP_OP || opcode == RET_OP;
        let indirect_bounce = (self.ir & 0b0000_0100_0000_0000) >> 10 == 1;
        let tmp = self.ir & 0b111_1111;

        let source = match ins_type {
            1 => {
                if it_is_bouncy {
                    if indirect_bounce {
                        ins_type = 4;
                        self.ir & 0b111
                    } else {
                        self.ir & 0b11_1111_1111
                    }
                } else if (self.ir & 0b10000000) >> 7 == 1 {
                    -tmp
                } else {
                    tmp
                }
            }
            _ => {
                if it_is_bouncy {
                    if indirect_bounce {
                        ins_type = 4;
                        self.ir & 0b111
                    } else {
                        self.ir & 0b11_1111_1111
                    }
                } else {
                    self.ir & 0b111_1111
                }
            }
        };
        let destination = (self.ir & 0b1110_0000_0000) >> 9;
        let mut part = match ins_type {
            0 => Register(source),
            1 => Literal(source),
            2 => MemPtr(source),
            _ => RegPtr(source),
        };

        if let RegPtr(value) = part {
            part = RegPtr(value & 0b111);
        }

        if let MemPtr(value) = part {
            part = MemPtr(value & 0b111_1111);
        }
        let invert = ((self.ir & 0b0000_1000_0000_0000) >> 11) == 1;
        // println!("{:04b}", opcode);
        match opcode {
            HLT_OP => HLT,
            ADD_OP => ADD(Register(destination), part),
            JO_OP => {
                let dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };
                if invert {
                    JNO(dest)
                } else {
                    JO(dest)
                }
            }
            POP_OP => {
                if self.ir & 2048 == 0 {
                    POP(Register(source))
                } else {
                    POP(MemAddr(self.ir & 2047))
                }
            }
            DIV_OP => DIV(Register(destination), part),
            RET_OP => {
                let dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };

                if self.ir & 4095 == 0 {
                    RET
                } else if invert {
                    JG(dest)
                } else {
                    JL(dest)
                }
            }
            LD_OP => {
                let part = self.ir & 0b0001_1111_1111;
                LD(Register(destination), MemAddr(part))
            }
            ST_OP => {
                if (self.ir & 0b1000_0000_0000) >> 11 == 1 {
                    let part = (self.ir & 0b1110000000) >> 7;
                    ST(RegPtr(part), Register(self.ir & 0b111))
                } else {
                    let part = (self.ir & 0b111111111000) >> 3;
                    ST(MemAddr(part), Register(self.ir & 0b111))
                }
            }
            JMP_OP => {
                let dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };
                if invert {
                    JR(dest)
                } else {
                    JMP(dest)
                }
            }
            JZ_OP => {
                let dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };
                if invert {
                    JNZ(dest)
                } else {
                    JZ(dest)
                }
            }
            CMP_OP => CMP(Register(destination), part),
            MUL_OP => MUL(Register(destination), part),
            PUSH_OP => PUSH(part),
            INT_OP => INT(Literal(source)),
            MOV_OP => MOV(Register(destination), part),
            LEA_OP => {
                let part = self.ir & 0b111111111;
                LEA(Register(destination), MemAddr(part))
            }
            _ => {
                eprintln!(
                    "Cannot parse this. Code should be unreachable. {} line {}",
                    file!(),
                    line!()
                );
                MOV(Register(0), Register(0))
            }
        }
    }
}
