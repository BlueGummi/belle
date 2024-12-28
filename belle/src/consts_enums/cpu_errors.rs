use crate::*;
use colored::Colorize;
use std::fmt;

#[derive(Debug)]
pub enum UnrecoverableError {
    SegmentationFault(i16, u16, Option<String>),
    IllegalInstruction(i16, u16, Option<String>),
    DivideByZero(i16, u16, Option<String>),
    InvalidRegister(i16, u16, Option<String>),
    StackOverflow(i16, u16, Option<String>),
    StackUnderflow(i16, u16, Option<String>),
}

#[derive(Debug)]
pub enum RecoverableError {
    UnknownFlag(u16, Option<String>),
    Overflow(u16, Option<String>),
    BackwardStack(u16, Option<String>),
}

pub type Oopsie = Result<(), RecoverableError>;
pub type Death = Result<(), UnrecoverableError>; // what am I supposed to call it?

impl std::error::Error for UnrecoverableError {}
impl std::error::Error for RecoverableError {}
impl UnrecoverableError {
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
            writeln!(f, "\nAt memory address {}", location.to_string().green())?;
            let mut cpu = CPU::new();
            cpu.ir = ir;
            writeln!(
                f,
                "{} was {}",
                "Instruction".blue(),
                cpu.decode_instruction().to_string().cyan()
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for RecoverableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !CONFIG.debug && !CONFIG.verbose {
            return Ok(());
        }
        let (err_type, location, msg) = self.details();
        write!(f, "{}: ", "RECOVERABLE ERROR:".yellow())?;
        write!(f, "{}", err_type.yellow())?;

        if let Some(s) = msg {
            if CONFIG.debug || CONFIG.verbose {
                write!(f, ": {}", s.magenta())?;
            }
        }
        if CONFIG.debug || CONFIG.verbose {
            writeln!(f, " at memory address {}", location.to_string().green())?;
        }
        Ok(())
    }
}
