use crate::{Argument::*, *};
impl CPU {
    pub fn handle_push(&mut self, arg: &Argument) -> PossibleCrash {
        let mut val: f32 = 0.0;
        if let Literal(l) = arg {
            val = (*l).into();
        }

        if let Register(_) = arg {
            val = self.get_value(arg)?;
        }
        if self.sp > self.bp || self.backward_stack {
            if self.sp != self.bp {
                println!("{}", RecoverableError::BackwardStack(self.pc, None));
            }
            if self.sp != self.bp || self.memory[self.bp as usize].is_some() {
                self.sp += 1;
            }
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
            if self.sp >= self.bp {
                self.backward_stack = true;
            }
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
            if self.sp != self.bp || self.memory[self.bp as usize].is_some() {
                self.sp -= 1;
            }
            self.memory[self.sp as usize] = Some(val as u16);
        }
        self.pc += 1;
        Ok(())
    }

    pub fn handle_pop(&mut self, arg: &Argument) -> PossibleCrash {
        let temp: i32 = self.sp.into();
        if let Some(v) = self.memory[temp as usize] {
            if let Register(_) = arg {
                self.set_register_value(arg, v.into())?;
            } else if let MemAddr(val) = arg {
                self.memory[*val as usize] = Some(v);
            }
            if self.sp > self.bp {
                if self.sp != self.bp {
                    println!("{}", RecoverableError::BackwardStack(self.pc, None));
                }
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
            return Err(UnrecoverableError::SegmentationFault(
                self.ir,
                self.pc,
                Some("segmentation fault while executing pop".to_string()),
            ));
        }
        self.pc += 1;
        Ok(())
    }
}
