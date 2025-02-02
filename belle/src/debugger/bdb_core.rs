use crate::{CPU, *};
use colored::Colorize;
use std::{
    io::{self, Write},
    vec::Vec,
};
pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}
pub struct BDB {
    pub dbgcpu: CPU,
    pub breakpoints: Vec<u16>,
    pub exe: String,
}

impl BDB {
    pub fn new(executable_path: &str) -> io::Result<Self> {
        let mut dbgcpu = CPU::new();
        dbgcpu.debugging = true;
        Ok(Self {
            dbgcpu,
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
                    std::process::exit(0);
                }
                "h" | "help" => Self::handle_help(arg),
                "l" => {
                    if let Err(e) = self.dbgcpu.load_rom(&create_rom(&self.exe)?) {
                        eprintln!("{e}");
                        return Ok(());
                    }
                }
                "r" | "run" => self.handle_run(),
                "spc" => self.handle_set_pc(arg),
                "p" | "pmem" => self.handle_print_memory(arg),
                "wb" => self.handle_where_begins(),
                "e" | "exc" => self.handle_execute(),
                "a" => self.handle_print_all_memory(),
                "w" => self.handle_print_cpu_state(),
                "cls" | "clear" => self.cls(),
                "pk" => self.handle_poke(arg),
                "b" => self.handle_set_breakpoint(arg),
                "br" => self.handle_remove_breakpoint(arg),
                "bp" => self.handle_print_all_breakpoints(),
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
        self.dbgcpu.debugging = true;
        println!("CPU reset.");
    }

    fn unknown_command(command: &str) {
        println!("Unknown command: '{command}'");
        println!("Type 'h' or 'help' for a list of available commands.");
    }
}
