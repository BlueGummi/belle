/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
use crate::config::CONFIG;
use belle::*;
use std::{io, process};

fn main() -> io::Result<()> {
    cli_argument_check();

    let executable_path = &CONFIG.rom; 
    if CONFIG.debug {
        let mut bdb = BDB::new(executable_path)?;
        if let Err(e) = bdb.run() {
            eprintln!("{e}");
            process::exit(1);
        }
    }
    let rom = create_rom(executable_path)?;
    let mut cpu = CPU::new();
    if let Err(e) = cpu.load_rom(&rom) {
        if !CONFIG.quiet {
            eprintln!("{e}");
        }
        process::exit(1);
    }
    if let Err(e) = cpu.run() {
        if !CONFIG.pretty && !CONFIG.quiet {
            eprintln!("{e}");
        }
        if CONFIG.write {
            write_crash(&cpu);
        }
        process::exit(1);
    }
    if cpu.err {
        process::exit(1);
    }
    Ok(())
}
