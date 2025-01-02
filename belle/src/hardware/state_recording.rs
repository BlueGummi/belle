use crate::{config::CONFIG, *};
use ahash::RandomState;
use colored::*;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

pub static CPU_STATE: Lazy<Mutex<HashMap<u32, Arc<ModCPU>, RandomState>>> =
    Lazy::new(|| Mutex::new(HashMap::default()));

pub static CLOCK: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

const MAX_MEMORY_LIMIT: usize = 128 * 1024 * 1024;

pub struct ModCPU {
    pub int_reg: [i16; 4], // r0 thru r5
    pub uint_reg: [u16; 2],
    pub float_reg: [f32; 2], // r6 and r7
    pub memory: Vec<(u16, u16)>,
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
        let memory: Vec<(u16, u16)> = origin
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
impl fmt::Display for ModCPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            " {}",
            if self.running {
                "RUNNING".green()
            } else {
                "HALTED".red()
            }
        )?;

        let mut register_lines = Vec::new();

        for (i, &val) in self.int_reg.iter().enumerate() {
            register_lines.push(format!(
                "| {}: {:^6}",
                format!("r{}", i).bold().blue(),
                val.to_string().bold()
            ));
        }
        for (i, &val) in self.uint_reg.iter().enumerate() {
            register_lines.push(format!(
                "| {}: {:^6}",
                format!("r{}", i + 4).bold().cyan(),
                val.to_string().bold()
            ));
        }
        for (i, &val) in self.float_reg.iter().enumerate() {
            register_lines.push(format!(
                "| {}: {:^6.2}",
                format!("r{}", i + 6).bold().magenta(),
                val.to_string().bold()
            ));
        }
        writeln!(f, " {} |", register_lines.join(" "))?;

        write!(f, " | {}:", "pc".yellow())?;
        write!(f, " {:^6} |", self.pc)?;
        write!(f, " {}:", "ir".yellow())?;
        write!(f, " {:016b}    |", self.ir)?;
        write!(f, " {}:", "sp".yellow())?;
        write!(f, " {:^6} |", self.sp)?;
        write!(f, " {}:", "bp".yellow())?;
        write!(f, " {:^6} |", self.bp)?;
        write!(f, " {}:", "ip".yellow())?;
        writeln!(f, " {:^6} |", self.ip)?;

        write!(
            f,
            " | {}: {} ",
            "zf".bright_green().bold(),
            if self.zflag {
                " set  ".green()
            } else {
                "unset ".red()
            }
        )?;
        write!(
            f,
            "| {}: {} ",
            "of".bright_red().bold(),
            if self.oflag {
                " set  ".green()
            } else {
                "unset ".red()
            }
        )?;
        write!(
            f,
            "| {}: {} ",
            "rf".bright_white().bold(),
            if self.rflag {
                " set  ".green()
            } else {
                "unset ".red()
            }
        )?;
        writeln!(
            f,
            "| {}: {} |",
            "sf".bright_purple().bold(),
            if self.sflag {
                " set  ".green()
            } else {
                "unset ".red()
            }
        )?;

        writeln!(f, "{}", " MEMORY".bright_purple().bold())?;
        for (index, &(_, value)) in self.memory.iter().enumerate() {
            let displayed = format!(
                "Value at {} decodes to {}",
                index.to_string().magenta(),
                value.to_string().green()
            );
            write!(f, "{displayed}")?;
            for _ in displayed.len()..38 {
                write!(f, " ")?;
            }
            writeln!(
                f,
                " - {} ({})",
                format!("{:016b}", value).bright_white(),
                value.to_string().bright_green()
            )?;
        }

        Ok(())
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
            println!("{cpu}");
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
