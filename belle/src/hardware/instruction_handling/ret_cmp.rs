use crate::{Argument::*, *};
impl CPU {
    pub fn handle_ret(&mut self) -> PossibleCrash {
        let temp: i32 = self.sp as i32;
        if let Some(v) = self.memory[temp as usize] {
            self.pc = v + 1;
            if self.sp > self.bp {
                self.memory[self.sp as usize] = None;
                if self.sp != self.bp {
                    self.sp -= 1;
                }
            } else {
                self.memory[self.sp as usize] = None;
                if self.sp != self.bp {
                    self.sp += 1;
                }
            }
        } else {
            return Err(UnrecoverableError::StackUnderflow(self.ir, self.pc, None));
        }
        Ok(())
    }

    pub fn handle_cmp(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let src = self.get_value(arg2)?;
        if let Register(_) = arg1 {
            let value = self.get_value(arg1)?;
            let result = value - src; // arg1 - arg2
            self.zflag = (result).abs() < f32::MIN_POSITIVE; // set if equal
            self.sflag = result < 0.0; // set if arg1 < arg2, JL if this is set
        }
        self.pc += 1;
        Ok(())
    }
}
