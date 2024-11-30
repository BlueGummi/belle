use crate::Argument::*;
use crate::Instruction::*;
use crate::*;
use colored::*;
use std::vec::Vec;
pub const MEMORY_SIZE: usize = 65536;
pub const SR_LOC: usize = 50000;
#[derive(Clone)]
pub struct CPU {
    pub int_reg: [i16; 6],   // r0 thru r5
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: [Option<i16>; MEMORY_SIZE],
    pub pc: u16, // program counter
    pub ir: i16,
    pub jloc: u16, // location from which a jump was performed
    pub starts_at: u16,
    pub running: bool,
    pub zflag: bool,
    pub oflag: bool,
    pub rflag: bool,
    pub hlt_on_overflow: bool,
}
impl Default for CPU {
    fn default() -> CPU {
        CPU::new()
    }
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            int_reg: [0; 6],
            float_reg: [0.0; 2],
            memory: [None; MEMORY_SIZE],
            pc: 0,
            ir: 0,
            jloc: 0,
            starts_at: 0,
            running: false,
            zflag: false,
            oflag: false,
            rflag: false,
            hlt_on_overflow: false,
        }
    }
    pub fn load_binary(&mut self, binary: Vec<i16>) {
        let mut in_subr = false;
        let mut counter = 0;
        let mut start_found = false;
        let mut sr_counter = 0;
        let mut subr_loc = SR_LOC;
        for element in binary {
            if in_subr && (element >> 12) != RET_OP {
                self.memory[subr_loc + sr_counter] = Some(element);
                sr_counter += 1;
                continue;
            } else if (element >> 12) == RET_OP {
                in_subr = false;
                self.memory[counter + subr_loc] = Some(element);
                sr_counter = 0;
                subr_loc += 100;
            }
            if (element >> 9) == 1 {
                if start_found {
                    EmuError::Duplicate(".start directives".to_string()).err();
                }
                self.starts_at = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".start directive found.");
                }
                start_found = true;
                self.shift_memory();
                if CONFIG.verbose {
                    println!("program starts at {}", self.starts_at);
                }
                continue;
            }
            if (element >> 12) & 0b0000000000001111u16 as i16 != 0b1111 {
                self.memory[counter + self.starts_at as usize] = Some(element);
                if CONFIG.verbose {
                    println!("Element {:016b} loaded into memory", element);
                }
            } else {
                self.memory[subr_loc + sr_counter] = Some(element);
                in_subr = true;
                sr_counter += 1;
                continue;
            }
            counter += 1;
        }
    }
    #[allow(unused_comparisons)]
    fn shift_memory(&mut self) {
        let mut some_count = 0;
        if CONFIG.verbose {
            println!("Shifting memory...");
        }
        for element in self.memory {
            if element.is_some() {
                some_count += 1;
            }
        }
        // check for overflow
        if some_count as u32 + self.starts_at as u32 > 65535 {
            // unused comparison
            EmuError::MemoryOverflow().err();
        }
        let mem_copy = self.memory;
        self.memory = [None; 65536];
        for i in 0..=65535 {
            if mem_copy[i as usize].is_some() {
                self.memory[(i + self.starts_at) as usize] = mem_copy[i as usize];
            }
        }
        self.pc = self.starts_at;
        if CONFIG.verbose {
            println!("Shift completed.");
        }
    }
    pub fn run(&mut self) {
        self.running = true;
        if CONFIG.verbose {
            println!("  Starts At MemAddr: {}", self.starts_at);
        }
        while self.running {
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::thread::sleep(std::time::Duration::from_millis(
                CONFIG.time_delay.unwrap().into(),
            ));
            std::mem::drop(clock); // clock must go bye bye so it unlocks
            if self.memory[self.pc as usize].is_none() {
                if CONFIG.verbose {
                    println!("pc: {}", self.pc);
                }
                UnrecoverableError::SegmentationFault(
                    self.pc,
                    Some("Segmentation fault while finding next instruction".to_string()),
                )
                .err();
                if !CONFIG.quiet {
                    println!("Attempting to recover by restarting...")
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
                self.pc = self.starts_at;
            }
            self.ir = self.memory[self.pc as usize].unwrap();
            let parsed_ins = self.parse_instruction();
            self.execute_instruction(&parsed_ins);
            self.record_state();
            let clock = CLOCK.lock().unwrap();
            cpu::CPU::display_state(*clock);
            if self.oflag {
                RecoverableError::Overflow(self.pc, Some("Overflowed a register.".to_string()))
                    .err();
                if self.hlt_on_overflow {
                    self.running = false;
                }
            }
        }
        if !self.running {
            if !CONFIG.quiet {
                println!("Halting...");
            }
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::mem::drop(clock);
            self.record_state();
            let clock = CLOCK.lock().unwrap(); // might panic
            cpu::CPU::display_state(*clock);
            std::process::exit(0);
        }
    }
}
// we need a function to load instructions into RAM
// we also need interrupts for pseudo-instructions
//
// debug messages would be nice too
