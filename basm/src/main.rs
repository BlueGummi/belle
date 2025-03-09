use basm::*;
use colored::*;
use std::fs::File;
use std::io::{self, Write};
use std::ops::Range;
use std::sync::{Arc, Mutex};

fn main() {
    let file = &CONFIG.source.clone().unwrap_or_else(|| "stdin".to_string());
    let fname = &CONFIG.source.clone().unwrap_or_else(|| "stdin".to_string());

    if CONFIG.repl {
        println!("Welcome to the basm-REPL!");
        let prompt = "repl".green();
        let mut input_string = String::new();
        let mut fullstr = String::new();
        let mut indicator = ">".green();
        let temp_file = Arc::new(Mutex::new(Some(
            TempFile::new().expect("Failed to create temporary file"),
        )));

        {
            let temp_file = Arc::clone(&temp_file);
            ctrlc::set_handler(move || {
                if let Some(temp) = temp_file.lock().unwrap().take() {
                    drop(temp);
                }
                println!("\nExiting...");
                std::process::exit(0);
            })
            .expect("Error setting Ctrl-C handler");
        }

        loop {
            print!("{prompt}{} ", indicator);
            io::stdout().flush().unwrap();

            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            let command = input_string.trim();

            if command.is_empty() {
                indicator = ">".green();
                continue;
            }

            if command.ends_with('{') {
                let mut block = String::new();
                block.push_str(&input_string);

                loop {
                    print!(".... ");
                    io::stdout().flush().unwrap();

                    input_string.clear();
                    io::stdin().read_line(&mut input_string).unwrap();
                    block.push_str(&input_string);

                    if input_string.trim().ends_with('}') {
                        break;
                    }
                }

                input_string = block;
            }

            let temp_path = {
                let temp_file_guard = temp_file.lock().unwrap();
                if let Some(ref temp) = *temp_file_guard {
                    temp.path.to_owned()
                } else {
                    eprintln!("Error: Temporary file has been deleted.");
                    break;
                }
            };
            if input_string.trim() == ":st" {
                print_symbol_tables();
                continue;
            }
            fullstr.push_str(&input_string);
            let mut file = File::create(&temp_path).expect("Failed to open temporary file");
            file.write_all(fullstr.as_bytes())
                .expect("Failed to write to temporary file");

            let mut error_count = 0;

            let mut parser = match create_parser(fname, &input_string, &mut error_count) {
                Some(parser) => parser,
                None => {
                    indicator = "x".red();
                    continue;
                }
            };

            let mut toks = match parse_tokens(&mut parser, &input_string, &mut error_count) {
                Some(tokens) => tokens,
                None => {
                    indicator = "x".red();
                    continue;
                }
            };

            process_includes(&mut toks, &mut error_count);
            process_macros(&mut toks, &mut error_count);
            process_start(&mut toks, &mut error_count);

            if error_count > 0 {
                indicator = "x".red();
                continue;
            }

            let toks: Vec<(String, TokenKind, Range<usize>)> = toks
                .clone()
                .into_iter()
                .filter(|(_, x, _)| !matches!(x, TokenKind::Newline))
                .collect();

            let mut binary = Vec::new();
            let vecref = &toks;
            for (ind, (fname, tok, span)) in vecref.iter().enumerate() {
                match encode((fname, tok, span), fname, &toks.get(ind + 1)) {
                    Ok(value) => {
                        binary.extend(value);
                    }
                    Err((m, similars)) => {
                        println!("{m}");
                        if !similars.is_empty() {
                            let size = similars.len() - 1;
                            let max_filename_length = similars
                                .iter()
                                .map(|(filename, _)| filename.len())
                                .max()
                                .unwrap_or(0);
                            for (index, (filename, location)) in similars.into_iter().enumerate() {
                                let (l_num, data) = highlight_range_in_file(&filename, &location);
                                let connector = if index != size { "├" } else { "╰" };
                                println!(
                                    "         {}{} in {:<width$} {}{} {:^6} {} {}",
                                    connector.bright_red(),
                                    ">".yellow(),
                                    filename.green(),
                                    "-".bright_red(),
                                    ">".yellow(),
                                    l_num.to_string().blue(),
                                    "│".blue(),
                                    data,
                                    width = max_filename_length,
                                );
                            }
                            println!();
                        }
                        error_count += 1;
                    }
                }
            }

            if error_count > 0 {
                indicator = "x".red();
                continue;
            }

            for byte in &binary {
                println!("{:016b}", byte);
            }
            indicator = ">".green();
        }
    }

    let input_string = read_file(&CONFIG.source.clone().unwrap_or_else(|| "stdin".to_string()));

    let mut error_count = 0;

    if CONFIG.verbose {
        print_msg!("RAW INPUT");
        println!("{input_string}");
    }

    let mut parser = match create_parser(file, &input_string, &mut error_count) {
        Some(parser) => parser,
        None => {
            print_errc!(error_count);
            std::process::exit(1);
        }
    };

    let mut toks = match parse_tokens(&mut parser, &input_string, &mut error_count) {
        Some(tokens) => tokens,
        None => {
            print_errc!(error_count);
            std::process::exit(1);
        }
    };

    process_includes(&mut toks, &mut error_count);
    process_macros(&mut toks, &mut error_count);
    process_start(&mut toks, &mut error_count);

    if CONFIG.verbose {
        print_msg!("COMPLETE TOKENS");
        for (_, f, _) in &toks {
            println!("{f}");
        }
    }

    let toks: Vec<(String, TokenKind, Range<usize>)> = toks
        .clone()
        .into_iter()
        .filter(|(_, x, _)| !matches!(x, TokenKind::Newline))
        .collect();

    let mut binary = Vec::new();
    if toks.is_empty() {
        println!(
            "{}: {} appears empty",
            "warning".yellow().underline(),
            CONFIG
                .source
                .clone()
                .unwrap_or_else(|| "stdin".to_string())
                .green()
        );
    }

    let vecref = &toks;
    for (ind, (fname, tok, span)) in vecref.iter().enumerate() {
        match encode((fname, tok, span), fname, &toks.get(ind + 1)) {
            Ok(value) => {
                binary.extend(value);
            }
            Err((m, similars)) => {
                println!("{m}");
                if !similars.is_empty() {
                    let size = similars.len() - 1;
                    let max_filename_length = similars
                        .iter()
                        .map(|(filename, _)| filename.len())
                        .max()
                        .unwrap_or(0);
                    for (index, (filename, location)) in similars.into_iter().enumerate() {
                        let (l_num, data) = highlight_range_in_file(&filename, &location);
                        let connector = if index != size { "├" } else { "╰" };
                        println!(
                            "         {}{} in {:<width$} {}{} {:^6} {} {}",
                            connector.bright_red(),
                            ">".yellow(),
                            filename.green(),
                            "-".bright_red(),
                            ">".yellow(),
                            l_num.to_string().blue(),
                            "│".blue(),
                            data,
                            width = max_filename_length,
                        );
                    }
                    println!();
                }
                error_count += 1;
            }
        }
    }

    print_errc!(error_count);

    if CONFIG.verbose {
        print_msg!("SYMBOL TABLES");
        print_symbol_tables();
    }

    match &CONFIG.output {
        Some(path) => {
            let mut bytes: Vec<u8> = Vec::new();
            for value in &binary {
                bytes.extend_from_slice(&value.to_be_bytes());
            }

            let start_bin = *START_LOCATION.lock().unwrap();
            if !CONFIG.thin {
                let glob_str = METADATA_STR.lock().unwrap();
                if glob_str.len() % 2 != 0 {
                    bytes.insert(0, 0);
                }
                for character in glob_str.chars().rev() {
                    bytes.insert(0, character as u8);
                }
                let strlen = if glob_str.len() % 2 == 0 {
                    glob_str.len()
                } else {
                    glob_str.len() + 1
                };
                bytes.insert(0, (strlen & 0xff) as u8);
                bytes.insert(0, ((strlen & 0xff00) >> 8) as u8);
                bytes.insert(0, (start_bin & 0xff) as u8);
                bytes.insert(0, ((start_bin & 0xff00) >> 8) as u8);
                bytes.insert(0, 0x02);
                bytes.insert(0, 0x01);
            }
            match write_bytes_to_file(path, &bytes) {
                Ok(()) => (),
                Err(e) => {
                    eprintln!("{}: {e}", "error writing to output".bright_red());
                    error_count += 1;
                    print_errc!(error_count);
                }
            }
        }
        _ => {
            gen_ice!("BINARY NAME APPEARS EMPTY - SHOULD BE SET TO `a.out` BY DEFAULT");
        }
    }
}

pub fn write_bytes_to_file(filename: &str, encoded_instructions: &[u8]) -> io::Result<()> {
    if CONFIG.verbose {
        println!("{}", "wrote to file.".green());
    }
    let mut file = File::create(filename)?;
    file.write_all(encoded_instructions)?;
    Ok(())
}
