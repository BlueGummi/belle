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

    let executable_path = &CONFIG.file;

    let rom = create_rom(executable_path)?;
    if CONFIG.debug {
        let mut bdb = BDB::new(executable_path)?;
        if let Err(e) = bdb.run() {
            eprintln!("{e}");
            process::exit(1);
        }
    }

    let mut cpu = CPU::new();
    if let Err(e) = cpu.load_rom(&rom) {
        eprintln!("{e}");
        process::exit(1);
    }
    if let Err(e) = cpu.run() {
        eprintln!("{e}");
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
