use crate::{config::CONFIG, *};
use colored::*;
use std::io::{self, Read, Write};

#[cfg(feature = "window")]
extern crate piston_window;
#[cfg(feature = "window")]
use crate::UnrecoverableError::*;
#[cfg(feature = "window")]
use piston_window::*;

#[cfg(feature = "window")]
use rusttype::Font;

#[cfg(feature = "window")]
use std::time::{Duration, Instant};

#[cfg(feature = "window")]
const WIDTH: usize = 128;
#[cfg(feature = "window")]
const HEIGHT: usize = 104;
#[cfg(feature = "window")]
const SQUARE_SIZE: f64 = 10.;
#[cfg(feature = "window")]
const FONT_DATA: &[u8] = include_bytes!("../../../src/vga.ttf");
impl CPU {
    pub fn handle_int(&mut self, arg: &Argument) -> PossibleCrash {
        if self.fuzz {
            return Ok(());
        }
        let code = self.get_value(arg)? as u16;
        match code {
            0_u16..=3_u16 => {
                if CONFIG.verbose || CONFIG.debug {
                    print_b();
                    println!("╭─────────╮");
                    println!("│ {:^5}   │", self.int_reg[code as usize]);
                    println!("╰─────────╯");
                    print_t();
                } else {
                    println!("{}", self.int_reg[code as usize]);
                }
            }
            4 => {
                if CONFIG.verbose || CONFIG.debug {
                    print_b();
                    println!("╭─────────╮");
                    println!("│ {:^5}   │", self.uint_reg[0]);
                    println!("╰─────────╯");
                    print_t();
                } else {
                    println!("{}", self.uint_reg[0]);
                }
            }
            5 => {
                if CONFIG.verbose || CONFIG.debug {
                    print_b();
                    println!("╭─────────╮");
                    println!("│ {:^5}   │", self.uint_reg[1]);
                    println!("╰─────────╯");
                    print_t();
                } else {
                    println!("{}", self.uint_reg[1]);
                }
            }
            6 => {
                if CONFIG.verbose || CONFIG.debug {
                    print_b();
                    println!("╭─────────╮");
                    println!("│ {:^5.5} │", self.float_reg[0]);
                    println!("╰─────────╯");
                    print_t();
                } else {
                    println!("{}", self.float_reg[0]);
                }
            }
            7 => {
                if CONFIG.verbose || CONFIG.debug {
                    print_b();
                    println!("╭─────────╮");
                    println!("│ {:^5.5} │", self.float_reg[1]);
                    println!("╰─────────╯");
                    print_t();
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
                        if CONFIG.verbose || CONFIG.debug {
                            stringy = format!("{}{}", stringy, value as u8 as char);
                        } else {
                            print!("{}", value as u8 as char);
                        }
                    }
                }
                if CONFIG.verbose || CONFIG.debug {
                    print_b();
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
                        println!("╰{}╯", "─".repeat(max_length + 2));
                    } else {
                        println!("╰{}╯", "─".repeat(12));
                    }
                    if !CONFIG.compact_print {
                        println!();
                    }
                    print_t();
                }
                io::stdout().flush().expect("Failed to flush stdout");
            }
            9 => {
                if CONFIG.verbose || CONFIG.debug {
                    print_b();
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
                print_t();
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
                print_b();
                loop {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => match input.trim().parse::<i16>() {
                            Ok(value) => {
                                self.int_reg[0] = value;
                                break;
                            }
                            Err(e) => {
                                println!("{}", EmuError::ReadFail(e.to_string()));
                            }
                        },
                        Err(e) => {
                            println!("{}", EmuError::ReadFail(e.to_string()));
                        }
                    }
                }

                print_t();
            }
            41 => self.sflag = true,
            42 => self.sflag = false,
            43 => self.sflag = !self.sflag,

            51 => self.hlt_on_overflow = true,
            52 => self.hlt_on_overflow = false,
            53 => self.hlt_on_overflow = !self.hlt_on_overflow,

            60 => self.sp = self.uint_reg[0],
            61 => self.bp = self.uint_reg[0],
            70 => self.pushret = true,
            71 => self.pushret = false,
            #[cfg(feature = "window")]
            100 => {
                if CONFIG.no_display {
                    return Ok(());
                }
                let duration = Duration::new(self.uint_reg[0] as u64, 0);
                let start_time = Instant::now();

                let window = WindowSettings::new(
                    "BELLE display",
                    [
                        WIDTH as u32 * SQUARE_SIZE as u32,
                        HEIGHT as u32 * SQUARE_SIZE as u32,
                    ],
                )
                .exit_on_esc(true)
                .build::<PistonWindow>();

                let mut window = match window {
                    Ok(win) => win,
                    Err(e) => {
                        return Err(WindowFail(
                            self.ir,
                            self.pc,
                            Some(format!(
                                "Failed to create window on interrupt call 100: {}",
                                e
                            )),
                        ))
                    }
                };

                let pixel_data: [[u16; 8]; 104] = {
                    let mut data = [[0; 8]; 104];
                    for (i, row) in data.iter_mut().enumerate() {
                        for (j, col) in row.iter_mut().enumerate() {
                            let index = VMEM_START + (i * 8 + j) * std::mem::size_of::<u16>();
                            *col = self.memory.get(index).unwrap_or(&None).unwrap_or(0) as u16;
                        }
                    }
                    data
                };

                while let Some(event) = window.next() {
                    if start_time.elapsed() >= duration {
                        break;
                    }
                    window.draw_2d(&event, |c, g, _| {
                        clear([0.0, 0.0, 0.0, 1.0], g);

                        for (row_index, row) in pixel_data.iter().enumerate() {
                            for (u16_index, &u16_value) in row.iter().enumerate() {
                                for x in 0..16 {
                                    let is_set = (u16_value >> (15 - x)) & 1 == 1;
                                    let color = if is_set {
                                        [1.0, 1.0, 1.0, 1.0]
                                    } else {
                                        [0.0, 0.0, 0.0, 1.0]
                                    };
                                    rectangle(
                                        color,
                                        [
                                            (u16_index * 16 + x) as f64 * SQUARE_SIZE,
                                            row_index as f64 * SQUARE_SIZE,
                                            SQUARE_SIZE,
                                            SQUARE_SIZE,
                                        ],
                                        c.transform,
                                        g,
                                    );
                                }
                            }
                        }
                    });
                }
            }
            #[cfg(feature = "window")]
            101 => {
                let duration = Duration::new(self.uint_reg[0] as u64, 0);
                let start_time = Instant::now();
                let starting_point = self.int_reg[0];
                let end_point = self.int_reg[1];
                let mut stringy = String::from("");
                for index in starting_point..end_point {
                    if index < 0 || index as usize >= self.memory.len() {
                        return Err(self.generate_segfault(
                            "Segmentation fault. Memory index out of bounds on interrupt call 8.",
                        ));
                    }

                    if let Some(value) = self.memory[index as usize] {
                        stringy = format!("{}{}", stringy, value as u8 as char);
                    }
                }

                let width = WIDTH as u32 * SQUARE_SIZE as u32;
                let height = HEIGHT as u32 * SQUARE_SIZE as u32;
                let mut window: PistonWindow =
                    WindowSettings::new("BELLE display", [width, height])
                        .exit_on_esc(true)
                        .build()
                        .unwrap();
                let texture_context = window.create_texture_context();

                let font = Font::try_from_bytes(FONT_DATA).expect("Failed to load font");
                let mut glyphs = Glyphs::from_font(font, texture_context, TextureSettings::new());

                while let Some(event) = window.next() {
                    if start_time.elapsed() >= duration {
                        break;
                    }
                    window.draw_2d(&event, |c, g, _| {
                        clear([0.0, 0.0, 0.0, 1.0], g);

                        let transform = c.transform.trans(50.0, 100.0);
                        let text_color = [1.0, 1.0, 1.0, 1.0];
                        let font_size = 32;

                        if let Err(e) = text::Text::new_color(text_color, font_size).draw(
                            &stringy,
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        ) {
                            eprintln!("Error drawing text: {}", e);
                        }
                    });
                    glyphs.factory.encoder.flush(&mut window.device);
                }
            }
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

pub fn print_b() {
    if CONFIG.compact_print && CONFIG.verbose {
        println!("╰────────────────┴───────────┴───────────┴───────────┴───────────┴───────────┴───────────┴───────────┴───────────┴───────────┴───────────┴───────────┴─────╯");
    }
}

pub fn print_t() {
    if CONFIG.compact_print && CONFIG.verbose {
        println!("╭────────────────┬───────────┬───────────┬───────────┬───────────┬───────────┬───────────┬───────────┬───────────┬───────────┬───────────┬───────────┬─────╮");
    }
}
