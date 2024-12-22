use crate::Argument::*;
use crate::*;
use std::io::{self, Read, Write};

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
                    let result = self.uint_reg[0].wrapping_add(value as u16);
                    self.uint_reg[0] = result;
                    result as i64 + value as i64
                }
                5 => {
                    let result = self.uint_reg[1].wrapping_add(value as u16);
                    self.uint_reg[1] = result;
                    result as i64 + value as i64
                }
                6 => {
                    let result = self.float_reg[0] + value;
                    self.float_reg[0] = result;
                    (result as i64).wrapping_add(value as i64)
                }
                7 => {
                    let result = self.float_reg[1] + value;
                    self.float_reg[1] = result;
                    (result as i64).wrapping_add(value as i64)
                }
                n if n > 5 => {
                    self.report_invalid_register();
                    0
                }
                _ => {
                    let result = self.int_reg[*n as usize].wrapping_add(value as i16);
                    self.int_reg[*n as usize] = result;
                    result as i64 + value as i64
                }
            };

            if let Err(e) = self.check_overflow(new_value, *n as u16) {
                eprint!("{e}");
            }
        }
        Ok(())
    }
    pub fn handle_jo(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        if !self.oflag {
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_pop(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        if let Register(_) = arg {
            let temp: i32 = self.sp.into();
            if let Some(v) = self.memory[temp as usize] {
                self.set_register_value(arg, v.into())?;
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
        }
        Ok(())
    }

    pub fn handle_div(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
        let divisor = match self.get_value(arg2) {
            Ok(v) => {
                if v == 0.0 || v as u16 == 0 {
                    self.report_divide_by_zero();
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
                    let result = self.uint_reg[0] / divisor as u16;
                    self.uint_reg[0] = result;
                    result as i64
                }
                5 => {
                    if self.uint_reg[1] as f32 % divisor != 0.0 {
                        self.rflag = true;
                    }
                    let result = self.uint_reg[1] / divisor as u16;
                    self.uint_reg[1] = result;
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
                    return Err(self.report_invalid_register());
                }
                _ => {
                    if f32::from(self.int_reg[*n as usize]) % divisor != 0.0 {
                        self.rflag = true;
                    }
                    let result = self.int_reg[*n as usize] / divisor as i16;
                    self.int_reg[*n as usize] = result;
                    result as i64
                }
            };

            if let Err(e) = self.check_overflow(new_value, *n as u16) {
                eprint!("{e}");
            }
        }
        Ok(())
    }
    pub fn handle_ret(&mut self) -> Result<(), UnrecoverableError> {
        let temp: i32 = self.sp.into();
        if let Some(v) = self.memory[temp as usize] {
            self.pc = v as u16;
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

    pub fn handle_ld(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
        let source = self.get_value(arg2)?;
        if let Register(n) = arg1 {
            match *n {
                4 => self.uint_reg[0] = source as u16,
                5 => self.uint_reg[1] = source as u16,
                6 => self.float_reg[0] = source,
                7 => self.float_reg[1] = source,
                n if n > 5 => return Err(self.report_invalid_register()),
                _ => {
                    if let Err(e) = self.check_overflow(source as i64, *n as u16) {
                        eprint!("{e}");
                        return Ok(());
                    }
                    self.int_reg[*n as usize] = source as i16;
                }
            }
        }
        Ok(())
    }

    pub fn handle_st(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
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
            self.memory[index] = Some(source);
        } else if let RegPtr(n) = arg1 {
            let addr = match self.get_value(&Register(*n)) {
                Ok(a) => a as usize,
                Err(e) => return Err(e),
            };

            if addr >= self.memory.len() {
                return Err(UnrecoverableError::IllegalInstruction(
                    self.ir, self.pc, None,
                ));
            }
            self.memory[addr] = Some(source);
        }

        Ok(())
    }
    pub fn handle_jmp(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        self.jmp(arg)?;
        Ok(())
    }

    pub fn handle_jz(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        if !self.zflag {
            return Ok(());
        }
        self.jmp(arg)?;
        Ok(())
    }

    fn jmp(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        self.handle_push(&Argument::Literal(self.pc.try_into().unwrap()))?;
        if let MemAddr(n) = arg {
            if { *n } - 1 < 0 {
                return Err(UnrecoverableError::IllegalInstruction(
                    self.ir,
                    self.pc,
                    Some("attempted to jump to an invalid address".to_string()),
                ));
            }
            self.pc = (*n as u16) - 1;
        } else if let RegPtr(n) = arg {
            if self.get_value(&Argument::Register(*n))? - 1.0 < 0.0 {
                return Err(UnrecoverableError::IllegalInstruction(
                    self.ir,
                    self.pc,
                    Some("attempted to jump to an invalid address".to_string()),
                ));
            }
            self.pc = (self.get_value(&Argument::Register(*n))? as u16) - 1;
        }
        Ok(())
    }

    pub fn handle_cmp(
        &mut self,
        arg1: &Argument,
        arg2: &Argument,
    ) -> Result<(), UnrecoverableError> {
        let src = self.get_value(arg2)?;
        if let Register(_) = arg1 {
            let value = self.get_value(arg1)?;
            let result = value - src;
            self.zflag = (result).abs() < f32::EPSILON;
            self.sflag = result < 0.0;
        }
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
                    self.report_invalid_register();
                    return Err(UnrecoverableError::InvalidRegister(self.ir, self.pc, None));
                }
                _ => {
                    self.int_reg[*n as usize] =
                        self.int_reg[*n as usize].wrapping_mul(value as i16);
                    (self.int_reg[*n as usize] as i64).wrapping_mul(value as i64)
                }
            };

            if let Err(e) = self.check_overflow(new_value, *n as u16) {
                eprint!("{e}");
            }
        }
        Ok(())
    }
    pub fn handle_push(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
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

            self.memory[self.sp as usize] = Some(val as i16);
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
            self.memory[self.sp as usize] = Some(val as i16);
        }
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
                n if n > 5 => return Err(self.report_invalid_register()),
                _ => {
                    self.int_reg[*n as usize] = value as i16;
                }
            }
        }
        Ok(())
    }

    pub fn handle_int(&mut self, arg: &Argument) -> Result<(), UnrecoverableError> {
        if CONFIG.fuzz {
            return Ok(());
        }
        let code = self.get_value(arg)? as u16;
        match code {
            0_u16..=3_u16 => {
                println!("{}", self.int_reg[code as usize]);
            }
            4 => println!("{}", self.uint_reg[0]),
            5 => println!("{}", self.uint_reg[1]),
            6 => println!("{}", self.float_reg[0]),
            7 => println!("{}", self.float_reg[1]),
            8 => {
                let starting_point = self.int_reg[0];
                let end_point = self.int_reg[1];
                let memory = &self.memory;
                let mut toprint = String::from("");
                if end_point < 0
                    || end_point as usize >= memory.len()
                    || starting_point < 0
                    || starting_point as usize >= memory.len()
                {
                    return Err(self.handle_segmentation_fault(
                        "Segmentation fault. Memory index out of bounds on interrupt call 8.",
                    ));
                }

                for index in starting_point..=end_point {
                    if index < 0 || index as usize >= memory.len() {
                        return Err(self.handle_segmentation_fault(
                            "Segmentation fault. Memory index out of bounds on interrupt call 8.",
                        ));
                    }

                    if let Some(value) = memory[index as usize] {
                        match u32::try_from(value) {
                            Ok(v) => {
                                if let Some(character) = char::from_u32(v) {
                                    toprint = format!("{}{}", toprint, character);
                                } else {
                                    return Err(self.handle_segmentation_fault(
                                        "Segmentation fault. Invalid Unicode value.",
                                    ));
                                }
                            }
                            Err(_) => {
                                return Err(self.handle_segmentation_fault(
                    "Segmentation fault. Conversion error from memory value to u32.",
                ));
                            }
                        }
                    }
                }
                print!("{toprint}");
            }
            9 => {
                use crossterm::terminal;
                terminal::enable_raw_mode().unwrap();
                let mut buffer = [0; 1];
                let _ = io::stdin().read_exact(&mut buffer);
                self.int_reg[0] = buffer[0] as i16;

                terminal::disable_raw_mode().unwrap();
                io::stdout().flush().expect("Failed to flush stdout");
            }
            10 => {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            11 => self.zflag = true,
            12 => self.zflag = false,
            13 => self.zflag = !self.zflag,
            20 => {
                self.max_clk = Some(self.int_reg[0] as usize);
            }
            21 => self.oflag = true,
            22 => self.oflag = false,
            23 => self.oflag = !self.oflag,
            30 => cls(),
            31 => self.rflag = true,
            32 => self.rflag = false,
            33 => self.rflag = !self.rflag,

            41 => self.sflag = true,
            42 => self.sflag = false,
            43 => self.sflag = !self.sflag,

            51 => self.hlt_on_overflow = true,
            52 => self.hlt_on_overflow = false,
            53 => self.hlt_on_overflow = !self.hlt_on_overflow,

            60 => self.sp = self.uint_reg[0],
            61 => self.bp = self.uint_reg[0],

            // 10 - 20 set flags
            // 20 - 30 unset them
            // 30 - 40 invert them
            _ => println!(
                "{}",
                RecoverableError::UnknownFlag(
                    self.pc,
                    Some(String::from("Occurred whilst handling INT")),
                )
            ),
        }
        Ok(())
    }
}
