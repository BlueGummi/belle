use crate::CPU;
use colored::Colorize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::vec::Vec;
pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}
pub struct BDB {
    pub dbgcpu: CPU,
    pub clock: u32,
    pub breakpoints: Vec<u16>,
    pub exe: String,
}

impl BDB {
    pub fn new(executable_path: &str) -> io::Result<Self> {
        let bin = bin_to_vec(executable_path)?;
        let mut dbgcpu = CPU::new();
        dbgcpu.load_binary(&bin);
        Ok(Self {
            dbgcpu,
            clock: 0,
            exe: executable_path.to_string(),
            breakpoints: Vec::new(),
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let prompt = "(bdb)> ".green();
        println!("Welcome to the BELLE-debugger!");
        println!("First time? Type 'h' or 'help'");

        loop {
            let _ = ctrlc::set_handler(move || {
                println!("\nExiting...");
                std::process::exit(0);
            });
            print!("\n{prompt}");
            let mut input = String::new();
            io::stdout().flush()?;

            io::stdin().read_line(&mut input)?;
            let command = input.trim();

            if command.is_empty() {
                continue;
            }

            let (cmd, arg) = Self::parse_command(command);
            match cmd.to_lowercase().as_str() {
                "q" | "quit" | ":q" => {
                    println!("{}", "Exiting...\n".yellow());
                    return Ok(());
                }
                "h" | "help" => Self::handle_help(arg),
                "l" => self.dbgcpu.load_binary(&bin_to_vec(&self.exe)?),
                "r" | "run" => self.handle_run(),
                "spc" => self.handle_set_pc(arg),
                "p" | "pmem" => self.handle_print_memory(arg),
                "i" | "info" => self.handle_info(arg),
                "wb" => self.handle_where_begins(),
                "e" | "exc" => self.handle_execute(),
                "a" => self.handle_print_all_memory(),
                "w" => self.handle_print_cpu_state(),
                "cls" | "clear" => self.cls(),
                "pk" => self.handle_set_memory_value(arg),
                "b" => self.handle_set_breakpoint(arg),
                "br" => self.handle_remove_breakpoint(arg),
                "ba" => {
                    println!("Breakpoints cleared.");
                    self.breakpoints.clear();
                }
                "rs" => self.reset_cpu(),
                _ => Self::unknown_command(command),
            }
        }
    }

    fn parse_command(input: &str) -> (&str, &str) {
        let mut parts = input.splitn(2, ' ');
        (parts.next().unwrap(), parts.next().unwrap_or(""))
    }

    fn cls(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn reset_cpu(&mut self) {
        self.dbgcpu = CPU::new();
        println!("CPU reset.");
    }

    fn unknown_command(command: &str) {
        println!("Unknown command: '{command}'");
        println!("Type 'h' or 'help' for a list of available commands.");
    }
}

pub fn bin_to_vec(file_path: &str) -> io::Result<Vec<i16>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut result: Vec<i16> = Vec::new();

    for chunk in buffer.chunks(2) {
        if chunk.len() == 2 {
            let value = i16::from_be_bytes([chunk[0], chunk[1]]);
            result.push(value);
        }
    }

    Ok(result)
}
