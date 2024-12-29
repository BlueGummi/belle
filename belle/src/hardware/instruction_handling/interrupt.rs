use crate::{config::CONFIG, *};
use std::io::{self, Read, Write};

impl CPU {
    pub fn handle_int(&mut self, arg: &Argument) -> PossibleCrash {
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
                if end_point < 0
                    || end_point as usize >= memory.len()
                    || starting_point < 0
                    || starting_point as usize >= memory.len()
                {
                    return Err(self.generate_segfault(
                        "Segmentation fault. Memory index out of bounds on interrupt call 8.",
                    ));
                }

                for index in starting_point..end_point {
                    if index < 0 || index as usize >= memory.len() {
                        return Err(self.generate_segfault(
                            "Segmentation fault. Memory index out of bounds on interrupt call 8.",
                        ));
                    }

                    if let Some(value) = memory[index as usize] {
                        print!("{}", value as u8 as char);
                    }
                }
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
        self.pc += 1;
        Ok(())
    }
}
