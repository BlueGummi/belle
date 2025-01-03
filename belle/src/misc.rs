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
#[macro_export]
macro_rules! test_instruction {
    ($bcpu:expr, $op:ident, $arg1:expr, $arg2:expr) => {{
        use $crate::Argument::*;
        use $crate::Instruction::*;
        use $crate::*;

        let bcpu = &mut $bcpu;

        let parse_argument = |arg: &str| -> Argument {
            match arg {
                s if s.starts_with("&r") => {
                    let val: i16 = s[2..].parse().expect("Invalid register pointer");
                    RegPtr(val)
                }
                s if s.starts_with("&$") => {
                    let val: i16 = s[2..].parse().expect("Invalid memory pointer");
                    MemPtr(val)
                }
                s if s.starts_with("$") => {
                    let val: i16 = s[1..].parse().expect("Invalid memory address");
                    MemAddr(val)
                }
                s if s.parse::<i16>().is_ok() => {
                    let val: i16 = s.parse().expect("Invalid literal");
                    Literal(val)
                }
                s if s.starts_with("r") => {
                    let val: i16 = s[1..].parse().expect("Invalid register");
                    Register(val)
                }
                _ => panic!("moo"),
            }
        };

        let ins = match stringify!($op).to_uppercase().as_str() {
            "HLT" => HLT,
            "ADD" => ADD(parse_argument($arg1), parse_argument($arg2)),
            "JO" => JO(parse_argument($arg1)),
            "POP" => POP(parse_argument($arg1)),
            "DIV" => DIV(parse_argument($arg1), parse_argument($arg2)),
            "RET" => RET,
            "LD" => LD(parse_argument($arg1), parse_argument($arg2)),
            "ST" => ST(parse_argument($arg1), parse_argument($arg2)),
            "JMP" => JMP(parse_argument($arg1)),
            "JZ" => JZ(parse_argument($arg1)),
            "CMP" => CMP(parse_argument($arg1), parse_argument($arg2)),
            "MUL" => MUL(parse_argument($arg1), parse_argument($arg2)),
            "PUSH" => PUSH(parse_argument($arg1)),
            "INT" => INT(parse_argument($arg1)),
            "MOV" => MOV(parse_argument($arg1), parse_argument($arg2)),
            "NOP" => NOP,
            _ => panic!("cheep cheep"),
        };

        if let Err(e) = bcpu.execute_instruction(&ins) {
            panic!("{e}");
        }
        bcpu
    }};

    ($bcpu:expr, $op:ident, $arg1:expr) => {{
        test_instruction!($bcpu, $op, $arg1, "");
    }};

    ($bcpu:expr, $op:ident) => {{
        test_instruction!($bcpu, $op, "", "");
    }};
}

#[macro_export]
macro_rules! set_register {
    ($bcpu:expr, $register:expr, $value:expr) => {{
        use $crate::*;

        let bcpu = &mut $bcpu;
        if let Err(e) = bcpu.set_register_value(&Argument::Register($register), $value) {
            panic!("{e}");
        }
        bcpu
    }};
}
pub fn length_without_ansi(text: &str) -> usize {
    let mut cleaned_text = String::new();
    let mut inside_ansi = false;

    for c in text.chars() {
        if c == '\x1B' {
            inside_ansi = true;
        } else if inside_ansi {
            if c == 'm' || c == 'K' {
                inside_ansi = false;
            }
        } else {
            cleaned_text.push(c);
        }
    }

    cleaned_text.len()
}
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
        if executable_path.is_empty() {
            process::exit(0);
        }
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
