use crate::{CONFIG, CPU};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub static CPU_STATE: Lazy<Mutex<HashMap<u32, Arc<CPU>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static CLOCK: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
const MEMORY_LIMIT: usize = 1024 * 1024 * 5;
impl CPU {
    pub fn record_state(&self) {
        let mut state = CPU_STATE.lock().unwrap();
        let clock = CLOCK.lock().unwrap();
        if state.len() * std::mem::size_of::<(u32, Arc<CPU>)>() > MEMORY_LIMIT {
            println!("Memory limit exceeded, skipping state recording.");
            std::process::exit(1);
        }

        state.insert(*clock, Arc::new(self.clone()));
    }

    pub fn display_state(clock: u32) {
        if !CONFIG.verbose && !CONFIG.debug {
            return;
        }
        let state = CPU_STATE.lock().unwrap();
        if let Some(cpu) = state.get(&clock) {
            println!("\nCPU State for clock cycle {clock}:");
            println!("  Integer Registers        : {:?}", cpu.int_reg);
            println!("  Float Registers          : {:?}", cpu.float_reg);
            println!("  Program Counter          : {}", cpu.pc);
            println!("  Instruction Register     : {:016b}", cpu.ir);
            println!("  Running                  : {}", cpu.running);
            println!("  Zero flag                : {}", cpu.zflag);
            println!("  Overflow flag            : {}", cpu.oflag);
            println!("  Remainder flag           : {}", cpu.rflag);
            println!("  Stack pointer            : {}", cpu.sp);
            println!("  Base pointer             : {}", cpu.bp);
            println!("  Disassembled Instruction : \n{}", cpu.parse_instruction());
        } else {
            println!("No CPU state found for clock: {clock}");
        }
    }
}