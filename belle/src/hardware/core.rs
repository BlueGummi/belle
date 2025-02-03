use crate::{config::CONFIG, interrupt::*, Argument::*, Instruction::*, *};
#[cfg(feature = "window")]
use sdl2::event::Event;
#[cfg(feature = "window")]
use sdl2::keyboard::Keycode;
#[cfg(feature = "window")]
use sdl2::pixels::Color;

use colored::Colorize;
use std::{thread, time::Duration};
pub const MEMORY_SIZE: usize = 65536;

#[cfg(feature = "window")]
use rusttype::*;

#[cfg(feature = "window")]
const WIDTH: usize = 685;
#[cfg(feature = "window")]
const HEIGHT: usize = 480;
#[cfg(feature = "window")]
const FONT_SIZE: usize = 16;

#[cfg(feature = "window")]
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

        #[cfg(feature = "window")]
        let (tx, rx) = mpsc::sync_channel(1);

        let execution_handle = {
            #[cfg(feature = "window")]
            let tx = tx.clone();
            let mut self_clone = self.clone();

            let delay = if let Some(delay) = CONFIG.time_delay {
                delay
            } else {
                0
            };
            let delay = delay as u64;
            thread::spawn(move || {
                while self_clone.running {
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
                            let _ = tx.send(None);
                            return Err(error_msg);
                        }
                    }

                    let parsed_ins = self_clone.decode_instruction();
                    if let Err(e) = self_clone.execute_instruction(&parsed_ins) {
                        self_clone.err = true;
                        self_clone.errmsg = e.only_err().to_string();
                        self_clone.running = false;
                        #[cfg(feature = "window")]
                        let _ = tx.send(None);
                        return Err(e);
                    }

                    if CONFIG.verbose {
                        println!("{}", self_clone);
                    }

                    if !CONFIG.no_display {
                        let mut stringy = String::with_capacity(5000);
                        for index in 0xFF..0x9C9 {
                            if let Some(value) =
                                self_clone.memory.get(index as usize).and_then(|&x| x)
                            {
                                if index % 76 == 0 {
                                    stringy.push('\n');
                                }
                                stringy.push(value as u8 as char);
                            }
                        }

                        #[cfg(feature = "window")]
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
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();
            let window = video_subsystem
                .window("BELLE display", WIDTH as u32, HEIGHT as u32)
                .position_centered()
                .build()
                .unwrap();
            let mut canvas = window.into_canvas().build().unwrap();

            let font_data = include_bytes!("../vga.ttf");
            let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
            let scale = Scale::uniform(FONT_SIZE as f32);

            let mut display_text = String::new();
            let mut event_pump = sdl_context.event_pump().unwrap();

            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        _ => {}
                    }
                }

                match rx.try_recv() {
                    Ok(Some(new_string)) => display_text = new_string,
                    Ok(None) => break 'running,
                    _ => {}
                }

                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                let mut y = 0;
                let texture_creator = canvas.texture_creator();
                let mut text_texture = texture_creator
                    .create_texture_target(None, WIDTH as u32, HEIGHT as u32)
                    .unwrap();
                canvas
                    .with_texture_canvas(&mut text_texture, |text_canvas| {
                        for line in display_text.lines() {
                            let glyphs: Vec<_> =
                                font.layout(line, scale, point(0.0, y as f32)).collect();
                            for glyph in glyphs {
                                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                                    glyph.draw(|x, y, v| {
                                        let x = x as i32 + bounding_box.min.x;
                                        let y = y as i32 + bounding_box.min.y;
                                        if x >= 0 && y >= 0 && x < WIDTH as i32 && y < HEIGHT as i32
                                        {
                                            text_canvas.set_draw_color(Color::RGB(
                                                (v * 255.0) as u8,
                                                (v * 255.0) as u8,
                                                (v * 255.0) as u8,
                                            ));
                                            text_canvas
                                                .draw_point(sdl2::rect::Point::new(x, y))
                                                .unwrap();
                                        }
                                    });
                                }
                            }
                            y += FONT_SIZE as i32;
                        }
                    })
                    .unwrap();
                canvas.copy(&text_texture, None, None).unwrap();
                canvas.present();
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
