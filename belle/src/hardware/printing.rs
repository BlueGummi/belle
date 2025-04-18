use crate::config::CONFIG;
use crate::CPU;
use crate::*;
use colored::*;
use std::fmt;

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if CONFIG.compact_print {
            if self.err {
                writeln!(f, "{}", self.errmsg)?;
                return Ok(());
            }
            let r0 = self.int_reg[0].to_string().magenta();
            let r1 = self.int_reg[1].to_string().magenta();
            let r2 = self.int_reg[2].to_string().magenta();
            let r3 = self.int_reg[3].to_string().magenta();
            let r4 = self.int_reg[4].to_string().magenta();
            let r5 = self.int_reg[5].to_string().magenta();
            let r6 = self.float_reg[0].to_string().magenta();
            let r7 = self.float_reg[1].to_string().magenta();
            let z = if self.zflag { "z".green() } else { "z".red() };

            let o = if self.oflag { "o".green() } else { "o".red() };

            let s = if self.sflag { "s".green() } else { "s".red() };
            let pc = format!("x{:X}", self.pc).blue();
            let sp = format!("x{:X}", self.sp).cyan();
            let bp = format!("x{:X}", self.bp).cyan();
            write!(f, "│ {:14} │ r0: {r0:^5} │ r1: {r1:^5} │ r2: {r2:^5} │ r3: {r3:^5} │ r4: {r4:^5} │ r5: {r5:^5} │ r6: {r6:^5} │ r7: {r7:^5} │ sp: {sp:^5} │ bp: {bp:^5} │ pc: {pc:^5} │ {z}{o}{s} │", self.decode_instruction().to_string().to_lowercase().bright_blue().bold())?;
            return Ok(());
        }
        let times = 12;
        let line = "─".repeat(times);
        let midpart = format!("├{}┼{}┼{}┼{}┴{}┼{}┤\n", line, line, line, line, line, line);
        if !self.err {
            writeln!(f, "╭{}────────╮", line)?;
        } else {
            writeln!(
                f,
                "╭{}─────────┬{}─{}─{}─{}────╮",
                line, line, line, line, line
            )?;
        }
        let exit = if self.running && !self.err {
            "RUNNING".green()
        } else if self.err {
            "CRASHED".bright_red()
        } else if self.debugging {
            "DEBUG".bright_purple()
        } else {
            "HALTED".red()
        };
        write!(f, "│ {} {}", "CPU STATE:".green(), exit)?;
        for _ in (exit.len() + 11)..19 {
            write!(f, " ")?;
        }
        if !self.err {
            writeln!(f, "│")?;
        } else {
            write!(f, " │ ")?;
        }
        if self.err {
            write!(f, "{}", self.errmsg)?;
            let length = length_without_ansi(&exit) + length_without_ansi(self.errmsg.trim());
            for _ in length..61 {
                write!(f, " ")?;
            }
            writeln!(f, "│")?;
        }
        if !self.err {
            writeln!(
                f,
                "├{}────────┴{}─{}─{}─{}─────╮",
                line, line, line, line, line
            )?;
        } else {
            writeln!(
                f,
                "├{}─────────┴{}─{}─{}─{}────┤",
                line, line, line, line, line
            )?;
        }
        write!(f, "│{}:", " Instruction".bold())?;
        write!(f, " {}", self.decode_instruction().to_string().bold())?;
        let inslen =
            78 - "│ Instruction".len() - self.decode_instruction().to_string().trim().len();
        writeln!(f, "{}│", " ".repeat(inslen))?;
        let mut register_lines = Vec::new();

        writeln!(f, "├{}┬{}┬{}┬{}┬{}┬{}┤", line, line, line, line, line, line)?;

        for (i, &val) in self.int_reg.iter().enumerate() {
            register_lines.push(format!(
                "{}: {:^6} │",
                format!("r{}", i).bold().green(),
                val.to_string().bold().magenta()
            ));
        }

        writeln!(f, "│ {}", register_lines.join(" "))?;
        write!(f, "{midpart}")?;
        register_lines.clear();
        for (i, &val) in self.float_reg.iter().enumerate() {
            register_lines.push(format!(
                "{}: {:^6.6} │",
                format!("r{}", i + 6).bold().green(),
                val.to_string().bold().magenta()
            ));
        }

        write!(f, "│ {} ", register_lines.join(" "))?;

        let midpart = format!("├{}┼{}┼{}┼{}┬{}┼{}┤\n", line, line, line, line, line, line);
        let output = format!(
            "{}: {:^4}   │ {}: {:016b}    │ {}: {:^4}   │\n{}│ {}: {:^4}   │           ",
            "pc".truecolor(252, 244, 52),
            format!("x{:X}", self.pc).bold(),
            "ir".truecolor(252, 244, 52),
            self.ir,
            "sp".truecolor(156, 89, 209),
            format!("x{:X}", self.sp).bold(),
            midpart,
            "bp".truecolor(156, 89, 209),
            format!("x{:X}", self.bp).bold(),
        );

        write!(f, "{} │", output)?;
        writeln!(
            f,
            " {}: {} │ {}: {} │ {}: {} │ {}: {} │",
            "zf".bright_green().bold(),
            if self.zflag {
                " set  ".green()
            } else {
                "unset ".red()
            },
            "of".bright_red().bold(),
            if self.oflag {
                " set  ".green()
            } else {
                "unset ".red()
            },
            "rf".bright_white().bold(),
            if self.rflag {
                " set  ".green()
            } else {
                "unset ".red()
            },
            "sf".bright_purple().bold(),
            if self.sflag {
                " set  ".green()
            } else {
                "unset ".red()
            },
        )?;
        let footer = format!("╰{}┴{}┴{}┴{}┴{}┴{}╯", line, line, line, line, line, line);
        writeln!(f, "{}", footer)?;
        writeln!(f, "╭─────────────────╮")?;
        writeln!(
            f,
            "│ pushret: {} │",
            if self.pushret {
                " set  ".green()
            } else {
                "unset ".red()
            },
        )?;
        writeln!(f, "╰─────────────────╯")?;

        if self.pmem {
            writeln!(f, "╭{}─{}─{}─{}─{}─{}╮", line, line, line, line, line, line)?;
            writeln!(
                f,
                "│ {}{}│",
                "MEMORY".bright_purple().bold(),
                " ".repeat(70)
            )?;
            writeln!(
                f,
                "├─────────┬────────{}─────{}─────{}─{}┤",
                line, line, line, line
            )?;
            writeln!(
                f,
                "│ {} │ {}   {}│",
                "ADDRESS".bright_purple().bold(),
                "VALUE".bright_cyan().bold(),
                " ".repeat(58)
            )?;
            writeln!(
                f,
                "├─────────┼────────{}─────{}─────{}─{}┤",
                line, line, line, line
            )?;
            for (index, element) in self.memory.iter().enumerate() {
                let value = element;
                if *value != 0 {
                    let mut temp = CPU::new();
                    temp.ir = *value as i16;
                    let displayed = format!(
                        "│  {}  │ {}",
                        format!("x{:04X}", index).magenta(),
                        temp.decode_instruction().to_string().green()
                    );
                    write!(f, "{displayed}")?;
                    let spacelen = 50 - displayed.len();
                    for _ in displayed.len()..50 {
                        write!(f, " ")?;
                    }
                    let binary_string = format!("{:016b}", value).bright_white();
                    write!(
                        f,
                        " - {} ({})",
                        binary_string,
                        value.to_string().bright_green()
                    )?;

                    let numberlen = format!(" - {:016b} ({})", value, value).len();

                    for _ in numberlen..30 {
                        write!(f, " ")?;
                    }

                    let spacelen = spacelen + (30 - numberlen);

                    let escapelen = if *value <= 127 {
                        if *value < 32 {
                            let escape_code = match *value {
                                0 => "\\0",
                                1 => "\\a",
                                2 => "\\b",
                                3 => "\\t",
                                4 => "\\n",
                                5 => "\\v",
                                6 => "\\f",
                                7 => "\\r",
                                8 => "\\x08",
                                9 => "\\x09",
                                10 => "\\n",
                                11 => "\\x0b",
                                12 => "\\f",
                                13 => "\\r",
                                _ => &format!("\\x{:02x}", *value),
                            };
                            write!(f, " [{}]", escape_code)?;
                            format!(" [{}]", escape_code).len()
                        } else if *value == 127 {
                            write!(f, " [DEL]")?;
                            6
                        } else {
                            write!(f, " [{}]", *value as u8 as char)?;
                            format!(" [{}]", *value as u8 as char).len()
                        }
                    } else {
                        0
                    };

                    let mut pointer_ind = if self.sp as usize == index && self.bp as usize == index
                    {
                        "  <-- s. ptr & b. ptr".to_string()
                    } else if self.sp as usize == index {
                        "  <-- s. ptr".to_string()
                    } else if self.bp as usize == index {
                        "  <-- b. ptr".to_string()
                    } else {
                        String::new()
                    };

                    if self.pc as usize == index {
                        if !pointer_ind.is_empty() {
                            pointer_ind = format!("{} & pc", pointer_ind);
                        } else {
                            pointer_ind = "  <-- pc".to_string();
                        }
                    }

                    write!(f, "{}", pointer_ind.green())?;

                    let complete =
                        displayed.len() + numberlen + spacelen + escapelen + pointer_ind.len();

                    for _ in complete..100 {
                        write!(f, " ")?;
                    }

                    writeln!(f, "│")?;
                } else if self.sp as usize == index && self.bp as usize == index {
                    writeln!(
                        f,
                        "│  {}  │    ╺{}Stack and base pointer{}╸    │",
                        format!("x{:04X}", index).magenta(),
                        "─".repeat(18),
                        "─".repeat(17)
                    )?;
                } else if self.sp as usize == index {
                    writeln!(
                        f,
                        "│  {}  │    ╺{}────Stack pointer─────{}╸    │",
                        format!("x{:04X}", index).magenta(),
                        "─".repeat(18),
                        "─".repeat(17)
                    )?;
                } else if self.bp as usize == index {
                    writeln!(
                        f,
                        "│  {}  │    ╺{}─────Base pointer─────{}╸    │",
                        format!("x{:04X}", index).magenta(),
                        "─".repeat(18),
                        "─".repeat(17)
                    )?;
                }
            }
            writeln!(
                f,
                "╰─────────┴────────{}─────{}─────{}─{}╯",
                line, line, line, line
            )?;
        }

        Ok(())
    }
}

impl CPU {
    pub fn pmem(&mut self) {
        let times = 12;
        let line = "─".repeat(times);
        println!("╭{}─{}─{}─{}─{}─{}╮", line, line, line, line, line, line);
        println!("│ {}{}│", "MEMORY".bright_purple().bold(), " ".repeat(70));
        println!(
            "├─────────┬────────{}─────{}─────{}─{}┤",
            line, line, line, line
        );
        println!(
            "│ {} │ {}   {}│",
            "ADDRESS".bright_purple().bold(),
            "VALUE".bright_cyan().bold(),
            " ".repeat(58)
        );
        println!(
            "├─────────┼────────{}─────{}─────{}─{}┤",
            line, line, line, line
        );
        for (index, element) in self.memory.iter().enumerate() {
            let value = element;
            if *element != 0 {
                let mut temp = CPU::new();
                temp.ir = *value as i16;
                let displayed = format!(
                    "│  {}  │ {}",
                    format!("x{:04X}", index).magenta(),
                    temp.decode_instruction().to_string().green()
                );
                print!("{displayed}");
                let spacelen = 50 - displayed.len();
                for _ in displayed.len()..50 {
                    print!(" ");
                }
                print!(
                    " - {} ({})",
                    format!("{:016b}", value).bright_white(),
                    value.to_string().bright_green()
                );

                let numberlen = format!(" - {:016b} ({})", value, value).len();

                for _ in numberlen..30 {
                    print!(" ");
                }

                let spacelen = spacelen + (30 - numberlen);

                let escapelen = if *value <= 127 {
                    if *value < 32 {
                        let escape_code = match *value {
                            0 => "\\0",
                            1 => "\\a",
                            2 => "\\b",
                            3 => "\\t",
                            4 => "\\n",
                            5 => "\\v",
                            6 => "\\f",
                            7 => "\\r",
                            8 => "\\x08",
                            9 => "\\x09",
                            10 => "\\n",
                            11 => "\\x0b",
                            12 => "\\f",
                            13 => "\\r",
                            _ => &format!("\\x{:02x}", *value),
                        };
                        print!(" [{}]", escape_code);
                        format!(" [{}]", escape_code).len()
                    } else if *value == 127 {
                        print!(" [DEL]");
                        6
                    } else {
                        print!(" [{}]", *value as u8 as char);
                        format!(" [{}]", *value as u8 as char).len()
                    }
                } else {
                    0
                };

                let mut pointer_ind = if self.sp as usize == index && self.bp as usize == index {
                    "  <-- s. ptr & b. ptr".to_string()
                } else if self.sp as usize == index {
                    "  <-- s. ptr".to_string()
                } else if self.bp as usize == index {
                    "  <-- b. ptr".to_string()
                } else {
                    String::new()
                };

                if self.pc as usize == index {
                    if !pointer_ind.is_empty() {
                        pointer_ind = format!("{} & pc", pointer_ind);
                    } else {
                        pointer_ind = "  <-- pc".to_string();
                    }
                }

                print!("{}", pointer_ind.green());

                let complete =
                    displayed.len() + numberlen + spacelen + escapelen + pointer_ind.len();

                for _ in complete..100 {
                    print!(" ");
                }

                println!("│");
            } else if self.sp as usize == index && self.bp as usize == index {
                println!(
                    "│  {}  │    ╺{}Stack and base pointer{}╸    │",
                    format!("x{:04X}", index).magenta(),
                    "─".repeat(18),
                    "─".repeat(17)
                );
            } else if self.sp as usize == index {
                println!(
                    "│  {}  │    ╺{}────Stack pointer─────{}╸    │",
                    format!("x{:04X}", index).magenta(),
                    "─".repeat(18),
                    "─".repeat(17)
                );
            } else if self.bp as usize == index {
                println!(
                    "│  {}  │    ╺{}─────Base pointer─────{}╸    │",
                    format!("x{:04X}", index).magenta(),
                    "─".repeat(18),
                    "─".repeat(17)
                );
            }
        }
        println!(
            "╰─────────┴────────{}─────{}─────{}─{}╯",
            line, line, line, line
        );
    }
}
