use crate::{Argument::*, *};
impl CPU {
    pub fn handle_push(&mut self, arg: &Argument) -> PossibleCrash {
        let val = self.get_value(arg)?;

        if self.sp > self.bp || self.backward_stack {
            self.sp += 1;
            if self.sp as usize >= MEMORY_SIZE {
                self.running = false;
                self.err = true;
                return Err(UnrecoverableError::StackOverflow(
                    self.ir,
                    self.pc,
                    Some("Overflowed while pushing onto stack".to_string()),
                ));
            }

            self.memory[self.sp as usize] = Some(val as u16);
            self.backward_stack = self.sp >= self.bp;
        } else {
            if self.sp == 0 {
                self.running = false;
                self.err = true;
                return Err(UnrecoverableError::StackOverflow(
                    self.ir,
                    self.pc,
                    Some("Overflowed while pushing onto stack".to_string()),
                ));
            }
            self.sp -= 1;
            self.memory[self.sp as usize] = Some(val as u16);
        }
        self.pc += 1;
        Ok(())
    }

    pub fn handle_pop(&mut self, arg: &Argument) -> PossibleCrash {
        let temp: i32 = self.sp.into();
        if let Some(v) = self.memory[temp as usize] {
            if let Register(_) = arg {
                self.set_register_value(arg, v as f32)?;
            } else if let MemAddr(val) = arg {
                self.memory[*val as usize] = Some(v);
            }
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
            self.err = true;
            self.running = false;
            return Err(UnrecoverableError::StackUnderflow(
                self.ir,
                self.pc,
                Some("segmentation fault while executing pop".to_string()),
            ));
        }
        self.pc += 1;
        Ok(())
    }
}
