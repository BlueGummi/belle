use crate::CONFIG;
use colored::Colorize;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Tip = Option<String>;
#[derive(Debug)]
pub enum Error<'a> {
    InvalidSyntax(String, usize, Option<usize>, Tip),
    ExpectedArgument(&'a str, usize, Option<usize>, Tip),
    NonexistentData(&'a str, usize, Option<usize>, Tip),
    UnknownCharacter(String, usize, Option<usize>, Tip),
    OtherError(&'a str, usize, Option<usize>, Tip),
    LineLessError(&'a str),
}

pub type AssemblerError<'a> = Result<(), Error<'a>>;

impl std::error::Error for Error<'_> {}
impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_number = match self {
            Error::InvalidSyntax(_, n, _, _)
            | Error::ExpectedArgument(_, n, _, _)
            | Error::NonexistentData(_, n, _, _)
            | Error::UnknownCharacter(_, n, _, _)
            | Error::OtherError(_, n, _, _) => *n,

            Error::LineLessError(_) => {
                return write!(
                    f,
                    "{}: {}",
                    "error".underline().bright_red().bold(),
                    self.message()
                );
            }
        };

        let error_message = match self {
            Error::InvalidSyntax(s, _, _, _) => format!("{}: {s}", "invalid syntax".bold()),
            Error::ExpectedArgument(s, _, _, _) => {
                format!("{}: {}", "expected argument".bold(), s.cyan())
            }
            Error::NonexistentData(s, _, _, _) => format!("nonexistent data: {s}"),
            Error::UnknownCharacter(s, _, _, _) => {
                format!("{}: {}", "unknown character".bold(), s.cyan())
            }
            Error::OtherError(s, _, _, _) => (*s).to_string(),
            _ => unreachable!(),
        };

        let location = match self {
            Error::InvalidSyntax(_, _, n, _)
            | Error::ExpectedArgument(_, _, n, _)
            | Error::NonexistentData(_, _, n, _)
            | Error::UnknownCharacter(_, _, n, _)
            | Error::OtherError(_, _, n, _) => n,
            _ => unreachable!(),
        };

        writeln!(
            f,
            "{}: {}",
            "error".bright_red().bold().underline(),
            error_message
        )?;

        let input: &String = &CONFIG.source;
        let path = Path::new(input);
        for (current_line, line) in io::BufReader::new(File::open(path).unwrap())
            .lines()
            .enumerate()
        {
            if current_line + 1 == line_number {
                let line_content_original = line.unwrap();
                let mut line_content = line_content_original.trim().to_string();
                let mut in_quotes = false;
                let mut comment_index = None;

                for (i, c) in line_content.chars().enumerate() {
                    if c == '"' {
                        in_quotes = !in_quotes;
                    } else if c == ';' && !in_quotes {
                        comment_index = Some(i);
                        break;
                    }
                }

                if let Some(index) = comment_index {
                    let code_part = &line_content[..index];
                    let comment_part = &line_content[index..];
                    line_content = format!("{}{}", code_part, comment_part.dimmed());
                }
                let spaces = line_content_original
                    .chars()
                    .take_while(|&c| c == ' ')
                    .count();
                if let Some(place) = location {
                    let place = *place - 2;
                    let left = if self.get_tip().is_empty() {
                        "╰"
                    } else {
                        "├"
                    };
                    let second_line_char = if self.get_tip().is_empty() {
                        " "
                    } else {
                        "│"
                    };
                    if place < line_content.len() {
                        let before = &line_content[..place];
                        let error_char = &line_content[place..place + 1];
                        let after = &line_content[place + 1..];
                        writeln!(
                            f,
                            "{}{} {}:{}:{}",
                            left.bright_red(),
                            "─".bright_red(),
                            CONFIG.source.green(),
                            line_number,
                            place + spaces + 1
                        )?;
                        writeln!(
                            f,
                            "{}{:^6} {} {}{}{}",
                            second_line_char.bright_red(),
                            line_number.to_string().blue(),
                            "│".blue(),
                            before,
                            error_char.bright_red().bold(),
                            after
                        )?;
                    } else {
                        writeln!(
                            f,
                            "{}{} {}:{}",
                            left.bright_red(),
                            "─".bright_red(),
                            CONFIG.source.green(),
                            line_number
                        )?;
                        writeln!(
                            f,
                            "{}{:^6} {} {}",
                            second_line_char.bright_red(),
                            line_number.to_string().blue(),
                            "│".blue(),
                            line_content
                        )?;
                    }
                }
            }
        }
        let left_char = if self.get_tip().is_empty() {
            " "
        } else {
            "│"
        };
        if let Some(place) = location {
            let spaces = " ".repeat({ *place } + 6).bright_red();
            writeln!(
                f,
                "{}{}{}",
                left_char.to_string().bright_red(),
                spaces,
                "^^^".red().bold()
            )?;
        }
        if !self.get_tip().is_empty() {
            writeln!(
                f,
                "{}{} {}: {}",
                "╰".bright_red(),
                ">".yellow(),
                "help".yellow(),
                self.get_tip()
            )?;
        }

        Ok(())
    }
}

impl Error<'_> {
    fn message(&self) -> String {
        match self {
            Error::LineLessError(s) => s.bold().to_string(),
            _ => String::from(""),
        }
    }
    fn get_tip(&self) -> String {
        match self {
            Error::InvalidSyntax(_, _, _, s)
            | Error::ExpectedArgument(_, _, _, s)
            | Error::NonexistentData(_, _, _, s)
            | Error::UnknownCharacter(_, _, _, s)
            | Error::OtherError(_, _, _, s) => {
                if let Some(v) = s {
                    v.to_string()
                } else {
                    String::from("")
                }
            }
            _ => unreachable!(),
        }
    }
}
