use crate::{Argument::*, Instruction::*, *};
use std::fmt;

pub const HLT_OP: i16 = 0b0000; // we need this
pub const ADD_OP: i16 = 0b0001; // we also need this
pub const BO_OP: i16 = 0b0010; // maybe optional ?
pub const POP_OP: i16 = 0b0011; // we need this
pub const DIV_OP: i16 = 0b0100; // we need this
pub const RET_OP: i16 = 0b0101; // we need this
pub const LD_OP: i16 = 0b0110; // we need this
pub const ST_OP: i16 = 0b0111; // we need this
pub const JMP_OP: i16 = 0b1000; // maybe optional?
pub const BZ_OP: i16 = 0b1001; // we need this
pub const CMP_OP: i16 = 0b1010; // we need this
pub const NAND_OP: i16 = 0b1011; // we need this
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
}

pub enum Instruction {
    HLT,
    ADD(Argument, Argument),
    BO(Argument),
    BNO(Argument),
    POP(Argument),
    DIV(Argument, Argument),
    RET,
    LD(Argument, Argument),
    ST(Argument, Argument),
    JMP(Argument),
    BZ(Argument),
    BNZ(Argument),
    BL(Argument),
    BG(Argument),
    CMP(Argument, Argument),
    NAND(Argument, Argument),
    PUSH(Argument),
    INT(Argument),
    MOV(Argument, Argument),
    LEA(Argument, Argument),
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Register(val) => write!(f, "r{val}"),
            Argument::MemAddr(val) => write!(f, "[x{val:X}]"),
            Argument::Literal(val) => write!(f, "{val}"),
            Argument::RegPtr(val) => write!(f, "&r{val}"),
            Argument::MemPtr(val) => write!(f, "&x{val:X}"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::HLT => write!(f, "HLT"),
            Instruction::ADD(arg1, arg2) => write!(f, "ADD {arg1}, {arg2}"),
            Instruction::BO(arg) => write!(f, "BO {arg}"),
            Instruction::POP(arg) => write!(f, "POP {arg}"),
            Instruction::DIV(arg1, arg2) => write!(f, "DIV {arg1}, {arg2}"),
            Instruction::RET => write!(f, "RET"),
            Instruction::LD(arg1, arg2) => write!(f, "LD {arg1}, {arg2}"),
            Instruction::ST(arg1, arg2) => write!(f, "ST {arg1}, {arg2}"),
            Instruction::JMP(arg) => write!(f, "JMP {arg}"),
            Instruction::BNZ(arg) => write!(f, "BNZ {arg}"),
            Instruction::BL(arg) => write!(f, "BL {arg}"),
            Instruction::BG(arg) => write!(f, "BG {arg}"),
            Instruction::BNO(arg) => write!(f, "BNO {arg}"),
            Instruction::BZ(arg) => write!(f, "BZ {arg}"),
            Instruction::CMP(arg1, arg2) => write!(f, "CMP {arg1}, {arg2}"),
            Instruction::NAND(arg1, arg2) => write!(f, "NAND {arg1}, {arg2}"),
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
    #[inline(always)]
    pub fn get_value(&mut self, arg: &Argument) -> Result<f32, UnrecoverableError> {
        match arg {
            Register(n) => match n {
                6 => Ok(self.float_reg[0]),
                7 => Ok(self.float_reg[1]),
                8 => Ok(self.pc as f32),
                9 => Ok(self.sp as f32),
                n if *n > 5 || *n < 0 => Err(self.generate_invalid_register()),
                _ => Ok(self.int_reg[*n as usize] as f32),
            },
            Literal(n) => Ok((*n) as f32),
            MemPtr(n) => {
                let tmp = self.memory[*n as usize] as usize;
                if tmp > MEMORY_SIZE {
                    self.err = true;
                    return Err(UnrecoverableError::IllegalInstruction(
                        self.ir,
                        self.pc,
                        Some("Segmentation fault whilst processing pointer.\nMemory address invalid (too large).".to_string()),
                    ));
                }
                Ok(self.memory[tmp] as f32)
            }
            RegPtr(n) => {
                let tmp = match n {
                    6 => self.float_reg[0],
                    7 => self.float_reg[1],
                    8 => self.pc as f32,
                    9 => self.sp as f32,
                    n if *n > 5 || *n < 0 => {
                        return Err(self.generate_invalid_register());
                    }
                    _ => self.int_reg[*n as usize] as f32,
                };
                let memloc = tmp as usize;
                if memloc >= self.memory.len() {
                    self.err = true;
                    self.running = false;
                    return Err(self
                        .generate_segfault("Segmentation fault handling pointer.\nAddress OOB."));
                }
                Ok(self.memory[memloc] as f32)
            }
            MemAddr(n) => Ok(self.memory[*n as usize] as f32),
        }
    }

    pub fn decode_instruction(&self) -> Instruction {
        let ir = self.ir;
        let opcode = (ir >> 12) & 0b1111;
        let mut ins_type = if ((ir >> 8) & 1) == 1 {
            1
        } else if ((ir >> 7) & 1) == 1 {
            2
        } else if ((ir >> 6) & 1) == 1 {
            3
        } else {
            0
        };
        let it_is_bouncy =
            opcode == BZ_OP || opcode == BO_OP || opcode == JMP_OP || opcode == RET_OP;

        let source = match ins_type {
            1 => {
                if it_is_bouncy {
                    if (ir & 0b0000_0100_0000_0000) >> 10 == 1 {
                        ins_type = 4;
                        ir & 0b1111
                    } else {
                        ir & 0b11_1111_1111
                    }
                } else if (ir & 0b10000000) >> 7 == 1 {
                    -(ir & 0b111_1111)
                } else {
                    ir & 0b111_1111
                }
            }
            _ => {
                if it_is_bouncy {
                    if (ir & 0b0000_0100_0000_0000) >> 10 == 1 {
                        ins_type = 4;
                        ir & 0b1111
                    } else {
                        ir & 0b11_1111_1111
                    }
                } else {
                    ir & 0b111_1111
                }
            }
        };
        let destination = (ir & 0b1110_0000_0000) >> 9;
        let part = match ins_type {
            0 => Register(source & 0b1111),
            1 => Literal(source),
            2 => MemPtr(source & 0b111_1111),
            _ => RegPtr(source & 0b1111),
        };

        let invert = ((ir & 0b1000_0000_0000) >> 11) == 1;

        match opcode {
            HLT_OP => HLT,
            ADD_OP => ADD(Register(destination), part),
            BO_OP => {
                let j_dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };
                if invert {
                    BNO(j_dest)
                } else {
                    BO(j_dest)
                }
            }
            POP_OP => {
                if ir & 2048 == 0 {
                    POP(Register(source))
                } else {
                    POP(MemAddr(ir & 2047))
                }
            }
            DIV_OP => DIV(Register(destination), part),
            RET_OP => {
                let j_dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };
                if ir & 4095 == 0 {
                    RET
                } else if invert {
                    BG(j_dest)
                } else {
                    BL(j_dest)
                }
            }
            LD_OP => {
                let part = ir & 0b0001_1111_1111;
                LD(Register(destination), MemAddr(part))
            }
            ST_OP => {
                if (ir & 0b1000_0000_0000) >> 11 == 1 {
                    let part = (ir & 0b1110000000) >> 7;
                    ST(RegPtr(part), Register(ir & 0b111))
                } else {
                    let part = (ir & 0b111111111000) >> 3;
                    ST(MemAddr(part), Register(ir & 0b111))
                }
            }
            JMP_OP => {
                let j_dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };
                JMP(j_dest)
            }
            BZ_OP => {
                let j_dest = if ins_type == 4 {
                    RegPtr(source)
                } else {
                    MemAddr(source)
                };
                if invert {
                    BNZ(j_dest)
                } else {
                    BZ(j_dest)
                }
            }
            CMP_OP => CMP(Register(destination), part),
            NAND_OP => NAND(Register(destination), part),
            PUSH_OP => PUSH(part),
            INT_OP => INT(Literal(source)),
            MOV_OP => MOV(Register(destination), part),
            LEA_OP => {
                let part = ir & 0b111111111;
                LEA(Register(destination), MemAddr(part))
            }
            _ => unsafe {
                std::hint::unreachable_unchecked();
            },
        }
    }
}
