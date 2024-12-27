/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
use belle::*;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    if CONFIG.debug && CONFIG.verbose {
        eprintln!(
            "{}",
            EmuError::Impossible("Cannot have both debug and verbose flags".to_string())
        );
        process::exit(1);
    }
    if CONFIG.quiet && CONFIG.verbose {
        eprintln!(
            "{}",
            EmuError::Impossible("Cannot have both debug and quiet flags".to_string())
        );
        process::exit(1);
    }
    let executable_path = &CONFIG.file;

    if let Ok(metadata) = fs::metadata(executable_path) {
        if metadata.is_dir() {
            eprintln!("{}", EmuError::IsDirectory());
            process::exit(1);
        }
    }
    if File::open(Path::new(executable_path)).is_err() {
        eprintln!("{}", EmuError::FileNotFound());
        process::exit(1);
    }
    let bin = bin_to_vec(executable_path)?;
    if CONFIG.debug {
        let mut bdb = BDB::new(executable_path)?;
        if let Err(e) = bdb.run() {
            eprintln!("{e}");
            process::exit(1);
        }
    }
    if CONFIG.verbose {
        println!("CPU Initialized");
    }
    let mut cpu = CPU::new();
    if let Err(e) = cpu.load_binary(&bin) {
        eprintln!("{e}");
        process::exit(1);
    }
    if let Err(e) = cpu.run() {
        eprintln!("{e}");
        if CONFIG.write {
            let mut file_index = 0;
            let mut filename;

            loop {
                filename = format!("crashdump-{file_index:02}.txt");
                if fs::metadata(&filename).is_err() {
                    let mut file = File::create(&filename).expect("Failed to create file");

                    let mut write_to_file = |msg: &str| {
                        if let Err(e) = writeln!(file, "{}", msg) {
                            eprintln!("Failed to write to file: {}", e);
                        }
                    };
                    write_to_file("------ CRASH DUMP ------");
                    write_to_file(&format!(
                        "\n\n  Signed Integer Registers : {:?}",
                        cpu.int_reg
                    ));
                    write_to_file(&format!("  Uint registers           : {:?}", cpu.uint_reg));
                    write_to_file(&format!("  Float Registers          : {:?}", cpu.float_reg));
                    write_to_file(&format!("  Program Counter          : {}", cpu.pc));
                    write_to_file(&format!("  Instruction Register     : {:016b}", cpu.ir));
                    write_to_file(&format!("  Running                  : {}", cpu.running));
                    write_to_file(&format!("  Zero flag                : {}", cpu.zflag));
                    write_to_file(&format!("  Overflow flag            : {}", cpu.oflag));
                    write_to_file(&format!("  Remainder flag           : {}", cpu.rflag));
                    write_to_file(&format!("  Sign flag                : {}", cpu.sflag));
                    write_to_file(&format!("  Stack pointer            : {}", cpu.sp));
                    write_to_file(&format!("  Base pointer             : {}", cpu.bp));
                    write_to_file(&format!("  Instruction pointer      : {}", cpu.ip));
                    write_to_file(&format!(
                        "  Disassembled Instruction : {}",
                        cpu.parse_instruction()
                    ));

                    write_to_file("\n------ MEMORY ------\n");
                    for (index, value) in cpu.memory.iter().enumerate() {
                        if cpu.memory[index].is_some() {
                            if index == cpu.ip as usize {
                                write_to_file(&format!(
                                    "Address {index}: {:016b} <---- CRASH OCCURRED HERE",
                                    value.unwrap()
                                ));
                            } else {
                                write_to_file(&format!("Address {index}: {:016b}", value.unwrap()));
                            }
                        }
                    }
                    break;
                }
                file_index += 1;
            }
        }
        process::exit(1);
    }
    if cpu.err {
        process::exit(1);
    }
    Ok(())
}
