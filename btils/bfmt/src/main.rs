use clap::{Arg, Command};
use std::fs;
use std::io::{self, BufRead, Write};

const MAX_INDENTATION: usize = 4;

fn trim_and_format_line(line: &str) -> String {
    let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
    let trimmed_line = if leading_spaces > MAX_INDENTATION {
        &line[leading_spaces..]
    } else {
        line
    }
    .trim_start();

    if trimmed_line.is_empty() {
        return String::new();
    }

    let last_colon = trimmed_line.rfind(':');
    let should_not_trim =
        trimmed_line.starts_with('.') || last_colon.is_some() || trimmed_line.starts_with(';');

    if should_not_trim {
        line.to_string()
    } else {
        format!("{:width$}{}", "", trimmed_line, width = MAX_INDENTATION)
    }
}

fn process_file(filename: &str) -> io::Result<()> {
    let temp_filename = format!("{}.tmp", filename);
    let input_file = fs::File::open(filename)?;
    let reader = io::BufReader::new(input_file);
    let mut output_file = fs::File::create(&temp_filename)?;

    for line in reader.lines() {
        let formatted_line = trim_and_format_line(&line?);
        if !formatted_line.is_empty() {
            writeln!(output_file, "{}", formatted_line)?;
        }
    }

    fs::rename(temp_filename, filename)?;

    Ok(())
}

fn main() {
    let matches = Command::new("BELLE-asm formatter")
        .version("0.2.0")
        .author("BlueGummi")
        .about("Format code written for the BELLE-assembler")
        .arg(
            Arg::new("files")
                .help("The files to format")
                .required(true)
                .num_args(1..),
        )
        .get_matches();

    let files: Vec<&str> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    for filename in files {
        if let Err(e) = process_file(filename) {
            eprintln!("Error processing {}: {}", filename, e);
        }
    }
}
