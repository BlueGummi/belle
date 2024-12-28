use crate::Argument::*;
use crate::*;
impl CPU {
    pub fn handle_jo(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        if !self.oflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_jmp(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_jz(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        if !self.zflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    fn jmp(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        self.handle_push(&Argument::Literal(self.pc.try_into().unwrap()))?;
        if let MemAddr(n) = arg {
            if *n < 0 {
                return Err(UnrecoverableError::SegmentationFault(
                    self.ir,
                    self.pc,
                    Some("attempted to jump to an invalid address".to_string()),
                ));
            }
            self.pc = *n as u16;
        } else if let RegPtr(n) = arg {
            if self.get_value(&Argument::Register(*n))? < 0.0 {
                return Err(UnrecoverableError::SegmentationFault(
                    self.ir,
                    self.pc,
                    Some("attempted to jump to an invalid address".to_string()),
                ));
            }
            self.pc = self.get_value(&Argument::Register(*n))? as u16;
        }
        Ok(())
    }
}