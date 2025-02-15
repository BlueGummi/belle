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

        included_lines.push(content);
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
