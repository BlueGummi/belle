use crate::{Argument::*, *};
impl CPU {
    pub fn handle_st(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let source = self.get_value(arg2)? as i16;
        if let MemAddr(n) = arg1 {
            let index = *n as usize;
            if index >= self.memory.len() {
                return Err(UnrecoverableError::SegmentationFault(
                    self.ir,
                    self.pc,
                    Some("segmentation fault whilst storing to an address. OOB".to_string()),
                ));
            }
            self.memory[index] = Some(source as u16);
        } else if let RegPtr(n) = arg1 {
            let addr = match self.get_value(&Register(*n)) {
                Ok(a) => a as usize,
                Err(e) => return Err(e),
            };

            if addr >= self.memory.len() {
                return Err(UnrecoverableError::SegmentationFault(
                    self.ir, self.pc, None,
                ));
            }
            self.memory[addr] = Some(source as u16);
        }

        self.pc += 1;
        Ok(())
    }
    pub fn handle_lea(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let source = if let MemAddr(value) = arg2 {
            *value
        } else {
            return Err(UnrecoverableError::IllegalInstruction(
                self.ir,
                self.pc,
                Some("Illegal instruction reached on LEA".to_string()),
            ));
        };
        if let Register(n) = arg1 {
            match *n {
                4 => self.uint_reg[0] = source as u16,
                5 => self.uint_reg[1] = source as u16,
                6 => self.float_reg[0] = source as u16 as i16 as f32,
                7 => self.float_reg[1] = source as u16 as i16 as f32,
                n if n > 3 => return Err(self.generate_invalid_register()),
                _ => {
                    if let Err(e) = self.check_overflow(source as i64, *n as u16) {
                        eprint!("{e}");
                    }
                    self.int_reg[*n as usize] = source as u16 as i16;
                }
            }
            if let Err(e) = self.check_overflow(source as i64, *n as u16) {
                eprint!("{e}");
            }
        }
        self.pc += 1;
        Ok(())
    }
    pub fn handle_ld(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let source = self.get_value(arg2)?;
        if let Register(n) = arg1 {
            match *n {
                4 => self.uint_reg[0] = source as u16,
                5 => self.uint_reg[1] = source as u16,
                6 => self.float_reg[0] = source as u16 as i16 as f32,
                7 => self.float_reg[1] = source as u16 as i16 as f32,
                n if n > 3 => return Err(self.generate_invalid_register()),
                _ => {
                    if let Err(e) = self.check_overflow(source as i64, *n as u16) {
                        eprint!("{e}");
                    }
                    self.int_reg[*n as usize] = source as u16 as i16;
                }
            }
            if let Err(e) = self.check_overflow(source as i64, *n as u16) {
                eprint!("{e}");
            }
        }
        self.pc += 1;
        Ok(())
    }
}
