/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
use basm::Error::*;
use basm::*;
use colored::Colorize;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let input: &String = &CONFIG.file;
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
    let _ = process_start(&lines);
    let _ = load_subroutines(&lines);

    let mut hlt_seen = false;
    for line in lines {
        let mut lexer = Lexer::new(&line, line_count);
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
                    println!("Raw line: {}", line.green());
                }
                for token in tokens {
                    if token.get_raw().to_lowercase() == "hlt" {
                        hlt_seen = true;
                    }
                    if CONFIG.debug {
                        println!(
                            "{} {}",
                            "Token:".green().bold(),
                            token.to_string().blue().bold()
                        );
                    }
                }
                if CONFIG.debug {
                    println!();
                }
                if let Some(ins) = instruction {
                    let encoded_instruction =
                        encode_instruction(ins, operand1, operand2, line_count);

                    match encoded_instruction {
                        Ok(Some(encoded)) => {
                            if let Err(err_msg) = verify(ins, operand1, operand2, line_count) {
                                write_to_file = false;
                                eprintln!("{}", err_msg);
                            } else {
                                encoded_instructions.extend(&encoded.to_be_bytes());
                                if CONFIG.verbose || CONFIG.debug {
                                    println!("Instruction: {:016b}", encoded);
                                }
                            }
                        }
                        Ok(None) => {
                            continue;
                        }
                        Err(err_msg) => {
                            write_to_file = false;
                            eprintln!("{}", err_msg);
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
            "{}: No HLT instruction found in program.",
            "Warning".yellow()
        );
    }

    if CONFIG.debug {
        print_subroutine_map();
    }

    match &CONFIG.output {
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
    let include_regex = Regex::new(r#"^\s*#include\s+"([^"]+)""#).unwrap();
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

        if content.trim().starts_with("#include") {
            if let Some(captures) = include_regex.captures(content.trim()) {
                let include_file = captures[1].to_string();
                if let Ok(included) = read_include_file(&include_file) {
                    included_lines.extend(included);
                } else if let Err(e) = read_include_file(&include_file) {
                    eprintln!(
                        "{}",
                        LineLessError(
                            format!("could not read included file: {include_file}").as_str()
                        )
                    );
                    return Err(e);
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
