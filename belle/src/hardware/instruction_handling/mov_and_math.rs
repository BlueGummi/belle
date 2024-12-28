use crate::Argument::*;
use crate::*;
impl CPU {
    pub fn handle_add(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
        let value = self.get_value(arg2)?;

        if let Register(n) = arg1 {
            let new_value = match *n {
                4 => {
                    self.uint_reg[0] = (self.uint_reg[0] as i16).wrapping_add(value as i16) as u16;
                    self.uint_reg[0] as i64 + value as i64
                }
                5 => {
                    self.uint_reg[1] = (self.uint_reg[1] as i16).wrapping_add(value as i16) as u16;
                    self.uint_reg[1] as i64 + value as i64
                }
                6 => {
                    self.float_reg[0] += value;
                    (self.float_reg[0] as i64).wrapping_add(value as i64)
                }
                7 => {
                    self.float_reg[1] += value;
                    (self.float_reg[1] as i64).wrapping_add(value as i64)
                }
                n if n > 5 => {
                    self.generate_invalid_register();
                    0
                }
                _ => {
                    self.int_reg[*n as usize] = if arg2.is_ptr() {
                        self.int_reg[*n as usize].wrapping_add(value as u16 as i16)
                    } else {
                        self.int_reg[*n as usize].wrapping_add(value as i16)
                    };
                    self.int_reg[*n as usize] as i64 + value as i64
                }
            };

            if let Err(e) = self.check_overflow(new_value, *n as u16) {
                eprint!("{e}");
            }
        }
        self.pc += 1;
        Ok(())
    }
    pub fn handle_div(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
        let divisor = match self.get_value(arg2) {
            Ok(v) => {
                if v == 0.0 || v as i32 == 0 {
                    self.generate_divbyz();
                    return Err(UnrecoverableError::DivideByZero(self.ir, self.pc, None));
                }
                v
            }
            Err(e) => return Err(e),
        };

        if let Register(n) = arg1 {
            let new_value = match *n {
                4 => {
                    if self.uint_reg[0] as f32 % divisor != 0.0 {
                        self.rflag = true;
                    }
                    let result = self.uint_reg[0] as i32 / divisor as i32;
                    self.uint_reg[0] = result as u16;
                    result as i64
                }
                5 => {
                    if self.uint_reg[1] as f32 % divisor != 0.0 {
                        self.rflag = true;
                    }
                    let result = self.uint_reg[1] as i32 / divisor as i32;
                    self.uint_reg[1] = result as u16;
                    result as i64
                }
                6 => {
                    if self.float_reg[0] % divisor != 0.0 {
                        self.rflag = true;
                    }
                    let result = self.float_reg[0] / divisor;
                    self.float_reg[0] = result;
                    result as i64
                }
                7 => {
                    if self.float_reg[1] % divisor != 0.0 {
                        self.rflag = true;
                    }
                    let result = self.float_reg[1] / divisor;
                    self.float_reg[1] = result;
                    result as i64
                }
                n if n > 5 => {
                    return Err(self.generate_invalid_register());
                }
                _ => {
                    if f32::from(self.int_reg[*n as usize]) % divisor != 0.0 {
                        self.rflag = true;
                    }
                    let result = if arg2.is_ptr() {
                        self.int_reg[*n as usize] / divisor as u16 as i16
                    } else {
                        self.int_reg[*n as usize] / divisor as i16
                    };
                    self.int_reg[*n as usize] = result;
                    result as i64
                }
            };

            if let Err(e) = self.check_overflow(new_value, *n as u16) {
                eprint!("{e}");
            }
        }
        self.pc += 1;
        Ok(())
    }
    pub fn handle_mul(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
        let value = self.get_value(arg2)?;

        if let Register(n) = arg1 {
            let new_value = match *n {
                4 => {
                    self.uint_reg[0] = self.uint_reg[0].wrapping_mul(value as u16);
                    self.uint_reg[0] as i64 * value as i64
                }
                5 => {
                    self.uint_reg[1] = self.uint_reg[1].wrapping_mul(value as u16);
                    self.uint_reg[1] as i64 * value as i64
                }
                6 => {
                    let temp = self.float_reg[0] * value;
                    self.float_reg[0] = temp;
                    temp as i64
                }
                7 => {
                    let temp = self.float_reg[1] * value;
                    self.float_reg[1] = temp;
                    temp as i64
                }
                n if n > 5 => {
                    self.generate_invalid_register();
                    return Err(UnrecoverableError::InvalidRegister(self.ir, self.pc, None));
                }
                _ => {
                    let result = if arg2.is_ptr() {
                        self.int_reg[*n as usize].wrapping_mul(value as u16 as i16)
                    } else {
                        self.int_reg[*n as usize].wrapping_mul(value as i16)
                    };
                    self.int_reg[*n as usize] = result;
                    (self.int_reg[*n as usize] as i64).wrapping_mul(value as i64)
                }
            };

            if let Err(e) = self.check_overflow(new_value, *n as u16) {
                eprint!("{e}");
            }
        }
        self.pc += 1;
        Ok(())
    }

    pub fn handle_mov(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
        let value = self.get_value(arg2)?;
        if let Register(n) = arg1 {
            match *n {
                4 => self.uint_reg[0] = value as u16,
                5 => self.uint_reg[1] = value as u16,
                6 => self.float_reg[0] = value,
                7 => self.float_reg[1] = value,
                n if n > 5 => return Err(self.generate_invalid_register()),
                _ => {
                    self.int_reg[*n as usize] = if arg2.is_ptr() {
                        value as u16 as i16
                    } else {
                        value as i16
                    };
                }
            }
        }
        self.pc += 1;
        Ok(())
    }
}