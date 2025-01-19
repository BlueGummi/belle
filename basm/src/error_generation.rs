use crate::CONFIG;
use colored::Colorize;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub enum Error<'a> {
    InvalidSyntax(&'a str, usize, Option<usize>),
    ExpectedArgument(&'a str, usize, Option<usize>),
    NonexistentData(&'a str, usize, Option<usize>),
    UnknownCharacter(String, usize, Option<usize>),
    OtherError(&'a str, usize, Option<usize>),
    LineLessError(&'a str),
}

pub type AssemblerError<'a> = Result<(), Error<'a>>;

impl std::error::Error for Error<'_> {}
impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_number = match self {
            Error::InvalidSyntax(_, n, _)
            | Error::ExpectedArgument(_, n, _)
            | Error::NonexistentData(_, n, _)
            | Error::UnknownCharacter(_, n, _)
            | Error::OtherError(_, n, _) => *n,

            Error::LineLessError(_) => {
                return write!(f, "error: {}", self.message());
            }
        };

        let error_message = match self {
            Error::InvalidSyntax(s, _, _) => format!("invalid syntax: \n{s}"),
            Error::ExpectedArgument(s, _, _) => format!("expected an argument: \n{s}"),
            Error::NonexistentData(s, _, _) => format!("nonexistent data: \n{s}"),
            Error::UnknownCharacter(s, _, _) => format!("has unknown character: \n{s}"),
            Error::OtherError(s, _, _) => (*s).to_string(),
            _ => unreachable!(),
        };

        let location = match self {
            Error::InvalidSyntax(_, _, n)
            | Error::ExpectedArgument(_, _, n)
            | Error::NonexistentData(_, _, n)
            | Error::UnknownCharacter(_, _, n)
            | Error::OtherError(_, _, n) => n,
            _ => unreachable!(),
        };

        writeln!(
            f,
            "error on line {}: {}",
            line_number.to_string().as_str().green(),
            error_message
        )?;

        let input: &String = &CONFIG.source;
        let path = Path::new(input);
        for (current_line, line) in io::BufReader::new(File::open(path).unwrap())
            .lines()
            .enumerate()
        {
            if current_line + 1 == line_number {
                writeln!(f, "{:^6} {} {}", line_number.to_string().blue(), "|".blue(), line.unwrap().trim())?;
            }
        }
        if let Some(place) = location {
            let spaces = " ".repeat({ *place } + 7);
            writeln!(f, "{}{}", spaces, "^^".red().bold())?;
        }

        Ok(())
    }
}

impl Error<'_> {
    fn message(&self) -> &str {
        match self {
            Error::LineLessError(s) => s,
            _ => "",
        }
    }
}
