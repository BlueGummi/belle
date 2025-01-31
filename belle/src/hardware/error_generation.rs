use crate::{config::CONFIG, *};
use colored::*;
use std::fmt;

#[derive(Debug)]
pub enum UnrecoverableError {
    SegmentationFault(i16, u16, Option<String>),
    IllegalInstruction(i16, u16, Option<String>),
    DivideByZero(i16, u16, Option<String>),
    InvalidRegister(i16, u16, Option<String>),
    StackOverflow(i16, u16, Option<String>),
    StackUnderflow(i16, u16, Option<String>),
    ReadFail(i16, u16, Option<String>),
    WindowFail(i16, u16, Option<String>), // IR, PC, MSG
}

#[derive(Debug)]
pub enum RecoverableError {
    UnknownFlag(u16, Option<String>),
    Overflow(u16, Option<String>),
    BackwardStack(u16, Option<String>),
}

pub type PossibleWarn = Result<(), RecoverableError>;
pub type PossibleCrash = Result<(), UnrecoverableError>;

impl std::error::Error for UnrecoverableError {}
impl std::error::Error for RecoverableError {}
impl UnrecoverableError {
    pub fn only_err(&self) -> String {
        let (_, err_type, _, _) = self.details();
        format!("{} {}", "UNRECOVERABLE ERROR:".red(), err_type.bold().red())
    }
    fn details(&self) -> (i16, &str, u16, &Option<String>) {
        match self {
            UnrecoverableError::SegmentationFault(ir, loc, msg) => {
                (*ir, "Segmentation fault", *loc, msg)
            }
            UnrecoverableError::IllegalInstruction(ir, loc, msg) => {
                (*ir, "Illegal instruction", *loc, msg)
            }
            UnrecoverableError::DivideByZero(ir, loc, msg) => (*ir, "Divide by zero", *loc, msg),
            UnrecoverableError::InvalidRegister(ir, loc, msg) => {
                (*ir, "Invalid register", *loc, msg)
            }
            UnrecoverableError::StackOverflow(ir, loc, msg) => (*ir, "Stack overflow", *loc, msg),
            UnrecoverableError::StackUnderflow(ir, loc, msg) => (*ir, "Stack underflow", *loc, msg),
            UnrecoverableError::ReadFail(ir, loc, msg) => (*ir, "Read fail", *loc, msg),
            UnrecoverableError::WindowFail(ir, loc, msg) => (*ir, "Window fail", *loc, msg),
        }
    }
}

impl RecoverableError {
    fn details(&self) -> (&str, u16, &Option<String>) {
        match self {
            RecoverableError::UnknownFlag(loc, msg) => ("Unknown flag", *loc, msg),
            RecoverableError::Overflow(loc, msg) => ("Overflow", *loc, msg),
            RecoverableError::BackwardStack(loc, msg) => ("Backwards stack", *loc, msg),
        }
    }
}

impl fmt::Display for UnrecoverableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (ir, err_type, location, msg) = self.details();
        write!(f, "{} ", "UNRECOVERABLE ERROR:".red())?;
        write!(f, "{}", err_type.bold().red())?;

        if let Some(s) = msg {
            if CONFIG.debug || CONFIG.verbose {
                write!(f, "\n{}", s.yellow())?;
            }
        }
        if CONFIG.debug || CONFIG.verbose {
            let line = "─".repeat(12);
            writeln!(f, "\n╭{}──────────{}─{}────╮", line, line, line)?;
            let mut cpu = CPU::new();
            cpu.ir = ir;
            write!(f, "│{}:", " Instruction".bold())?;
            write!(f, " {}", cpu.decode_instruction().to_string().bold())?;
            let inslen = 52
                - "│ Instruction".len()
                - cpu.decode_instruction().to_string().trim().len()
                - " Address: ".len()
                - location.to_string().len();
            write!(
                f,
                " {}: {}",
                "Address".blue().bold(),
                location.to_string().bold()
            )?;
            writeln!(f, "{}│", " ".repeat(inslen))?;
            writeln!(f, "╰{}──────────{}─{}────╯", line, line, line)?;
        }
        Ok(())
    }
}

impl fmt::Display for RecoverableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if CONFIG.compact_print || (!CONFIG.debug && !CONFIG.verbose) {
            return Ok(());
        }
        let (err_type, location, msg) = self.details();
        write!(f, "{}: ", "RECOVERABLE ERROR".yellow())?;
        write!(f, "{}", err_type.yellow())?;

        if let Some(s) = msg {
            if CONFIG.debug || (CONFIG.verbose && !CONFIG.compact_print) {
                write!(f, ": {}", s.magenta())?;
            }
        }
        if CONFIG.debug || (CONFIG.verbose && !CONFIG.compact_print) {
            writeln!(f, " at memory address {}", location.to_string().green())?;
        }
        Ok(())
    }
}

impl CPU {
    pub fn generate_invalid_register(&mut self) -> UnrecoverableError {
        self.running = false;
        UnrecoverableError::IllegalInstruction(
            self.ir,
            self.pc,
            Some("The register number is too large.".to_string()),
        )
    }

    pub fn generate_unknown_flag(&self, instruction: &str) -> RecoverableError {
        RecoverableError::UnknownFlag(
            self.pc,
            Some(format!("Unknown flag in {instruction} instruction")),
        )
    }

    pub fn generate_divbyz(&mut self) -> UnrecoverableError {
        self.running = false;
        UnrecoverableError::DivideByZero(
            self.ir,
            self.pc,
            Some("Attempted to divide by zero.".to_string()),
        )
    }
    pub fn check_overflow(&mut self, new_value: i64, register: u16) -> PossibleWarn {
        let overflowed = match register {
            0..=3 => new_value > i64::from(i16::MAX) || new_value < i64::from(i16::MIN),
            4..=5 => new_value > i64::from(u16::MAX) || new_value < i64::from(u16::MIN),
            6..=7 => new_value > f32::MAX as i64 || new_value < f32::MIN as i64,
            _ => true,
        };
        if overflowed {
            self.oflag = true;
            if self.hlt_on_overflow {
                self.running = false;
                if CONFIG.verbose {
                    println!("Halting...");
                }
                if CONFIG.pretty {
                    for i in 0..=3 {
                        println!(
                            "Register {}: {}, {:016b}, {:04x}",
                            i, self.int_reg[i], self.int_reg[i], self.int_reg[i]
                        );
                    }
                    for i in 0..=1 {
                        println!("Uint Register {}: {}", i, self.uint_reg[i]);
                    }
                    for i in 0..=1 {
                        println!("Float Register {}: {}", i, self.float_reg[i]);
                    }
                }
            }
            return Err(RecoverableError::Overflow(
                self.pc,
                Some("Overflowed a register.".to_string()),
            ));
        }
        Ok(())
    }

    pub fn generate_segfault(&mut self, message: &str) -> UnrecoverableError {
        self.running = false;
        self.err = true;
        UnrecoverableError::SegmentationFault(self.ir, self.pc, Some(message.to_string()))
    }
}
