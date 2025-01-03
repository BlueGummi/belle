use crate::{config::CONFIG, *};
use colored::*;
use std::io::{self, Read, Write};

impl CPU {
    pub fn handle_int(&mut self, arg: &Argument) -> PossibleCrash {
        if CONFIG.fuzz {
            return Ok(());
        }
        let code = self.get_value(arg)? as u16;
        match code {
            0_u16..=3_u16 => {
                if CONFIG.verbose {
                    println!("╭─────────╮");
                    println!("│ {:^5}   │", self.int_reg[code as usize]);
                    println!("╰─────────╯");
                } else {
                    println!("{}", self.int_reg[code as usize]);
                }
            }
            4 => {
                if CONFIG.verbose {
                    println!("╭─────────╮");
                    println!("│ {:^5}   │", self.uint_reg[0]);
                    println!("╰─────────╯");
                } else {
                    println!("{}", self.uint_reg[0]);
                }
            }
            5 => {
                if CONFIG.verbose {
                    println!("╭─────────╮");
                    println!("│ {:^5}   │", self.uint_reg[1]);
                    println!("╰─────────╯");
                } else {
                    println!("{}", self.uint_reg[1]);
                }
            }
            6 => {
                if CONFIG.verbose {
                    println!("╭─────────╮");
                    println!("│ {:^5.5} │", self.float_reg[0]);
                    println!("╰─────────╯");
                } else {
                    println!("{}", self.float_reg[0]);
                }
            }
            7 => {
                if CONFIG.verbose {
                    println!("╭─────────╮");
                    println!("│ {:^5.5} │", self.float_reg[1]);
                    println!("╰─────────╯");
                } else {
                    println!("{}", self.float_reg[1]);
                }
            }
            8 => {
                let starting_point = self.int_reg[0];
                let end_point = self.int_reg[1];
                let memory = &self.memory;
                let mut stringy = String::new();
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
                        if CONFIG.verbose {
                            stringy = format!("{}{}", stringy, value as u8 as char);
                        } else {
                            print!("{}", value as u8 as char);
                        }
                    }
                }
                if CONFIG.verbose {
                    let lines: Vec<&str> = stringy.lines().collect();
                    let max_length =
                        if lines.iter().map(|line| line.len()).max().unwrap_or(10) >= 10 {
                            lines.iter().map(|line| line.len()).max().unwrap_or(10)
                        } else {
                            12
                        };
                    if max_length >= 10 {
                        println!("╭{}╮", "─".repeat(max_length + 2));
                    } else {
                        println!("╭{}╮", "─".repeat(12));
                    }
                    if max_length >= 10 {
                        println!(
                            "│ {} {}│",
                            "CPU STDOUT".to_string().bold().cyan(),
                            " ".repeat(max_length - 10)
                        );
                    } else {
                        println!("│ {} │", "CPU STDOUT".to_string().bold().cyan());
                    }
                    if max_length >= 10 {
                        println!("├{}┤", "─".repeat(max_length + 2));
                    } else {
                        println!("├{}┤", "─".repeat(12));
                    }
                    for line in lines {
                        println!("│ {}{} │", line, " ".repeat(max_length - line.len()));
                    }
                    if max_length >= 10 {
                        println!("╰{}╯\n", "─".repeat(max_length + 2));
                    } else {
                        println!("╰{}╯\n", "─".repeat(12));
                    }
                }
                io::stdout().flush().expect("Failed to flush stdout");
            }
            9 => {
                if CONFIG.verbose {
                    println!("╭─────────────────────────╮");
                    println!("│ CPU STDIN               │");
                    println!("│ Reading one character.. │");
                    println!("╰─────────────────────────╯\n");
                }
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
            40 => {
                let mut input = String::new();
                if CONFIG.verbose {
                    println!("╭─────────────────────────╮");
                    println!("│ CPU STDIN               │");
                    println!("│ Reading one integer..   │");
                    println!("╰─────────────────────────╯\n");
                }
                match io::stdin().read_line(&mut input) {
                    Ok(_) => match input.trim().parse::<i16>() {
                        Ok(value) => {
                            self.int_reg[0] = value;
                        }
                        Err(e) => {
                            println!("{}", EmuError::ReadFail(e.to_string()));
                            return Err(UnrecoverableError::ReadFail(
                                self.ir,
                                self.pc,
                                Some(e.to_string()),
                            ));
                        }
                    },
                    Err(e) => {
                        println!("{}", EmuError::ReadFail(e.to_string()));
                        return Err(UnrecoverableError::ReadFail(
                            self.ir,
                            self.pc,
                            Some(e.to_string()),
                        ));
                    }
                }
            }
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
