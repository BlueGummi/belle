use crate::{config::CONFIG, interrupt::*, Argument::*, Instruction::*, *};
use colored::Colorize;
use std::{thread, time::Duration};
pub const MEMORY_SIZE: usize = 65536;

#[cfg(feature = "window")]
use fontdue::{Font, FontSettings};
#[cfg(feature = "window")]
use minifb::{Key, Window, WindowOptions};

#[cfg(feature = "window")]
const WIDTH: usize = 685;
#[cfg(feature = "window")]
const HEIGHT: usize = 480;
#[cfg(feature = "window")]
const FONT_SIZE: f32 = 16.0;

#[cfg(feature = "window")]
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct CPU {
    pub int_reg: [u16; 6],   // r0 thru r5
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: Box<[Option<u16>; MEMORY_SIZE]>,
    pub pc: u16, // program counter
    pub ir: i16, // this doesn't actually impact much, rust just likes to scream
    // about different types, especially with decoder.rs
    // so we have it as an i16 variable instead
    pub starts_at: u16, // .start directive
    pub running: bool,  // running status
    pub zflag: bool,
    pub oflag: bool,
    pub rflag: bool,
    pub sflag: bool,
    pub sp: u16,
    pub bp: u16,
    pub backward_stack: bool,
    pub do_not_run: bool,
    pub err: bool,
    pub debugging: bool,
    pub errmsg: String,
    pub pmem: bool,
    pub pushret: bool,
    pub fuzz: bool,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU::new()
    }
}

impl CPU {
    #[must_use]
    pub fn new() -> CPU {
        CPU {
            int_reg: [0; 6],
            float_reg: [0.0; 2],
            memory: Box::new([None; MEMORY_SIZE]),
            pc: 0,
            ir: 0,
            starts_at: 100,
            running: false,
            zflag: false,
            oflag: false,
            rflag: false,
            sflag: false,
            sp: 99,
            bp: 99,
            backward_stack: false,
            do_not_run: false,
            err: false,
            pushret: true,
            debugging: false,
            errmsg: String::from(""),
            pmem: false,
            fuzz: false,
        }
    }
    pub fn run(&mut self) -> PossibleCrash {
        self.running = true;
        if self.do_not_run {
            return Ok(());
        }
        let mut can_make_window = true;
        #[cfg(feature = "window")]
        let mut window_attempt = None;
        #[cfg(feature = "window")]
        if !CONFIG.no_display {
            window_attempt = Some(Window::new(
                "BELLE display",
                WIDTH,
                HEIGHT,
                WindowOptions {
                    resize: true,
                    scale: minifb::Scale::X1,
                    scale_mode: minifb::ScaleMode::AspectRatioStretch,
                    ..WindowOptions::default()
                },
            ));
        }
        #[cfg(feature = "window")]
        let mut window = None;
        #[cfg(feature = "window")]
        if !CONFIG.no_display {
            window = match window_attempt {
                Some(Ok(w)) => Some(w),
                Some(Err(e)) => {
                    if !CONFIG.no_display {
                        eprintln!("{}: {e}", "Emulator cannot create window".bright_red());
                    }
                    can_make_window = false;
                    None
                }
                None => None,
            };
        }

        #[cfg(feature = "window")]
        let (tx, rx) = mpsc::sync_channel(1);

        let execution_handle = {
            let mut self_clone = self.clone();

            let delay = CONFIG.time_delay.unwrap_or_default();
            let delay = delay as u64;
            thread::spawn(move || {
                let mut cycles = 0;
                let starting = std::time::Instant::now();
                while self_clone.running {
                    cycles += 1;
                    if delay != 0 {
                        thread::sleep(Duration::from_millis(delay));
                    }
                    match self_clone.memory[self_clone.pc as usize] {
                        Some(instruction) => self_clone.ir = instruction as i16,
                        _ => {
                            self_clone.err = true;
                            let error_msg = UnrecoverableError::SegmentationFault(
                                self_clone.ir,
                                self_clone.pc,
                                Some(
                                    "Segmentation fault while finding next instruction".to_string(),
                                ),
                            );
                            self_clone.errmsg = error_msg.only_err().to_string();
                            self_clone.running = false;
                            #[cfg(feature = "window")]
                            if can_make_window {
                                let _ = tx.send(None);
                            }
                            return Err(error_msg);
                        }
                    }

                    let parsed_ins = self_clone.decode_instruction();
                    if let Err(e) = self_clone.execute_instruction(&parsed_ins) {
                        self_clone.err = true;
                        self_clone.errmsg = e.only_err().to_string();
                        self_clone.running = false;
                        #[cfg(feature = "window")]
                        if can_make_window {
                            let _ = tx.send(None);
                        }
                        if CONFIG.verbose {
                            println!("{}", self_clone);
                        }
                        return Err(e);
                    }

                    if CONFIG.verbose {
                        println!("{}", self_clone);
                    }

                    if !CONFIG.no_display && can_make_window {
                        let mut stringy = String::with_capacity(5000);
                        for index in 0xFF..0x9C9 {
                            if let Some(value) =
                                self_clone.memory.get(index as usize).and_then(|&x| x)
                            {
                                if (index - 0xFF) % 76 == 0 {
                                    stringy.push('\n');
                                }
                                if value as u8 != 10 && value as u8 as char != '\t' {
                                    stringy.push(value as u8 as char);
                                }
                            }
                        }

                        #[cfg(feature = "window")]
                        if self_clone.running {
                            let _ = tx.try_send(Some(stringy)).ok();
                        } else if can_make_window {
                            let _ = tx.send(None).ok();
                        }
                    }
                    cycles += 1;
                }
                /*println!(
                    "We took {:?} to execute {} instructions",
                    starting.elapsed(),
                    cycles
                );*/
                if CONFIG.pretty {
                    self_clone.pmem = !CONFIG.no_print_memory;
                    println!("{self_clone}");
                }
                Ok(())
            })
        };

        #[cfg(target_os = "linux")]
        configure_wayland();

        #[cfg(feature = "window")]
        if !CONFIG.no_display && !self.debugging && can_make_window {
            let mut window = window.unwrap();
            let font_data = include_bytes!("../vga.ttf");
            let font = Font::from_bytes(font_data as &[u8], FontSettings::default()).unwrap();

            let mut display_text = String::new();
            let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
            window.set_target_fps(60);
            while window.is_open() && !window.is_key_down(Key::Escape) {
                match rx.try_recv() {
                    Ok(Some(new_string)) => display_text = new_string,
                    Ok(None) => break,
                    Err(mpsc::TryRecvError::Empty) => {}
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }

                buffer.iter_mut().for_each(|p| *p = 0);
                let mut x = 0;
                let mut y = 0;
                for line in display_text.lines() {
                    for character in line.chars() {
                        let (metrics, bitmap) = font.rasterize(character, FONT_SIZE);

                        for (i, alpha) in bitmap.iter().enumerate() {
                            let px = x + (i % metrics.width);
                            let py =
                                y + (i / metrics.width) + (FONT_SIZE as usize - metrics.height);

                            if px < WIDTH && py < HEIGHT {
                                let color = if *alpha > 0 {
                                    let alpha_channel = (*alpha as u32) << 24;
                                    let rgb_color = 0xFFFFFF;
                                    alpha_channel | rgb_color
                                } else {
                                    0
                                };

                                buffer[py * WIDTH + px] = color;
                            }
                        }

                        x += metrics.advance_width as usize;
                    }

                    x = 0;
                    y += FONT_SIZE as usize;
                }

                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            }
        }
        if !self.running {
            if CONFIG.verbose && !CONFIG.compact_print {
                println!("╭────────────╮");
                println!("│ {} │", "Halting...".bold().white());
                println!("╰────────────╯");
            }
            self.pmem = !CONFIG.no_print_memory;
            if CONFIG.pretty ^ CONFIG.verbose {
                println!("{self}");
            }
            print_b();
        }

        execution_handle.join().unwrap()?;
        Ok(())
    }

    pub fn execute_instruction(&mut self, ins: &Instruction) -> PossibleCrash {
        match ins {
            HLT => self.running = false,
            ADD(arg1, arg2) => self.handle_add(arg1, arg2)?,
            BO(arg) => self.handle_bo(arg)?,
            BNO(arg) => self.handle_bno(arg)?,
            POP(arg) => self.handle_pop(arg)?,
            DIV(arg1, arg2) => self.handle_div(arg1, arg2)?,
            RET => self.handle_ret()?,
            BL(arg) => self.handle_bl(arg)?,
            BG(arg) => self.handle_bg(arg)?,
            LD(arg1, arg2) => self.handle_ld(arg1, arg2)?,
            ST(arg1, arg2) => self.handle_st(arg1, arg2)?,
            JMP(arg) => self.handle_jmp(arg)?,
            BZ(arg) => self.handle_bz(arg)?,
            BNZ(arg) => self.handle_bnz(arg)?,
            CMP(arg1, arg2) => self.handle_cmp(arg1, arg2)?,
            NAND(arg1, arg2) => self.handle_nand(arg1, arg2)?,
            PUSH(arg) => self.handle_push(arg)?,
            INT(arg) => self.handle_int(arg)?,
            MOV(arg1, arg2) => self.handle_mov(arg1, arg2)?,
            LEA(arg1, arg2) => self.handle_lea(arg1, arg2)?,
        }
        Ok(())
    }
    pub fn set_register_value(&mut self, arg: &Argument, value: f64) -> PossibleCrash {
        if let Register(n) = arg {
            if let Err(e) = self.check_overflow(value as i64, *n as u16) {
                eprint!("{e}");
                return Ok(());
            }
            match *n {
                6 => self.float_reg[0] = value as f32,
                7 => self.float_reg[1] = value as f32,
                8 => self.pc = value as u16,
                9 => self.sp = value as u16,
                n if !(0..=5).contains(&n) => return Err(self.generate_invalid_register()),
                _ => self.int_reg[*n as usize] = value as u16,
            }
            if let Err(e) = self.check_overflow(value as i64, *n as u16) {
                eprint!("{e}");
            }
        }
        Ok(())
    }
}
#[cfg(target_os = "linux")]
fn configure_wayland() {
    std::env::set_var("WAYLAND_DISPLAY", "wayland-0");
}
