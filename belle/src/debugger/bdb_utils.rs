use crate::*;
use colored::*;
use std::io::{self, Write};
impl BDB {
    pub fn handle_set_breakpoint(&mut self, arg: &str) {
        if let Ok(n) = arg.trim().parse::<u16>() {
            self.breakpoints.push(n);
            println!("Breakpoint {n} added.");
        } else {
            eprintln!("'b' requires a numeric argument.");
        }
    }

    pub fn handle_help(arg: &str) {
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

    pub fn handle_run(&mut self) {
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
                    value as i16
                } else {
                    eprintln!("Nothing at PC {}", self.dbgcpu.pc);
                    return;
                };
                let parsed_ins = self.dbgcpu.decode_instruction();
                if let Err(e) = self.dbgcpu.execute_instruction(&parsed_ins) {
                    eprintln!("An error occurred: {e}");
                }
            }
            if self.breakpoints.contains(&self.dbgcpu.pc) {
                println!("Breakpoint {} reached.", self.dbgcpu.pc);
            }
        }
    }

    pub fn handle_set_pc(&mut self, arg: &str) {
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

    pub fn handle_print_memory(&mut self, arg: &str) {
        if let Ok(n) = arg.parse::<usize>() {
            if let Some(memvalue) = self.dbgcpu.memory[n] {
                println!("Value in memory: {memvalue:016b} ({memvalue})");
                let oldvalue = self.dbgcpu.ir;
                self.dbgcpu.ir = memvalue as i16;
                println!("Dumped instruction: {}", self.dbgcpu.decode_instruction());
                self.dbgcpu.ir = oldvalue;
            } else {
                println!("{}", "Nothing in memory here.".yellow());
            }
        } else {
            eprintln!("'p' requires a numeric argument.");
        }
    }

    pub fn handle_info(&mut self, arg: &str) {
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

    pub fn handle_where_begins(&self) {
        if self.dbgcpu.memory.iter().all(|&x| x.is_none()) {
            eprintln!("{}", "CPU memory is empty. Load the program first.".red());
        } else {
            println!(
                "Execution begins at memory address {}",
                self.dbgcpu.starts_at
            );
        }
    }

    pub fn handle_execute(&mut self) {
        self.dbgcpu.ir = if let Some(value) = self.dbgcpu.memory[self.dbgcpu.pc as usize] {
            value as i16
        } else {
            eprintln!("Nothing at PC {}", self.dbgcpu.pc);
            return;
        };

        let parsed_ins = self.dbgcpu.decode_instruction();
        if let Err(e) = self.dbgcpu.execute_instruction(&parsed_ins) {
            eprintln!("An error occurred: {e}");
        }

        self.dbgcpu.record_state();
        self.dbgcpu.pmem = false;
        self.print_cpu_state();
        self.dbgcpu.pmem = true;
    }

    pub fn handle_print_all_memory(&mut self) {
        self.dbgcpu.pmem();
    }

    pub fn handle_print_cpu_state(&mut self) {
        self.print_cpu_state();
    }

    pub fn print_cpu_state(&mut self) {
        println!("{}", self.dbgcpu);

        if let Some(n) = self.dbgcpu.memory[self.dbgcpu.pc as usize] {
            self.dbgcpu.ir = n as i16;
            println!(
                "Next instruction: {}",
                self.dbgcpu.decode_instruction()
            );
        }
    }

    pub fn handle_poke(&mut self, arg: &str) {
        if let Ok(n) = arg.parse::<usize>() {
            if let Some(memvalue) = self.dbgcpu.memory[n] {
                println!("Value in memory: {memvalue:016b} ({memvalue})");
                let oldvalue = self.dbgcpu.ir;
                self.dbgcpu.ir = memvalue as i16;
                println!("{}", self.dbgcpu.decode_instruction());
                self.dbgcpu.ir = oldvalue;
            } else {
                println!("This memory address is empty.\n");
            }
            println!("Enter a new value");
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
                if let Ok(val) = u32::from_str_radix(&buffer.trim()[2..], 2) {
                    println!("Value in memory address {n} set to {val:016b}");
                    self.dbgcpu.memory[n] = Some(val as u16);
                } else {
                    println!("Input could not be parsed to binary");
                }
            } else if let Ok(v) = buffer.trim().parse::<u16>() {
                println!("Value in memory address {n} set to {v}");
                self.dbgcpu.memory[n] = Some(v);
            } else {
                println!("Could not parse a valid integer from input.");
            }
        } else {
            eprintln!("'pk' requires a numeric argument.");
        }
    }
    pub fn handle_remove_breakpoint(&mut self, arg: &str) {
        if let Ok(n) = arg.trim().parse::<u16>() {
            self.breakpoints.retain(|&x| x != n);
            println!("Breakpoint {n} removed.");
        } else {
            eprintln!("'br' requires a numeric argument.");
        }
    }
}
