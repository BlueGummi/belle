use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub input: String,
    pub message: String,
    pub help: Option<String>,
    pub file: String,
    pub start_pos: usize, // Start index of the error in the input string
    pub last_pos: usize,  // End index of the error in the input string
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start_pos >= self.input.len()
            || self.last_pos > self.input.len()
            || self.start_pos >= self.last_pos
        {
            return writeln!(
                f,
                "Error: Indices {}:{} are out of bounds or invalid.",
                self.start_pos, self.last_pos
            );
        }
        let pos = self.start_pos..self.last_pos;
        let lines: Vec<&str> = self.input.lines().collect();
        print_err_and_line(
            f,
            0,
            (
                "error",
                self.input.to_string(),
                self.message.to_string(),
                &self.help,
                self.file.to_string(),
                pos,
            ),
            lines,
        )
    }
}
