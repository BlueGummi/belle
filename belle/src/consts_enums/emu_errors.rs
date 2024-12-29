use crate::config::CONFIG;
use colored::Colorize;
use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum EmuError {
    FileNotFound(),
    IsDirectory(),
    MemoryOverflow(),
    Duplicate(String),
    ReadFail(String),
    Impossible(String),
}

impl EmuError {
    pub fn err(&self) {
        if CONFIG.quiet {
            return;
        }
        eprint!("{} ", "Emulator Error:".red());
        match self {
            EmuError::FileNotFound() => {
                eprintln!("File {} not found", CONFIG.file.to_string().green());
            }
            EmuError::MemoryOverflow() => {
                eprintln!("{}", "Memory will overflow".red());
            }
            EmuError::Duplicate(s) => {
                eprintln!("Duplicate: {}", s.red());
            }
            EmuError::ReadFail(s) => {
                eprintln!(
                    "{}: {}",
                    "Failed to read from stdin and parse to i16".red(),
                    s
                );
            }
            EmuError::Impossible(s) => {
                eprintln!("{}: {}", "Configuration combination not possible".red(), s);
            }
            EmuError::IsDirectory() => {
                eprintln!("{} is a directory", CONFIG.file.to_string().green());
            }
        }
        if let EmuError::ReadFail(_) = self {
            println!("{}", "Retrying..".yellow());
        }
    }
}

impl fmt::Display for EmuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmuError::FileNotFound() => {
                write!(
                    f,
                    "{} File {} not found",
                    "Emulator Error:".red(),
                    CONFIG.file.to_string().green(),
                )
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
