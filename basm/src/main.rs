/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
use basm::Error::*;
use basm::*;
use colored::Colorize;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let input: &String = &CONFIG.source;
    let file = Path::new(input);

    if input.is_empty() {
        eprintln!("{}", Error::LineLessError("no input files"));
        std::process::exit(1);
    }

    if File::open(file).is_err() {
        eprintln!(
            "{}",
            Error::LineLessError(format!("file {} does not exist", input).as_str())
        );
        std::process::exit(1);
    }
    if let Ok(metadata) = fs::metadata(input) {
        if metadata.is_dir() {
            let error_message = format!("{} is a directory", input);
            eprintln!("{}", Error::LineLessError(&error_message));
            std::process::exit(1);
        }
    }

    let lines = process_includes(input)?;

    let lines: Vec<String> = lines.iter().map(|line| line.trim().to_string()).collect();

    if CONFIG.verbose || CONFIG.debug {
        println!("{}", "Processing lines:".blue());
        for (index, line) in lines.iter().enumerate() {
            println!("{}: {}", index + 1, line.green());
        }
    }

    let mut encoded_instructions = Vec::new();
    let mut line_count: u32 = 1;
    let mut write_to_file: bool = true;
    if let Err(e) = process_variables(&lines) {
        eprintln!("{e}");
        write_to_file = false;
    }
    if let Err(e) = process_start(&lines) {
        eprintln!("{e}");
        write_to_file = false;
    }
    if let Err(e) = load_subroutines(&lines) {
        eprintln!("{e}");
        write_to_file = false;
    }
    let mut hlt_seen = false;
    for line in lines.into_iter() {
        let mut lexer = Lexer::new(&line, line_count);
        let line_before_comment = if line.trim().contains(';') {
            line.trim().split(';').next().unwrap_or(&line)
        } else {
            line.trim()
        };
        if line_before_comment.contains('=') {
            lexer.line_number += 1;
            line_count += 1;
            continue;
        }
        match lexer.lex() {
            Ok(tokens) => {
                if tokens.is_empty() {
                    line_count += 1;
                    continue;
                }

                let instruction = tokens.first();
                let operand1 = tokens.get(1);
                let operand2 = {
                    if let Some(Token::Comma) = tokens.get(2) {
                        tokens.get(3)
                    } else {
                        tokens.get(2)
                    }
                };

                if CONFIG.debug {
                    println!("\nRaw line: {}", line.green());
                }
                for token in tokens {
                    if let Token::Ident(_) = token {
                        if token.get_raw().to_lowercase() == "hlt" {
                            hlt_seen = true;
                        }
                    }
                    if CONFIG.debug {
                        println!(
                            "{} {}",
                            "Token:".green().bold(),
                            token.to_string().blue().bold()
                        );
                    }
                }
                if tokens.contains(&Token::EqualSign) {
                    continue;
                }
                if let Some(ins) = instruction {
                    let encoded_instruction =
                        encode_instruction(ins, operand1, operand2, line_count);

                    match encoded_instruction {
                        Ok(Some(vector)) => {
                            if let Err(err_msg) = verify(ins, operand1, operand2, line_count) {
                                write_to_file = false;
                                eprintln!("{}", err_msg);
                            } else {
                                for encoded in vector {
                                    encoded_instructions.extend(&encoded.to_be_bytes());
                                    if CONFIG.verbose || CONFIG.debug {
                                        println!("Instruction: {:016b}", encoded);
                                    }
                                }
                            }
                        }
                        Ok(None) => {
                            continue;
                        }
                        Err(err_msg) => {
                            write_to_file = false;
                            eprintln!("{err_msg}");
                        }
                    }
                }

                line_count += 1;
            }
            Err(err) => {
                eprintln!("{err}");
                write_to_file = false;
            }
        }
    }

    if !hlt_seen {
        println!(
            "{}: No HLT instruction found in program {}",
            "Warning".yellow(),
            CONFIG.source
        );
    }

    if CONFIG.debug {
        print_subroutine_map();
    }

    match &CONFIG.binary {
        Some(output_file) if write_to_file => {
            write_encoded_instructions_to_file(output_file, &encoded_instructions)?;
        }
        _ => {
            std::process::exit(1);
        }
    }
    Ok(())
}

fn process_includes(input: &String) -> io::Result<Vec<String>> {
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

        let trimmed_content = content.trim();

        if trimmed_content.starts_with("#include") {
            let start_quote = trimmed_content.find('"');
            let end_quote = trimmed_content.rfind('"');

            if let (Some(start), Some(end)) = (start_quote, end_quote) {
                if start < end {
                    let include_file = &trimmed_content[start + 1..end];
                    if let Ok(included) = read_include_file(include_file) {
                        included_lines.extend(included);
                    } else {
                        eprintln!(
                            "{}",
                            LineLessError(
                                format!("could not read included file: {include_file}").as_str()
                            )
                        );
                        return Err(io::Error::new(io::ErrorKind::Other, "Include file error"));
                    }
                }
            }
            continue;
        }

        included_lines.push(content);
    }

    Ok(included_lines)
}
fn read_include_file(file_name: &str) -> io::Result<Vec<String>> {
    let mut included_lines = Vec::new();
    let reader = io::BufReader::new(File::open(file_name)?);

    for line in reader.lines() {
        match line {
            Ok(content) => included_lines.push(content),
            Err(e) => {
                eprintln!(
                    "{}",
                    LineLessError(format!("error while reading from include file: {e}").as_str())
                );
                return Err(e);
            }
        }
    }
    Ok(included_lines)
}

fn write_encoded_instructions_to_file(
    filename: &str,
    encoded_instructions: &[u8],
) -> io::Result<()> {
    if CONFIG.debug || CONFIG.verbose {
        println!("{}", "Wrote to file.".green());
    }
    let mut file = File::create(filename)?;
    file.write_all(encoded_instructions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // no tests
}
