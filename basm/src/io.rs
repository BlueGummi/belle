use crate::Error::*;
use crate::*;
use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead, Write};
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
pub fn write_encoded_instructions_to_file(
    filename: &str,
    encoded_instructions: &[u8],
) -> io::Result<()> {
    if CONFIG.verbose {
        println!("{}", "Wrote to file.".green());
    }
    let mut file = File::create(filename)?;
    file.write_all(encoded_instructions)?;
    Ok(())
}
