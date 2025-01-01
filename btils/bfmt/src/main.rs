use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::{self, BufRead, Write};

const DEFAULT_MAX_INDENTATION: usize = 4;

fn trim_and_format_line(line: &str, max_indentation: usize, use_tabs: bool) -> String {
    let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
    let trimmed_line = if leading_spaces > max_indentation {
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
        let indent = if use_tabs {
            "\t"
        } else {
            &" ".repeat(max_indentation)
        };
        format!("{}{}", indent, trimmed_line)
    }
}

fn process_file(filename: &str, max_indentation: usize, use_tabs: bool) -> io::Result<()> {
    let temp_filename = format!("{}.tmp", filename);
    let input_file = fs::File::open(filename)?;
    let reader = io::BufReader::new(input_file);
    let mut output_file = fs::File::create(&temp_filename)?;

    for line in reader.lines() {
        let formatted_line = trim_and_format_line(&line?, max_indentation, use_tabs);
        if !formatted_line.is_empty() {
            writeln!(output_file, "{}", formatted_line)?;
        }
    }

    fs::rename(temp_filename, filename)?;

    Ok(())
}

fn main() {
    let mut command = Command::new("bfmt")
        .version("0.2.0")
        .author("BlueGummi")
        .about("Format code written for the BELLE-assembler")
        .arg(Arg::new("files").help("The files to format").num_args(1..))
        .arg(
            Arg::new("max-indent")
                .short('I')
                .long("max-indent")
                .value_name("INDENTATION")
                .help("Set the maximum indentation level")
                .default_value("4")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("tabs")
                .short('t')
                .value_name("USE TABS")
                .action(ArgAction::SetTrue)
                .long("tabs")
                .help("Use tabs for indentation"),
        );

    let matches = command.clone().get_matches();

    if matches.get_many::<String>("files").is_none() {
        command.print_help().unwrap();
        std::process::exit(0);
    }

    let files: Vec<&str> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let max_indentation: usize = *matches.get_one::<usize>("max-indent").unwrap();
    let use_tabs = matches.get_flag("tabs");

    if use_tabs && max_indentation != DEFAULT_MAX_INDENTATION {
        eprintln!("Error: Cannot specify both max-indent and use tabs.");
        return;
    }

    for filename in files {
        if let Err(e) = process_file(filename, max_indentation, use_tabs) {
            eprintln!("Error processing {}: {}", filename, e);
        }
    }

    // Exit with code 0 after processing
    std::process::exit(0);
}
