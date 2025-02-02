use crate::{config::CONFIG, interrupt::*, Argument::*, Instruction::*, *};
use colored::Colorize;
use std::{thread, time::Duration};
pub const VMEM_START: usize = 0x1000;
pub const MEMORY_SIZE: usize = 65536;
#[cfg(feature = "window")]
extern crate piston_window;
#[cfg(feature = "window")]
use piston_window::*;

#[cfg(feature = "window")]
use rusttype::Font;

#[cfg(feature = "window")]
const WIDTH: usize = 128;
#[cfg(feature = "window")]
const HEIGHT: usize = 104;
#[cfg(feature = "window")]
const SQUARE_SIZE: f64 = 7.;
#[cfg(feature = "window")]
const FONT_DATA: &[u8] = include_bytes!("../vga.ttf");

use std::sync::mpsc;

#[derive(Debug, Clone)]
pub struct CPU {
    pub int_reg: [i16; 4], // r0 thru r5
    pub uint_reg: [u16; 2],
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: Box<[Option<u16>; MEMORY_SIZE]>,
    pub pc: u16, // program counter
    pub ir: i16, // this doesn't actually impact much, rust just likes to scream
    // about different types, especially with decoder.rs
    // so we have it as an i16 variable instead
    pub starts_at: u16, // .start directive
    pub running: bool,  // running status
    pub has_ran: bool,
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
            int_reg: [0; 4],
            uint_reg: [0; 2],
            float_reg: [0.0; 2],
            memory: Box::new([None; MEMORY_SIZE]),
            pc: 0,
            ir: 0,
            starts_at: 100,
            running: false,
            has_ran: false,
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
        self.has_ran = true;
        self.running = true;
        if self.do_not_run {
            return Ok(());
        }

        #[allow(unused)]
        let (tx, rx) = mpsc::sync_channel(1);

        let execution_handle = {
            let tx = tx.clone();
            let mut self_clone = self.clone();
            thread::spawn(move || {
                while self_clone.running {
                    if let Some(delay) = CONFIG.time_delay {
                        if delay != 0 {
                            thread::sleep(Duration::from_millis(delay.into()));
                        }
                    }

                    match self_clone.memory.get(self_clone.pc as usize) {
                        Some(&Some(instruction)) => {
                            self_clone.ir = instruction as i16;
                        }
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
                            let _ = tx.send(None);
                            return Err(error_msg);
                        }
                    }

                    let parsed_ins = self_clone.decode_instruction();
                    if let Err(e) = self_clone.execute_instruction(&parsed_ins) {
                        self_clone.err = true;
                        self_clone.errmsg = e.only_err().to_string();
                        self_clone.running = false;
                        let _ = tx.send(None);
                        return Err(e);
                    }

                    if CONFIG.verbose {
                        println!("{}", self_clone);
                    }

                    let start = 0xFF;
                    let end = 0x200;
                    if !CONFIG.no_display {
                        let mut stringy = String::new();
                        for index in start..end {
                            if let Some(value) =
                                self_clone.memory.get(index as usize).copied().flatten()
                            {
                                stringy.push(value as u8 as char);
                            }
                        }

                        if self_clone.running {
                            let _ = tx.send(Some(stringy));
                        } else {
                            let _ = tx.send(None);
                        }
                    }
                }
                Ok(())
            })
        };

        #[cfg(feature = "window")]
        if !CONFIG.no_display && !self.debugging {
            let mut window: PistonWindow = WindowSettings::new(
                "BELLE display",
                [
                    WIDTH as u32 * SQUARE_SIZE as u32,
                    HEIGHT as u32 * SQUARE_SIZE as u32,
                ],
            )
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();

            let texture_context = window.create_texture_context();
            let font = Font::try_from_bytes(FONT_DATA).expect("Failed to load font");
            let mut glyphs = Glyphs::from_font(font, texture_context, TextureSettings::new());

            let mut display_text = String::new();
            while let Some(event) = window.next() {
                match rx.recv() {
                    Ok(Some(new_string)) => display_text = new_string,
                    Ok(None) => break,
                    Err(_) => (),
                }

                window.draw_2d(&event, |c, g, _| {
                    clear([0.0, 0.0, 0.0, 1.0], g);

                    let text_color = [1.0, 1.0, 1.0, 1.0];
                    let font_size = 16;
                    let line_height = font_size as f64 * 1.2;
                    for (i, line) in display_text.lines().enumerate() {
                        let transform = c.transform.trans(3.0, 17.0 + i as f64 * line_height);
                        if let Err(e) = text::Text::new_color(text_color, font_size).draw(
                            line,
                            &mut glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        ) {
                            eprintln!("Error drawing text: {}", e);
                        }
                    }
                });

                glyphs.factory.encoder.flush(&mut window.device);
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

        let _ = execution_handle.join().unwrap();
        Ok(())
    }

    pub fn execute_instruction(&mut self, ins: &Instruction) -> PossibleCrash {
        self.has_ran = true; // for debugger

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
    pub fn set_register_value(&mut self, arg: &Argument, value: f32) -> PossibleCrash {
        if let Register(n) = arg {
            if let Err(e) = self.check_overflow(value as i64, *n as u16) {
                eprint!("{e}");
                return Ok(());
            }
            match *n {
                4 => self.uint_reg[0] = value as u16,
                5 => self.uint_reg[1] = value as u16,
                6 => self.float_reg[0] = value,
                7 => self.float_reg[1] = value,
                8 => self.pc = value as u16,
                9 => self.sp = value as u16,
                n if n > 3 => return Err(self.generate_invalid_register()),
                n if n < 0 => return Err(self.generate_invalid_register()),
                _ => self.int_reg[*n as usize] = value as i16,
            }
        }
        Ok(())
    }
}
