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
            if let Register(n) = arg {
                match *n {
                    4 => self.uint_reg[0] = v,
                    5 => self.uint_reg[1] = v,
                    6 => self.float_reg[0] = v as f32,
                    7 => self.float_reg[1] = v as f32,
                    n if n > 3 => return Err(self.generate_invalid_register()),
                    _ => self.int_reg[*n as usize] = v as i16,
                }
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
