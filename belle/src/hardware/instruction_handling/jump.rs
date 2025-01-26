use crate::{Argument::*, *};
impl CPU {
    // all conditions are inverted because we do early returns here

    pub fn handle_bo(&mut self, arg: &Argument) -> PossibleCrash {
        if !self.oflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_bno(&mut self, arg: &Argument) -> PossibleCrash {
        if self.oflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_bg(&mut self, arg: &Argument) -> PossibleCrash {
        if self.sflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_bl(&mut self, arg: &Argument) -> PossibleCrash {
        if !self.sflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_jmp(&mut self, arg: &Argument) -> PossibleCrash {
        self.jmp(arg)?;
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
            if self.get_value(&Argument::Register(*n))? < 0.0
                || self.get_value(&Argument::Register(*n))? > MEMORY_SIZE as f32
            {
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

    pub fn handle_bz(&mut self, arg: &Argument) -> PossibleCrash {
        if !self.zflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_bnz(&mut self, arg: &Argument) -> PossibleCrash {
        if self.zflag {
            self.pc += 1;
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    fn jmp(&mut self, arg: &Argument) -> PossibleCrash {
        if self.pushret {
            self.handle_push(&Argument::Literal(self.pc.try_into().unwrap()))?;
        }
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
            if self.get_value(&Argument::Register(*n))? < 0.0
                || self.get_value(&Argument::Register(*n))? > MEMORY_SIZE as f32
            {
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
