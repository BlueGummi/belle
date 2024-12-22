use crate::display_mem;
use crate::CPU;
use colored::Colorize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::vec::Vec;
pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}
pub fn run_bdb(executable_path: &str) -> io::Result<()> {
    let prompt = "(bdb)> ".green();
    let mut dbgcpu = CPU::new();
    let mut clock = 0;
    println!("Welcome to the BELLE-debugger!");
    println!("First time? Type 'h' or 'help'\n");
    loop {
        let _ = ctrlc::set_handler(move || {
            println!("\nExiting...");
            std::process::exit(0);
        });
        let bin = bin_to_vec(executable_path)?;
        print!("{prompt}");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim().to_string();
        if command.is_empty() {
            continue;
        }
        let mut parts = command.splitn(2, ' ');

        let (cmd, arg) = (parts.next().unwrap(), parts.next().unwrap_or(""));
        match cmd.to_lowercase().as_str() {
            "q" | "quit" | ":q" => {
                println!("{}", "Exiting...\n".yellow());
                return Ok(());
            }
            "h" | "help" => {
                if arg.is_empty() {
                    println!("{}", "Available commands:".blue());
                    println!("q | quit | :q - Exit BDB");
                    println!("h | help      - Print help on BDB or a specific command");
                    println!("l | load      - Load program");
                    println!("r | run       - Run program");
                    println!("rs            - Reset emulator");
                    println!("cls           - Clear screen\n");

                    println!("Can be used whether or not the CPU has ran:");
                    println!("spc           - Set program counter to a given value");
                    println!("p | pmem      - Print value in memory");
                    println!("pk            - Set a new value for a location in memory");
                    println!("a             - Print all memory");
                    println!("wb            - Print CPU's starting memory address\n");

                    println!("Used to step through the program:");
                    println!("e | exc       - Execute instruction");
                    println!("w             - View the state of the CPU\n");

                    println!("Can only be used after the CPU has ran");
                    println!("i | info      - Print CPU state at debugger's clock");
                    println!("im            - Print a value in memory at the clock after the CPU has run\n");
                } else {
                    match arg.trim().to_lowercase().as_str() {
                        "q" | "quit" | ":q" => {
                            println!("'quit' takes no arguments.");
                            println!("'quit' exits the BELLE-debugger with code 0.");
                            println!(
                                "The CPU will stop execution or will exit if 'quit' is called.\n"
                            );
                        }
                        "h" | "help" => {
                            println!("'help' takes zero or one argument.");
                            println!("'help' prints out information on how to use commands.\n");
                        }
                        "l" | "load" => {
                            println!("'load' takes no arguments.");
                            println!("'load' loads the CPU's memory with the program, and does nothing afterwards.");
                            println!("'load' can be used to verify that the program is loaded correctly.");
                            println!(
                                "'p' | 'pmem' can be used after 'load' to view the CPU's memory\n"
                            );
                        }
                        "r" | "run" => {
                            println!("'run' takes no arguments");
                            println!(
                                "'run' executes the CPU with the data stored in its memory.\n"
                            );
                        }
                        "spc" => {
                            println!("'set program counter' takes one argument.");
                            println!("'spc' sets the CPU's program counter to a given number\n");
                        }
                        "p" | "pmem" => {
                            println!("'print memory' takes one argument.");
                            println!("'pmem' prints the value at the specified memory address.");
                            println!("If nothing is there, it will say so.\n");
                        }
                        "e" | "exc" => {
                            println!("'execute' takes no arguments");
                            println!("'e' executes the instruction at the current memory address (program counter)\n");
                        }
                        "i" | "info" => {
                            println!("'info' takes one argument");
                            println!("'info' prints the current state of the CPU on the specified clock cycle.\n");
                        }
                        "cls" | "clear" => {
                            println!("'clear' takes no arguments and resets the cursor to the top left\nof the terminal\n");
                        }
                        "wb" => {
                            println!("'where begins' takes no arguments");
                            println!("'wb' prints the starting memory address of the CPU\n");
                        }
                        "a" => {
                            println!("'all instructions' takes no arguments");
                            println!("'a' prints everything in memory as an instruction if it is a value\n");
                        }
                        "w" => {
                            println!("'w' takes no arguments");
                            println!("'w' prints the state of the CPU as-is\n");
                        }
                        "pk" => {
                            println!("'pk' takes one argument");
                            println!("'pk' will print the value in memory and ask for a new value");
                            println!("binary values, prefixed with 0b, are accepted");
                            println!("if an invalid value is entered or nothing is entered, it will not do anything\n");
                        }
                        "im" => {
                            println!("'info memory' takes one argument");
                            println!("'im' will print the value in memory at the clock cycle after the CPU has ran");
                            println!(
                                "if an invalid value or nothing is entered, nothing will happen\n"
                            );
                        }
                        "rs" => {
                            println!("'reset' takes no arguments");
                            println!("'reset' resets all parts of the emulator");
                        }
                        _ => {
                            println!("Unknown command: '{arg}'");
                            println!("Type 'h' or 'help' for a list of available commands.\n");
                        }
                    }
                }
            }
            "l" | "load" => dbgcpu.load_binary(&bin),
            "r" | "run" => {
                if dbgcpu.memory.iter().all(|&x| x.is_none()) {
                    eprintln!(
                        "{}",
                        "CPU memory is empty.\nTry to load the program first.\n".red()
                    );
                } else if let Err(e) = dbgcpu.run() {
                    eprintln!("{e}");
                    eprintln!("Consider resetting the CPU with 'rs'\n");
                }
            }
            "spc" => 'spc: {
                if dbgcpu.memory.iter().all(|&x| x.is_none()) {
                    eprintln!(
                        "{}",
                        "CPU memory is empty.\nTry to load the program first.\n".red()
                    );
                    break 'spc;
                }
                if let Ok(n) = arg.trim().parse::<u16>() {
                    dbgcpu.pc = n;
                    println!("Program counter set to {n}\n");
                } else {
                    eprintln!("{} requires a numeric argument\n", "p | pmem".red());
                }
            }
            "p" | "pmem" => {
                if let Ok(n) = arg.parse::<usize>() {
                    if let Some(memvalue) = dbgcpu.memory[n] {
                        println!("Value in memory is:\n{memvalue:016b}\n{memvalue}");
                        let oldvalue = dbgcpu.ir;
                        dbgcpu.ir = memvalue;
                        println!("dumped instruction: {}", dbgcpu.parse_instruction());
                        dbgcpu.ir = oldvalue;
                    } else {
                        println!("{}", "Nothing in memory here.\n".yellow());
                    }
                } else {
                    eprintln!("{} requires a numeric argument\n", "p | pmem".red());
                }
            }
            "i" | "info" => {
                if !dbgcpu.has_ran {
                    eprintln!("{}", "CPU has not run.\n".red());
                    continue;
                }
                match arg.parse::<u32>() {
                    Ok(n) => {
                        clock = n;
                        CPU::display_state(&clock);
                    }
                    Err(_) => eprintln!("Error parsing second argument.\n"),
                }
            }
            "wb" => {
                if dbgcpu.memory.iter().all(|&x| x.is_none()) {
                    eprintln!(
                        "{}",
                        "CPU memory is empty.\nTry to load the program first.\n".red()
                    );
                } else {
                    println!("Execution begins at memory address {}", dbgcpu.starts_at);
                }
            }
            "e" | "exc" => 'exc: {
                dbgcpu.ir = if let Some(value) = dbgcpu.memory[dbgcpu.pc as usize] {
                    value
                } else {
                    eprintln!("Nothing at PC {}", dbgcpu.pc);
                    break 'exc;
                };
                let parsed_ins = dbgcpu.parse_instruction();
                if let Err(e) = dbgcpu.execute_instruction(&parsed_ins) {
                    eprintln!("An error occurred: {e}");
                }
                dbgcpu.record_state();
                println!("  Signed Integer Registers : {:?}", dbgcpu.int_reg);
                println!("  Uint registers           : {:?}", dbgcpu.uint_reg);
                println!("  Float Registers          : {:?}", dbgcpu.float_reg);
                println!("  Program Counter          : {}", dbgcpu.pc);
                println!("  Instruction Register     : {:016b}", dbgcpu.ir);
                println!("  Running                  : {}", dbgcpu.running);
                println!("  Zero flag                : {}", dbgcpu.zflag);
                println!("  Overflow flag            : {}", dbgcpu.oflag);
                println!("  Remainder flag           : {}", dbgcpu.rflag);
                println!("  Stack pointer            : {}", dbgcpu.sp);
                println!("  Base pointer             : {}", dbgcpu.bp);
                println!("  Instruction pointer      : {}", dbgcpu.ip);
                println!(
                    "  Disassembled Instruction : {}",
                    dbgcpu.parse_instruction()
                );
                let tmp = dbgcpu.ir;
                if let Some(n) = dbgcpu.memory[dbgcpu.pc as usize] {
                    dbgcpu.ir = n;
                    println!(
                        "  Next instruction         : {}\n",
                        dbgcpu.parse_instruction()
                    );
                }
                dbgcpu.ir = tmp;
            }
            "a" => {
                for (index, element) in dbgcpu.memory.iter().enumerate() {
                    if element.is_some() {
                        let mut tmpcpu = CPU::new();
                        tmpcpu.ir = element.unwrap();
                        println!("Value at {} is {}", index, tmpcpu.parse_instruction());
                    }
                }
                for (index, element) in dbgcpu.memory.iter().enumerate() {
                    if element.is_some() {
                        println!("Value at {} is {:016b}", index, element.unwrap());
                    }
                }
            }
            "w" => {
                println!("  Signed Integer Registers : {:?}", dbgcpu.int_reg);
                println!("  Uint registers           : {:?}", dbgcpu.uint_reg);
                println!("  Float Registers          : {:?}", dbgcpu.float_reg);
                println!("  Program Counter          : {}", dbgcpu.pc);
                println!("  Instruction Register     : {:016b}", dbgcpu.ir);
                println!("  Running                  : {}", dbgcpu.running);
                println!("  Zero flag                : {}", dbgcpu.zflag);
                println!("  Overflow flag            : {}", dbgcpu.oflag);
                println!("  Remainder flag           : {}", dbgcpu.rflag);
                println!("  Stack pointer            : {}", dbgcpu.sp);
                println!("  Base pointer             : {}", dbgcpu.bp);
                println!("  Instruction pointer      : {}", dbgcpu.ip);
                println!(
                    "  Disassembled Instruction : {}",
                    dbgcpu.parse_instruction()
                );
                let tmp = dbgcpu.ir;
                if let Some(n) = dbgcpu.memory[dbgcpu.pc as usize] {
                    dbgcpu.ir = n;
                    println!(
                        "  Next instruction         : {}\n",
                        dbgcpu.parse_instruction()
                    );
                }
                dbgcpu.ir = tmp;
            }
            "cls" | "clear" => {
                cls();
            }
            "pk" => 'pk: {
                if let Ok(n) = arg.parse::<usize>() {
                    if let Some(memvalue) = dbgcpu.memory[n] {
                        println!("Value in memory is:\n{memvalue:016b}\n{memvalue}");
                        let oldvalue = dbgcpu.ir;
                        dbgcpu.ir = memvalue;
                        println!("{}", dbgcpu.parse_instruction());
                        dbgcpu.ir = oldvalue;
                        let mut buffer = String::new();
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut buffer)?;
                        if buffer.is_empty() {
                            println!("Empty input\n");
                            break 'pk;
                        }
                        if buffer.trim().starts_with("0b") {
                            match i32::from_str_radix(&buffer.trim()[2..], 2) {
                                Ok(val) => {
                                    println!("Value in memory address {n} set to {val:016b}");
                                    dbgcpu.memory[n] = Some(val as i16);
                                    break 'pk;
                                }
                                Err(e) => println!("Input could not be parsed to binary\n{e}"),
                            }
                        }

                        if let Ok(v) = buffer.trim().parse::<i16>() {
                            println!("Value in memory address {n} set to {v}");
                            dbgcpu.memory[n] = Some(v);
                        } else {
                            println!("Could not parse a valid integer from input\n");
                        }
                    } else {
                        println!("{}", "Nothing in memory here.\n".yellow());
                    }
                } else {
                    eprintln!("{} requires a numeric argument\n", "pk".red());
                }
            }
            "im" => 'im: {
                if let Ok(n) = arg.parse::<usize>() {
                    let tmp = dbgcpu.ir;
                    let mval = display_mem(&n, &clock);
                    if mval.is_none() {
                        eprintln!("Nothing in memory here\n");
                        break 'im;
                    }
                    let uwrap_val = mval.unwrap() as i16;
                    dbgcpu.ir = uwrap_val; // can't panic
                    println!(
                        "Value in address {n} is\n{uwrap_val}\n{uwrap_val:016b}\nDisassembles to: {}\n",
                        dbgcpu.parse_instruction()
                    );
                    dbgcpu.ir = tmp;
                } else {
                    eprintln!("{} requires a numeric argument\n", "im".red());
                }
            }
            "rs" => {
                dbgcpu = CPU::new();
            }
            _ => unknown_command(&command),
        }
    }
}

fn unknown_command(command: &str) {
    println!(
        "Unknown command: {}\nType help or h to view available commands\n",
        command.red()
    );
}

pub fn bin_to_vec(file_path: &str) -> io::Result<Vec<i16>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut result: Vec<i16> = Vec::new();

    // Iterate over the buffer in chunks of 2 bytes
    for chunk in buffer.chunks(2) {
        if chunk.len() == 2 {
            // Only process full chunks of 2 bytes
            let value = i16::from_be_bytes([chunk[0], chunk[1]]);
            result.push(value);
        }
    }

    Ok(result)
}
