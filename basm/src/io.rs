use crate::Error::*;
use crate::*;
use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
pub fn process_includes(input: &String) -> io::Result<Vec<String>> {
    let mut included_lines = Vec::new();
    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let content = match line {
            Ok(content) => content,
            Err(e) => {
                eprintln!(
                    "{}",
                    LineLessError(format!("error while reading from file: {e}").as_str())
                );
                return Err(e);
            }
        };

        included_lines.push(content);
    }

    Ok(included_lines)
}
pub fn write_encoded_instructions_to_file(
    filename: &str,
    encoded_instructions: &[u8],
) -> io::Result<()> {
    if CONFIG.verbose {
        println!("{}", "wrote to file.".green());
    }
    let mut file = File::create(filename)?;
    file.write_all(encoded_instructions)?;
    Ok(())
}
pub fn print_line(line_number: usize, print_help: bool, print_header: bool) -> io::Result<()> {
    let path = Path::new(&CONFIG.source);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for (current_line, line) in reader.lines().enumerate() {
        if current_line + 1 == line_number {
            match line {
                Ok(content) => {
                    let trimmed_content = content.trim();
                    let mut printed_line = trimmed_content.to_string();
                    let mut comment_part = "".to_string();

                    let mut in_quotes = false;
                    if let Some(pos) = trimmed_content.find(|c| {
                        if c == '"' {
                            in_quotes = !in_quotes;
                        }
                        c == ';' && !in_quotes
                    }) {
                        printed_line = trimmed_content[..pos].trim().to_string();
                        comment_part = trimmed_content[pos..].trim().to_string();
                    }
                    let left_char = if print_help { "├" } else { "╰" };
                    let line_prefix = if print_help { "│" } else { " " };
                    if print_header {
                        println!(
                            "{}{} {}:{}",
                            left_char.bright_red(),
                            "─".bright_red(),
                            CONFIG.source.green(),
                            line_number
                        );
                    }
                    println!(
                        "{}{:^6} {} {} {}",
                        line_prefix.bright_red(),
                        (current_line + 1).to_string().blue(),
                        "|".blue(),
                        printed_line,
                        comment_part.dimmed()
                    );
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("error reading line: {}", e);
                    return Err(e);
                }
            }
        }
    }

    eprintln!("line number {} is out of bounds.", line_number);
    Err(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "line not found",
    ))
}
