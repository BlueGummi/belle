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
use std::path::Path;
fn main() {
    let input: &String = &CONFIG.source;
    let file = Path::new(input);
    let mut error_count = 0;
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

    let lines = match process_includes(input) {
        Ok(v) => v,
        Err(_) => std::process::exit(1),
    };

    let lines: Vec<String> = lines.iter().map(|line| line.trim().to_string()).collect();

    let mut encoded_instructions = Vec::new();
    let mut line_count = 1;
    let mut write_to_file: bool = true;
    if let Err((l, o, m, tip)) = process_variables(&lines) {
        // this will do a chain
        println!("{}: {}", "error".underline().bright_red().bold(), m);
        error_count += 1;
        if let Err(e2) = print_line(l + 1, !tip.is_empty(), true) {
            println!("{}: {}", "error".underline().bright_red().bold(), e2);
            error_count += 1;
        }
        if !tip.is_empty() {
            println!(
                "{}\n{}{} {}: {} {}",
                "│".bright_red(),
                "╰".bright_red(),
                ">".yellow(),
                "help".yellow(),
                "╮".bright_red(),
                tip
            );
        }
        if let Some(v) = o {
            print!("         {}{}", "╰".bright_red(), ">".yellow());
            if let Err(e2) = print_line(v + 1, false, false) {
                println!("{}: {}", "error".underline().bright_red().bold(), e2);
                error_count += 1;
            }
        }
        println!();
        write_to_file = false;
    }
    if let Err((l, e)) = process_start(&lines) {
        println!("{}: {}", "error".underline().bright_red().bold(), e);
        error_count += 1;
        if let Err(e2) = print_line(l + 1, false, true) {
            println!("{}: {}", "error".underline().bright_red().bold(), e2);
            error_count += 1;
        }
        println!();
        write_to_file = false;
    }
    if let Err((l, o, e, tip)) = load_labels(&lines) {
        // this also chains
        println!("{}: {}", "error".underline().bright_red().bold(), e);
        error_count += 1;
        if let Err(e2) = print_line(l + 1, !tip.is_empty(), true) {
            println!("{}: {}", "error".underline().bright_red().bold(), e2);
            error_count += 1;
        }
        if !tip.is_empty() {
            println!(
                "{}\n{}{} {}: {} {}",
                "│".bright_red(),
                "╰".bright_red(),
                ">".yellow(),
                "help".yellow(),
                "╮".bright_red(),
                tip
            );
        }
        if let Some(v) = o {
            print!("         {}{}", "╰".bright_red(), ">".yellow());
            if let Err(e2) = print_line(v + 1, false, false) {
                println!("{}: {}", "error".underline().bright_red().bold(), e2);
                error_count += 1;
            }
        }
        println!();
        write_to_file = false;
    }

    let label_lock = LABEL_MAP.lock().unwrap();
    let variable_lock = VARIABLE_MAP.lock().unwrap();

    let label_keys: HashSet<_> = label_lock.keys().collect();
    let variable_keys: HashSet<_> = variable_lock.keys().collect();
    if let Some(key) = label_keys.intersection(&variable_keys).next() {
        eprintln!(
            "variable and label {} cannot have the same name.",
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
                if CONFIG.verbose {
                    println!("\n{}: {line}", "raw line".green());
                    for token in tokens {
                        println!("{}: {token}", "token".cyan());
                    }
                }
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
                                println!(
                                    "{}: {}",
                                    "error".underline().bright_red().bold(),
                                    err_msg
                                );
                                error_count += 1;
                                if let Err(e) = print_line(lineee, false, true) {
                                    println!("{}: {}", "error".underline().bright_red().bold(), e);
                                    error_count += 1;
                                }
                            } else {
                                for encoded in vector {
                                    encoded_instructions.extend(&encoded.to_be_bytes());
                                }
                            }
                        }
                        Ok(None) => (),
                        Err((line_num, (err_msg, tip))) => {
                            write_to_file = false;
                            println!("{}: {}", "error".underline().bright_red().bold(), err_msg);
                            error_count += 1;
                            if let Err(e) = print_line(line_num, !tip.is_empty(), true) {
                                println!("{}: {}", "error".underline().bright_red().bold(), e);
                                error_count += 1;
                            }
                            if !tip.is_empty() {
                                println!(
                                    "{}\n{}{} {}: {}\n",
                                    "│".bright_red(),
                                    "╰".bright_red(),
                                    ">".yellow(),
                                    "help".yellow(),
                                    tip
                                );
                            } else {
                                println!("\n");
                            }
                        }
                    }
                }
            }
            Err(err) => {
                for error in err {
                    println!("{error}");
                    error_count += 1;
                }
                write_to_file = false;
            }
        }
        line_count += 1;
    }

    if !hlt_seen && error_count == 0 {
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
            if let Err(e) = write_encoded_instructions_to_file(output_file, &encoded_instructions) {
                println!("{}: {}", "error".underline().bright_red().bold(), e);
            }
        }
        _ => {
            if error_count > 0 {
                eprintln!("{}", "compilation unsuccessful".bold());
                if error_count != 1 {
                    eprintln!(
                        "{} {}.",
                        error_count.to_string().bold(),
                        "errors generated".bright_red()
                    );
                } else {
                    eprintln!(
                        "{} {}.",
                        error_count.to_string().bold(),
                        "error generated".bright_red()
                    );
                }
            }
            std::process::exit(1);
        }
    }
}
