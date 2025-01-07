use crate::{config::CONFIG, Argument::*, Instruction::*, *};
use colored::Colorize;
use std::arch::asm;
use std::{thread, time::Duration};

pub const MEMORY_SIZE: usize = 65536;

macro_rules! trust_me {
    ($input:expr) => {
        unsafe {
            asm!($input);
        }
    };
}

#[derive(Debug)]
pub struct CPU {
    pub int_reg: [i16; 4], // r0 thru r5
    pub uint_reg: [u16; 2],
    pub float_reg: [f32; 2],                     // r6 and r7
    pub memory: Box<[Option<u16>; MEMORY_SIZE]>,
    pub pc: u16,                                 // program counter
    pub ir: i16, // this doesn't actually impact much, rust just likes to scream
                 // about different types, especially with decoder.rs
                 // so we have it as an i16 variable instead
    pub starts_at: u16, // .start directive
    pub running: bool, // running status
    pub has_ran: bool,
    pub zflag: bool,
    pub oflag: bool,
    pub rflag: bool,
    pub sflag: bool,
    pub hlt_on_overflow: bool,
    pub sp: u16,
    pub bp: u16,
    pub ip: u16,
    pub backward_stack: bool,
    pub max_clk: Option<usize>,
    pub hit_max_clk: bool,
    pub do_not_run: bool,
    pub err: bool,
    pub debugging: bool,
    pub errmsg: String,
    pub pmem: bool,
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
            hlt_on_overflow: false,
            sp: 99,
            bp: 99,
            ip: 0,
            backward_stack: false,
            max_clk: None,
            hit_max_clk: false,
            do_not_run: false,
            err: false,
            debugging: false,
            errmsg: String::from(""),
            pmem: false,
        }
    }

    pub fn run(&mut self) -> PossibleCrash {
        self.has_ran = true; // for debugger
        self.running = true;
        if self.do_not_run {
            return Ok(());
        }
        while self.running {
            if !CONFIG.debug {
                let _ = ctrlc::set_handler(move || {
                    println!("Halting...");
                    std::process::exit(0);
                });
            }
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            if CONFIG.time_delay != Some(0) && CONFIG.time_delay.is_some() {
                thread::sleep(Duration::from_millis(CONFIG.time_delay.unwrap().into()));
            }
            std::mem::drop(clock); // clock must go bye bye so it unlocks

            match self.memory[self.pc as usize] {
                Some(instruction) => {
                    self.ip = self.pc;
                    self.ir = instruction as i16;
                }
                None => {
                    self.err = true;
                    self.errmsg = UnrecoverableError::SegmentationFault(
                        self.ir,
                        self.pc,
                        Some("Segmentation fault while finding next instruction".to_string()),
                    )
                    .only_err()
                    .to_string();
                    self.pmem = true;
                    if CONFIG.pretty || CONFIG.verbose {
                        println!("{self}");
                    }
                    return Err(UnrecoverableError::SegmentationFault(
                        self.ir,
                        self.pc,
                        Some("Segmentation fault while finding next instruction".to_string()),
                    ));
                }
            }
            let parsed_ins = self.decode_instruction();
            if let Err(e) = self.execute_instruction(&parsed_ins) {
                self.err = true;
                self.errmsg = e.only_err().to_string();
                self.running = false;
                self.pmem = true;
                if CONFIG.pretty || CONFIG.verbose {
                    println!("{self}");
                }
                return Err(e);
            }
            if CONFIG.verbose {
                println!("{self}");
            }
            if !self.running {
                break;
            }
            if CONFIG.debug {
                self.record_state();
            }

            let clock = CLOCK.lock().unwrap();

            if self.oflag && self.hlt_on_overflow {
                self.running = false;
            }

            if let Some(v) = self.max_clk {
                if *clock == v as u32 {
                    self.running = false;
                    if CONFIG.verbose {
                        println!("Clock limit reached");
                    }
                }
            }
        }

        if !self.running {
            if CONFIG.verbose {
                println!("╭────────────╮");
                println!("│ {} │", "Halting...".bold().white());
                println!("╰────────────╯");
            }
            self.pmem = true;
            if CONFIG.pretty ^ CONFIG.verbose {
                println!("{self}");
            }
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::mem::drop(clock);
            self.record_state();
        }

        Ok(())
    }

    pub fn execute_instruction(&mut self, ins: &Instruction) -> PossibleCrash {
        self.has_ran = true; // for debugger

        match ins {
            HLT => self.running = false,
            ADD(arg1, arg2) => self.handle_add(arg1, arg2)?,
            JO(arg) => self.handle_jo(arg)?,
            POP(arg) => self.handle_pop(arg)?,
            DIV(arg1, arg2) => self.handle_div(arg1, arg2)?,
            RET => self.handle_ret()?,
            LD(arg1, arg2) => self.handle_ld(arg1, arg2)?,
            ST(arg1, arg2) => self.handle_st(arg1, arg2)?,
            JMP(arg) => self.handle_jmp(arg)?,
            JZ(arg) => self.handle_jz(arg)?,
            CMP(arg1, arg2) => self.handle_cmp(arg1, arg2)?,
            MUL(arg1, arg2) => self.handle_mul(arg1, arg2)?,
            PUSH(arg) => self.handle_push(arg)?,
            INT(arg) => self.handle_int(arg)?,
            MOV(arg1, arg2) => self.handle_mov(arg1, arg2)?,
            NOP => {
                // SAFETY: NOP
                trust_me!("nop");
                self.pc += 1;
            } // NOP
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
                n if n > 3 => return Err(self.generate_invalid_register()),
                n if n < 0 => return Err(self.generate_invalid_register()),
                _ => self.int_reg[*n as usize] = value as i16,
            }
        }
        Ok(())
    }
}
