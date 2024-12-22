use crate::*;
use std::thread;
use std::time::Duration;
use std::vec::Vec;
pub const MEMORY_SIZE: usize = 65535;

#[derive(Clone)]
pub struct CPU {
    pub int_reg: [i16; 4], // r0 thru r5
    pub uint_reg: [u16; 2],
    pub float_reg: [f32; 2],                     // r6 and r7
    pub memory: Box<[Option<i16>; MEMORY_SIZE]>, // Use Box to allocate the array on the heap
    pub pc: u16,                                 // program counter
    pub ir: i16,
    pub starts_at: u16,
    pub running: bool,
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
        }
    }

    pub fn load_binary(&mut self, binary: &Vec<i16>) {
        let mut counter = 0;
        let mut start_found = false;

        for element in binary {
            if (element >> 9) == 1 {
                // start directive
                if start_found {
                    EmuError::Duplicate(".start directives".to_string()).err();
                    self.do_not_run = true;
                }
                self.starts_at = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".start directive found.");
                }
                start_found = true;
                if CONFIG.verbose {
                    println!("program starts at {}", self.starts_at);
                }
                continue;
            } else if (element >> 9) == 2 {
                self.sp = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".ssp directive found");
                }
                continue;
            } else if (element >> 9) == 3 {
                self.bp = (element & 0b111111111) as u16;
                if CONFIG.verbose {
                    println!(".sbp directive found");
                }
                continue;
            }
            self.memory[counter + self.starts_at as usize] = Some(*element);
            if CONFIG.verbose {
                println!("Element {element:016b} loaded into memory");
            }

            counter += 1;
        }
        self.shift_memory();
        self.pc = self.starts_at;
    }

    fn shift_memory(&mut self) {
        if let Some(first_val) = self.memory.iter().position(|&e| e.is_some()) {
            if self.pc == first_val as u16 {
                return;
            }
        }

        if CONFIG.verbose {
            println!("Shifting memory...");
        }

        let some_count = self.memory.iter().filter(|&&e| e.is_some()).count();

        if some_count as u32 + u32::from(self.starts_at) > MEMORY_SIZE.try_into().unwrap() {
            EmuError::MemoryOverflow().err();
        }

        let mut new_memory = Box::new([None; MEMORY_SIZE]);

        let first_some_index = self.memory.iter().position(|&e| e.is_some()).unwrap_or(0);
        for (i, value) in self.memory.iter().enumerate() {
            if let Some(val) = value {
                let new_index = (self.starts_at + (i - first_some_index) as u16) as usize;
                new_memory[new_index] = Some(*val);
            }
        }

        std::mem::swap(&mut self.memory, &mut new_memory);
        self.pc = self.starts_at;

        if CONFIG.verbose {
            println!("Shift completed.");
        }
    }

    pub fn run(&mut self) -> Result<(), UnrecoverableError> {
        self.has_ran = true; // for debugger
        self.running = true;
        if self.do_not_run {
            return Ok(());
        }
        if CONFIG.verbose {
            println!("  Starts At MemAddr: {}", self.starts_at);
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
            thread::sleep(Duration::from_millis(CONFIG.time_delay.unwrap().into()));
            std::mem::drop(clock); // clock must go bye bye so it unlocks

            // Check for segmentation fault
            match self.memory[self.pc as usize] {
                Some(instruction) => {
                    self.ip = self.pc;
                    self.ir = instruction;
                }
                None => {
                    if CONFIG.verbose {
                        println!("PC: {}", self.pc);
                    }
                    return Err(UnrecoverableError::SegmentationFault(
                        self.ir,
                        self.pc,
                        Some("Segmentation fault while finding next instruction".to_string()),
                    ));
                }
            }
            let parsed_ins = self.parse_instruction();
            if let Err(e) = self.execute_instruction(&parsed_ins) {
                self.running = false;
                return Err(e);
            }

            if CONFIG.debug || CONFIG.verbose {
                self.record_state();
            }

            let clock = CLOCK.lock().unwrap();
            if CONFIG.verbose {
                cpu::CPU::display_state(&clock);
            }

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
                println!("Halting...");
            }
            let mut clock = CLOCK.lock().unwrap(); // might panic
            *clock += 1;
            std::mem::drop(clock);
            self.record_state();

            let clock = CLOCK.lock().unwrap(); // might panic
            if CONFIG.verbose {
                cpu::CPU::display_state(&clock);
            }

            if CONFIG.pretty {
                for i in 0..=3 {
                    println!(
                        "Register {}: {}, {:016b}, {:04x}",
                        i, self.int_reg[i], self.int_reg[i], self.int_reg[i]
                    );
                }
                for i in 0..=1 {
                    println!("Uint Register {}: {}", i, self.uint_reg[i]);
                }
                for i in 0..=1 {
                    println!("Float Register {}: {}", i, self.float_reg[i]);
                }
            }
        }

        Ok(())
    }
}
