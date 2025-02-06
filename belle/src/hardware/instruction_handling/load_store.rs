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
        if let Register(_) = arg1 {
            self.set_register_value(arg1, source as f64)?;
        }
        self.pc += 1;
        Ok(())
    }
    pub fn handle_ld(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let source = self.get_value(arg2)?;
        if let Register(_) = arg1 {
            self.set_register_value(arg1, source as f64)?;
        }
        self.pc += 1;
        Ok(())
    }
}
