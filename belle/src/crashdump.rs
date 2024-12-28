use crate::*;
use std::fs::{self, File};
use std::io::Write;
pub fn write_crash(cpu: &CPU) {
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
                cpu.decode_instruction()
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
