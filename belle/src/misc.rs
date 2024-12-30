use crate::config::CONFIG;
use colored::Colorize;
use std::{
    error::Error,
    fmt,
    fs::{self, File},
    io::{self, Read},
    path::Path,
    process,
};

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

#[derive(Debug)]
pub enum EmuError {
    FileNotFound(),
    IsDirectory(),
    MemoryOverflow(),
    Duplicate(String),
    ReadFail(String),
    Impossible(String),
}

impl fmt::Display for EmuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmuError::FileNotFound() => {
                if !CONFIG.file.to_string().trim().is_empty() {
                    write!(
                        f,
                        "{} File {} not found",
                        "Emulator Error:".red(),
                        CONFIG.file.to_string().green(),
                    )
                } else {
                    write!(
                        f,
                        "{} {}",
                        "Emulator Error:".red(),
                        "No ROM provided".bold()
                    )
                }
            }
            EmuError::MemoryOverflow() => {
                write!(
                    f,
                    "{} {}",
                    "Emulator Error:".red(),
                    "Memory will overflow".red()
                )
            }
            EmuError::Duplicate(s) => {
                write!(f, "{} Duplicate: {}", "Emulator Error:".red(), s.red(),)
            }
            EmuError::ReadFail(s) => {
                write!(
                    f,
                    "{} Failed to read from stdin and parse to i16: {}",
                    "Emulator Error:".red(),
                    s,
                )
            }
            EmuError::Impossible(s) => {
                write!(
                    f,
                    "{} Configuration combination not possible: {}",
                    "Emulator Error:".red(),
                    s,
                )
            }
            EmuError::IsDirectory() => {
                write!(
                    f,
                    "{} {} is a directory",
                    "Emulator Error:".red(),
                    CONFIG.file.to_string().green(),
                )
            }
        }
    }
}

impl Error for EmuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
