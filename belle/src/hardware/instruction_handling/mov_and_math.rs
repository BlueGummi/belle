use crate::{Argument::*, *};
impl CPU {
    pub fn handle_add(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let value = self.get_value(arg2)?;

        if let Register(n) = arg1 {
            let new_value = match *n {
                6 => self.float_reg[0] + value,
                7 => self.float_reg[1] + value,
                n if !(0..=5).contains(&n) => {
                    return Err(self.generate_invalid_register());
                }
                _ => {
                    let v = if let Argument::Literal(_) = arg2 {
                        value as i16 as u16
                    } else {
                        value as u16
                    };
                    self.oflag = self.int_reg[*n as usize]
                        .checked_add(value as u16)
                        .is_none();
                    self.int_reg[*n as usize].wrapping_add(v) as f32
                }
            };
            self.set_register_value(arg1, new_value as f64)?;
        }
        self.pc += 1;
        Ok(())
    }
    pub fn handle_div(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let divisor = match self.get_value(arg2) {
            Ok(v) => {
                if v == 0.0 || v as u16 == 0 {
                    self.generate_divbyz();
                    return Err(UnrecoverableError::DivideByZero(self.ir, self.pc, None));
                }
                v
            }
            Err(e) => return Err(e),
        };

        if let Register(n) = arg1 {
            let new_value = match *n {
                6 => {
                    self.rflag = self.float_reg[0] % divisor != 0.0;
                    self.float_reg[0] / divisor
                }
                7 => {
                    self.rflag = self.float_reg[1] % divisor != 0.0;
                    self.float_reg[1] / divisor
                }
                n if !(0..=5).contains(&n) => {
                    return Err(self.generate_invalid_register());
                }
                _ => {
                    self.rflag = f32::from(self.int_reg[*n as usize]) % divisor != 0.0;

                    let result = self.int_reg[*n as usize] / divisor as u16;
                    result as f32
                }
            };
            self.set_register_value(arg1, new_value as f64)?;
        }
        self.pc += 1;
        Ok(())
    }
    pub fn handle_nand(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let value = self.get_value(arg2)?;

        if let Register(n) = arg1 {
            let new_value = match *n {
                6 => !(self.float_reg[0].to_bits() & value.to_bits()) as f32,
                7 => !(self.float_reg[1].to_bits() & value.to_bits()) as f32,
                n if !(0..=5).contains(&n) => {
                    return Err(self.generate_invalid_register());
                }
                _ => !(self.int_reg[*n as usize] & (value as u16)) as f32,
            };
            self.set_register_value(arg1, new_value as f64)?
        }
        self.pc += 1;
        Ok(())
    }

    pub fn handle_mov(&mut self, arg1: &Argument, arg2: &Argument) -> PossibleCrash {
        let value = self.get_value(arg2)?;
        if let Register(_) = arg1 {
            let value = if arg2.is_ptr() {
                value as u16 as i16 as f32
            } else {
                value
            };
            self.set_register_value(arg1, value as f64)?
        }
        self.pc += 1;
        Ok(())
    }
}
