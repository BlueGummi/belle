/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
use basm::*;
use colored::Colorize;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
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

    if CONFIG.verbose {
        println!("{}", "Processing lines:".blue());
        for (index, line) in lines.iter().enumerate() {
            println!("{}: {}", index + 1, line.green());
        }
    }

    let mut encoded_instructions = Vec::new();
    let mut line_count = 1;
    let mut write_to_file: bool = true;
    if let Err((l, m)) = process_variables(&lines) {
        println!("{}: {}", "error".bright_red().bold(), m);
        print_line(l)?;
        write_to_file = false;
    }
    if let Err((l, e)) = process_start(&lines) {
        println!("{}: {}", "error".bright_red().bold(), e);
        print_line(l)?;
        write_to_file = false;
    }
    if let Err(e) = load_labels(&lines) {
        println!("{}: {}", "error".bright_red().bold(), e);
        write_to_file = false;
    }

    let label_lock = LABEL_MAP.lock().unwrap();
    let variable_lock = VARIABLE_MAP.lock().unwrap();

    let label_keys: HashSet<_> = label_lock.keys().collect();
    let variable_keys: HashSet<_> = variable_lock.keys().collect();
    if let Some(key) = label_keys.intersection(&variable_keys).next() {
        eprintln!(
            "Variable and label {} cannot have the same name.",
            key.to_string().yellow()
        );
        std::process::exit(1);
    }
    std::mem::drop(label_lock);
    std::mem::drop(variable_lock);

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

                for token in tokens {
                    if let Token::Ident(_) = token {
                        if token.get_raw().to_lowercase() == "hlt" {
                            hlt_seen = true;
                        }
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
                            if let Err((lineee, err_msg)) =
                                verify(ins, operand1, operand2, line_count)
                            {
                                write_to_file = false;
                                println!("{}: {}", "error".bright_red().bold(), err_msg);
                                print_line(lineee)?;
                            } else {
                                for encoded in vector {
                                    encoded_instructions.extend(&encoded.to_be_bytes());
                                }
                            }
                        }
                        Ok(None) => (),
                        Err((line_num, err_msg)) => {
                            write_to_file = false;
                            println!("{}: {}", "error".bright_red().bold(), err_msg);
                            print_line(line_num)?;
                        }
                    }
                }

                line_count += 1;
            }
            Err(err) => {
                println!("{err}");
                write_to_file = false;
            }
        }
    }

    if !hlt_seen {
        println!(
            "{}: {} {}",
            "warning".yellow(),
            "no HLT instruction present in".bold(),
            CONFIG.source.green()
        );
    }

    print_label_map();

    match &CONFIG.binary {
        Some(output_file) if write_to_file => {
            let start_bin = 0b0010_0000_0000 | *START_LOCATION.lock().unwrap();
            encoded_instructions.insert(0, (start_bin & 0xff) as u8);
            encoded_instructions.insert(0, ((start_bin & 0xff00) >> 8) as u8);
            encoded_instructions.insert(0, 0x02);
            encoded_instructions.insert(0, 0x01);
            write_encoded_instructions_to_file(output_file, &encoded_instructions)?;
        }
        _ => {
            std::process::exit(1);
        }
    }
    Ok(())
}
