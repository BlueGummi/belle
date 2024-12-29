use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
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
        let it_is_bouncy = opcode == JZ_OP || opcode == JO_OP || opcode == JMP_OP;
        let indirect_bounce = (self.ir & 0b100000000000) >> 11 == 1;
        let tmp = self.ir & 0b1111111;

        let source = match ins_type {
            1 => {
                if it_is_bouncy {
                    if indirect_bounce {
                        ins_type = 4;
                        self.ir & 0b1111
                    } else {
                        self.ir & 0b111111111111
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
                        self.ir & 0b1111
                    } else {
                        self.ir & 0b111111111111
                    }
                } else {
                    self.ir & 0b1111111
                }
            }
        };
        let destination = (self.ir & 0b111000000000) >> 9;
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
            part = MemPtr(value & 0b1111111);
        }

        // println!("{:04b}", opcode);
        match opcode {
            HLT_OP => HLT,
            ADD_OP => ADD(Register(destination), part),
            JO_OP => {
                if ins_type == 4 {
                    JO(RegPtr(source))
                } else {
                    JO(MemAddr(source))
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
            RET_OP => RET,
            LD_OP => {
                let part = self.ir & 0b111111111;
                LD(Register(destination), MemAddr(part))
            }
            ST_OP => {
                if (self.ir & 0b100000000000) >> 11 == 1 {
                    let part = (self.ir & 0b1110000000) >> 7;
                    ST(RegPtr(part), Register(self.ir & 0b111))
                } else {
                    let part = (self.ir & 0b111111111000) >> 3;
                    ST(MemAddr(part), Register(self.ir & 0b111))
                }
            }
            JMP_OP => {
                if ins_type == 4 {
                    JMP(RegPtr(source))
                } else {
                    JMP(MemAddr(source))
                }
            }
            JZ_OP => {
                if ins_type == 4 {
                    JZ(RegPtr(source))
                } else {
                    JZ(MemAddr(source))
                }
            }
            CMP_OP => CMP(Register(destination), part),
            MUL_OP => MUL(Register(destination), part),
            PUSH_OP => PUSH(part),
            INT_OP => INT(Literal(source)),
            MOV_OP => MOV(Register(destination), part),
            NOP_OP => NOP,
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
