use crate::*;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::process;
pub fn cli_argument_check() {
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
}

pub fn create_rom(file_path: &str) -> io::Result<Vec<i16>> {
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
