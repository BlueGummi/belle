use basm::*;
use colored::*;
use std::fs::File;
use std::io::{self, Write};
use std::ops::Range;

fn main() {
    let input_string = read_file(&CONFIG.source);

    let file = &CONFIG.source;
    let mut error_count = 0;

    if CONFIG.verbose {
        print_msg!("RAW INPUT");
        println!("{input_string}");
    }

    let mut parser = match create_parser(file, &input_string, &mut error_count) {
        Some(parser) => parser,
        None => std::process::exit(1),
    };
    print_errc!(error_count);

    let mut toks = match parse_tokens(&mut parser, &input_string, &mut error_count) {
        Some(tokens) => tokens,
        None => std::process::exit(1),
    };
    print_errc!(error_count);

    process_includes(&mut toks, &mut error_count);

    process_macros(&mut toks, &mut error_count);

    process_start(&mut toks, &mut error_count);

    if CONFIG.verbose {
        print_msg!("COMPLETE TOKENS");
        for (_, f, _) in &toks {
            println!("{f}");
        }
    }
    print_errc!(error_count);
    let l_map = LABEL_MAP.lock().unwrap();
    std::mem::drop(l_map);
    print_errc!(error_count);
    let toks: Vec<(String, TokenKind, Range<usize>)> = toks
        .clone()
        .into_iter()
        .filter(|(_, x, _)| !matches!(x, TokenKind::Newline))
        .collect();
    // Code should be valid when this point is reached
    // we can insert panics (maybe?) to reduce code
    let mut binary = Vec::new();
    let mut ind = 0;
    #[allow(clippy::explicit_counter_loop)]
    for (fname, tok, span) in &toks {
        // we should only have instructions at this point
        match encode((fname, tok, span), fname, &toks.get(ind + 1)) {
            Ok(value) => binary.extend(value),
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
                            "─".bright_red(),
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
                print_errc!(error_count);
            }
        }
        ind += 1;
    }
    print_errc!(error_count);
    match &CONFIG.output {
        Some(path) => {
            let mut bytes: Vec<u8> = Vec::new();
            for value in &binary {
                bytes.extend_from_slice(&value.to_be_bytes());
            }

            let start_bin = *START_LOCATION.lock().unwrap();
            if !CONFIG.thin {
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
            gen_ice!("BINARY APPEARS EMPTY - SHOULD BE SET TO `a.out` BY DEFAULT");
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
