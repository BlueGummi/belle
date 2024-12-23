use crate::CPU;
use colored::Colorize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::vec::Vec;
pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}
pub struct BDB {
    dbgcpu: CPU,
    clock: u32,
    breakpoints: Vec<u16>,
    exe: String,
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
                "rs" => self.reset_cpu(),
                _ => Self::unknown_command(command),
            }
        }
    }

    fn parse_command(input: &str) -> (&str, &str) {
        let mut parts = input.splitn(2, ' ');
        (parts.next().unwrap(), parts.next().unwrap_or(""))
    }

    fn handle_set_breakpoint(&mut self, arg: &str) {
        if let Ok(n) = arg.trim().parse::<u16>() {
            self.breakpoints.push(n);
            println!("Breakpoint {n} added.");
        } else {
            eprintln!("'b' requires a numeric argument.");
        }
    }

    fn handle_help(arg: &str) {
        if arg.is_empty() {
            let commands = vec![
                ("q", "Exit BDB"),
                ("h", "Print help on BDB or a specific command"),
                ("l", "Load program"),
                ("r", "Run program"),
                ("rs", "Reset emulator"),
                ("cls", "Clear screen"),
                ("spc", "Set program counter to a given value"),
                ("p", "Print value in memory"),
                ("pk", "Set a new value for a location in memory"),
                ("a", "Print all memory"),
                ("wb", "Print CPU's starting memory address"),
                ("e", "Execute instruction"),
                ("w", "View the state of the CPU"),
                ("i", "Print CPU state at debugger's clock"),
                ("b", "Set a breakpoint"),
                ("br", "Remove a breakpoint"),
                ("ba", "Remove all breakpoints"),
                (
                    "im",
                    "Print a value in memory at the clock after the CPU has run",
                ),
            ];

            println!("{}", "Available commands:".blue());
            let max_length = commands.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0);

            for (cmd, desc) in commands {
                let padding = " ".repeat(max_length - cmd.len() + 4);
                println!("{}{} - {}", cmd.yellow(), padding, desc);
            }
        } else {
            match arg.trim().to_lowercase().as_str() {
                "q" => println!("'quit' takes no arguments. Exits the debugger."),
                "h" => println!("'help' takes zero or one argument. Prints command information."),
                "l" => println!("'load' takes no arguments. Loads the CPU's memory with the program."),
                "r" => println!("'run' takes no arguments. Executes the CPU with the loaded data."),
                "spc" => println!("'set program counter' takes one argument to set the CPU's program counter."),
                "p" | "pmem" => println!("'print memory' takes one argument. Prints the value at the specified memory address."),
                "e" => println!("'execute' takes no arguments. Executes the instruction at the current program counter."),
                "i" => println!("'info' takes one argument. Prints the CPU state at the specified clock cycle."),
                "cls" => println!("'clear' takes no arguments. Resets the cursor to the top left of the terminal."),
                "wb" => println!("'where begins' takes no arguments. Prints the starting memory address of the CPU."),
                "a" => println!("'all instructions' takes no arguments. Prints all memory as instructions."),
                "w" => println!("'w' takes no arguments. Prints the current state of the CPU."),
                "pk" => println!("'pk' takes one argument. Sets a new value for a memory location."),
                "im" => println!("'info memory' takes one argument. Prints the value in memory after the CPU has run."),
                "rs" => println!("'reset' takes no arguments. Resets all parts of the emulator."),
                "b" => println!("'breakpoint' takes one argument. Sets a breakpoint at a specified memory address."),
                "br" => println!("'breakpoint remove' takes one argument. Removes a specified breakpoint."),
                "ba" => println!("'breakpoint remove all' takes no arguments. Removes all breakpoints."),
                _ => println!("Unknown command: '{arg}'. Type 'h' or 'help' for a list of commands."),
            }
        }
    }

    fn handle_run(&mut self) {
        if self.dbgcpu.memory.iter().all(|&x| x.is_none()) {
            eprintln!("{}", "CPU memory is empty. Load the program first.".red());
            return;
        }
        if self.breakpoints.is_empty() {
            if let Err(e) = self.dbgcpu.run() {
                eprintln!("{e}");
                eprintln!("Consider resetting the CPU with 'rs'.");
            }
        } else {
            self.dbgcpu.has_ran = true;
            while !self.breakpoints.contains(&self.dbgcpu.pc) {
                self.dbgcpu.ir = if let Some(value) = self.dbgcpu.memory[self.dbgcpu.pc as usize] {
                    value
                } else {
                    eprintln!("Nothing at PC {}", self.dbgcpu.pc);
                    return;
                };
                let parsed_ins = self.dbgcpu.parse_instruction();
                if let Err(e) = self.dbgcpu.execute_instruction(&parsed_ins) {
                    eprintln!("An error occurred: {e}");
                }
            }
            if self.breakpoints.contains(&self.dbgcpu.pc) {
                println!("Breakpoint {} reached.", self.dbgcpu.pc);
            }
        }
    }

    fn handle_set_pc(&mut self, arg: &str) {
        if self.dbgcpu.memory.iter().all(|&x| x.is_none()) {
            eprintln!("{}", "CPU memory is empty. Load the program first.".red());
            return;
        }
        if let Ok(n) = arg.trim().parse::<u16>() {
            self.dbgcpu.pc = n;
            println!("Program counter set to {n}.");
        } else {
            eprintln!("'spc' requires a numeric argument.");
        }
    }

    fn handle_print_memory(&mut self, arg: &str) {
        if let Ok(n) = arg.parse::<usize>() {
            if let Some(memvalue) = self.dbgcpu.memory[n] {
                println!("Value in memory: {memvalue:016b} ({memvalue})");
                let oldvalue = self.dbgcpu.ir;
                self.dbgcpu.ir = memvalue;
                println!("Dumped instruction: {}", self.dbgcpu.parse_instruction());
                self.dbgcpu.ir = oldvalue;
            } else {
                println!("{}", "Nothing in memory here.".yellow());
            }
        } else {
            eprintln!("'p' requires a numeric argument.");
        }
    }

    fn handle_info(&mut self, arg: &str) {
        if !self.dbgcpu.has_ran {
            eprintln!("{}", "CPU has not run.".red());
            return;
        }
        match arg.parse::<u32>() {
            Ok(n) => {
                self.clock = n;
                CPU::display_state(&self.clock);
            }
            Err(_) => eprintln!("Error parsing second argument."),
        }
    }

    fn handle_where_begins(&self) {
        if self.dbgcpu.memory.iter().all(|&x| x.is_none()) {
            eprintln!("{}", "CPU memory is empty. Load the program first.".red());
        } else {
            println!(
                "Execution begins at memory address {}",
                self.dbgcpu.starts_at
            );
        }
    }

    fn handle_execute(&mut self) {
        self.dbgcpu.ir = if let Some(value) = self.dbgcpu.memory[self.dbgcpu.pc as usize] {
            value
        } else {
            eprintln!("Nothing at PC {}", self.dbgcpu.pc);
            return;
        };

        let parsed_ins = self.dbgcpu.parse_instruction();
        if let Err(e) = self.dbgcpu.execute_instruction(&parsed_ins) {
            eprintln!("An error occurred: {e}");
        }

        self.dbgcpu.record_state();
        self.print_cpu_state();
    }

    fn handle_print_all_memory(&mut self) {
        for (index, element) in self.dbgcpu.memory.iter().enumerate() {
            if let Some(value) = element {
                self.dbgcpu.ir = *value;
                println!("Value at {} is {}", index, self.dbgcpu.parse_instruction());
            }
        }
    }

    fn handle_print_cpu_state(&mut self) {
        self.print_cpu_state();
    }

    fn print_cpu_state(&mut self) {
        println!("  Signed Integer Registers : {:?}", self.dbgcpu.int_reg);
        println!("  Uint registers           : {:?}", self.dbgcpu.uint_reg);
        println!("  Float Registers          : {:?}", self.dbgcpu.float_reg);
        println!("  Program Counter          : {}", self.dbgcpu.pc);
        println!("  Instruction Register     : {:016b}", self.dbgcpu.ir);
        println!("  Running                  : {}", self.dbgcpu.running);
        println!("  Zero flag                : {}", self.dbgcpu.zflag);
        println!("  Overflow flag            : {}", self.dbgcpu.oflag);
        println!("  Remainder flag           : {}", self.dbgcpu.rflag);
        println!("  Sign flag                : {}", self.dbgcpu.sflag);
        println!("  Stack pointer            : {}", self.dbgcpu.sp);
        println!("  Base pointer             : {}", self.dbgcpu.bp);
        println!("  Instruction pointer      : {}", self.dbgcpu.ip);
        println!(
            "  Disassembled Instruction : {}",
            self.dbgcpu.parse_instruction()
        );

        if let Some(n) = self.dbgcpu.memory[self.dbgcpu.pc as usize] {
            self.dbgcpu.ir = n;
            println!(
                "  Next instruction         : {}",
                self.dbgcpu.parse_instruction()
            );
        }
    }

    fn handle_set_memory_value(&mut self, arg: &str) {
        if let Ok(n) = arg.parse::<usize>() {
            if let Some(memvalue) = self.dbgcpu.memory[n] {
                println!("Value in memory: {memvalue:016b} ({memvalue})");
                let oldvalue = self.dbgcpu.ir;
                self.dbgcpu.ir = memvalue;
                println!("{}", self.dbgcpu.parse_instruction());
                self.dbgcpu.ir = oldvalue;

                let mut buffer = String::new();
                io::stdout().flush().unwrap();
                if let Err(e) = io::stdin().read_line(&mut buffer) {
                    println!("{e}");
                }

                if buffer.is_empty() {
                    println!("Empty input");
                    return;
                }

                if buffer.trim().starts_with("0b") {
                    if let Ok(val) = i32::from_str_radix(&buffer.trim()[2..], 2) {
                        println!("Value in memory address {n} set to {val:016b}");
                        self.dbgcpu.memory[n] = Some(val as i16);
                    } else {
                        println!("Input could not be parsed to binary");
                    }
                } else if let Ok(v) = buffer.trim().parse::<i16>() {
                    println!("Value in memory address {n} set to {v}");
                    self.dbgcpu.memory[n] = Some(v);
                } else {
                    println!("Could not parse a valid integer from input.");
                }
            } else {
                println!("{}", "Nothing in memory here.".yellow());
            }
        } else {
            eprintln!("'pk' requires a numeric argument.");
        }
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
