use crate::*;
use colored::*;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

pub fn process_includes(toks: &mut Vec<(String, TokenKind, Range<usize>)>, error_count: &mut i32) {
    loop {
        let mut included_toks = Vec::new();
        let mut index = 0;
        let mut has_include = false;

        use crate::TokenKind::*;
        #[allow(clippy::explicit_counter_loop)]
        for (fname, element, loc) in toks.iter() {
            if let IncludeFile(file_path) = element {
                has_include = true;
                if *file_path == *fname {
                    handle_core_error(
                        fname,
                        loc,
                        error_count,
                        &format!("cannot include {fname} in itself"),
                        None,
                    );
                    println!("{toks:#?}");
                    break;
                }
                let mut file_data = match File::open(file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        handle_core_error(
                            fname,
                            loc,
                            error_count,
                            &format!("cannot read file \"{}\": {e}", file_path.bold().magenta()),
                            None,
                        );
                        std::process::exit(1);
                    }
                };

                let mut contents = String::new();
                match file_data.read_to_string(&mut contents) {
                    Ok(_) => (),
                    Err(e) => {
                        handle_core_error(
                            fname,
                            loc,
                            error_count,
                            &format!("cannot read file \"{}\": {e}", file_path.bold().magenta()),
                            None,
                        );
                    }
                }
                if let Some(mut parser) = create_parser(file_path, &contents, error_count) {
                    if let Some(tokens) = parse_tokens(&mut parser, &contents, error_count) {
                        for token in tokens.into_iter().rev() {
                            included_toks.insert(index, token);
                        }
                    }
                }
            } else {
                included_toks.push((fname.to_string(), element.clone(), loc.clone()));
            }
            index += 1;
        }

        *toks = included_toks;
        if !has_include {
            break;
        }
    }
    print_errc!(*error_count);
}

pub fn read_file(file_path: &str) -> String {
    let mut file_data = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "{}: cannot open file \"{}\": {e}",
                "error".bright_red(),
                file_path.bold().magenta()
            );
            std::process::exit(1)
        }
    };

    let mut contents = String::new();
    match file_data.read_to_string(&mut contents) {
        Ok(_) => contents,
        Err(e) => {
            println!(
                "{}: cannot read file \"{}\": {e}",
                "error".bright_red(),
                file_path.bold().magenta()
            );
            std::process::exit(1);
        }
    }
}
