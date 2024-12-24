use crate::*;
use ahash::RandomState;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub static CPU_STATE: Lazy<Mutex<HashMap<u32, Arc<ModCPU>, RandomState>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

pub static CLOCK: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

const MAX_MEMORY_LIMIT: usize = 128 * 1024 * 1024;

pub struct ModCPU {
    pub int_reg: [i16; 4], // r0 thru r5
    pub uint_reg: [u16; 2],
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: Vec<(u16, i16)>,
    pub pc: u16, // program counter
    pub ir: i16,
    pub running: bool,
    pub zflag: bool,
    pub oflag: bool,
    pub rflag: bool,
    pub sflag: bool,
    pub sp: u16,
    pub bp: u16,
    pub ip: u16,
}

impl ModCPU {
    pub fn modcpu_from_cpu(origin: &CPU) -> ModCPU {
        let memory: Vec<(u16, i16)> = origin
            .memory
            .iter()
            .enumerate()
            .filter_map(|(i, element)| element.map(|value| (i as u16, value)))
            .collect();

        ModCPU {
            int_reg: origin.int_reg,
            uint_reg: origin.uint_reg,
            float_reg: origin.float_reg,
            memory,
            pc: origin.pc,
            ir: origin.ir,
            running: origin.running,
            zflag: origin.zflag,
            oflag: origin.oflag,
            rflag: origin.rflag,
            sflag: origin.sflag,
            sp: origin.sp,
            bp: origin.bp,
            ip: origin.ip,
        }
    }
}

impl CPU {
    pub fn record_state(&self) {
        let mut state = CPU_STATE.lock().unwrap();

        let clock = CLOCK.lock().unwrap();
        while std::mem::size_of_val(&state) * state.len() * 24 > MAX_MEMORY_LIMIT {
            if CONFIG.debug || CONFIG.verbose {
                println!("State records exceeds limit. Removing oldest state.");
            }
            if let Some(key) = state.keys().next().cloned() {
                state.remove(&key);
                return;
            }
        }
        let modified = ModCPU::modcpu_from_cpu(self);
        state.insert(*clock, Arc::new(modified));
    }

    pub fn display_state(clock: &u32) {
        if !CONFIG.verbose && !CONFIG.debug {
            return;
        }
        let state = CPU_STATE.lock().unwrap();
        if let Some(cpu) = state.get(clock) {
            println!("\nCPU State for clock cycle {clock}:");
            println!("  Signed Integer Registers : {:?}", cpu.int_reg);
            println!("  Uint registers           : {:?}", cpu.uint_reg);
            println!("  Float Registers          : {:?}", cpu.float_reg);
            println!("  Program Counter          : {}", cpu.pc);
            println!("  Instruction Register     : {:016b}", cpu.ir);
            println!("  Running                  : {}", cpu.running);
            println!("  Zero flag                : {}", cpu.zflag);
            println!("  Overflow flag            : {}", cpu.oflag);
            println!("  Remainder flag           : {}", cpu.rflag);
            println!("  Sign flag                : {}", cpu.sflag);
            println!("  Stack pointer            : {}", cpu.sp);
            println!("  Base pointer             : {}", cpu.bp);
            println!("  Instruction pointer      : {}", cpu.ip);
            let mut tmp = CPU::new();
            tmp.ir = cpu.ir;
            println!("  Disassembled Instruction : {}", tmp.parse_instruction());
            if let Some((_, n)) = cpu.memory.iter().find(|&&(first, _)| first == cpu.pc) {
                let mut tmp = CPU::new();
                tmp.ir = *n;
                println!("  Next instruction         : {}\n", tmp.parse_instruction());
            }
        } else {
            println!("No CPU state found for clock: {clock}");
        }
    }
}

pub fn display_mem(addr: &usize, clock: &u32) -> Option<i32> {
    let state = CPU_STATE.lock().unwrap();
    if let Some(cpu) = state.get(clock) {
        if let Some((_, v)) = cpu
            .memory
            .iter()
            .find(|&&(first, _)| first == (*addr as u16))
        {
            Some(*v as i32)
        } else {
            eprintln!("Nothing in memory here on this clock cycle\n");
            None
        }
    } else {
        None
    }
}
