use crate::*;
use colored::*;
use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct MacroValidatorError {
    pub err_file: String,
    pub err_input: String,
    pub err_message: String,
    pub help: Option<String>,
    pub orig_input: String,     // data for original macro loc
    pub orig_pos: Range<usize>, // macro call spot
    pub mac: MacroContent,
}

impl fmt::Display for MacroValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s_pos = self.mac.name.1.start;
        let e_pos = if let Some((_, _, v)) = self.mac.parameters.last() {
            v.end
        } else {
            self.mac.name.1.end
        };

        let m_pos = s_pos..e_pos;
        if self.orig_pos.start >= self.orig_input.len()
            || self.orig_pos.end > self.orig_input.len()
            || self.orig_pos.start >= self.orig_pos.end
        {
            return writeln!(
                f,
                "Error: Indices {}:{} are out of bounds or invalid.",
                self.orig_pos.start, self.orig_pos.end
            );
        }

        let lines: Vec<&str> = self.orig_input.lines().collect();
        print_err_and_line(
            f,
            0,
            (
                "error",
                &self.orig_input,
                &self.err_message,
                &Some(String::from("")),
                &self.err_file,
                &self.orig_pos,
            ),
            lines,
        )?;
        write!(f, "{}", "╮".bright_red())?;
        print_err_and_line(
            f, // fmter
            9, // spaces
            (
                "",                                                       // prelude msg
                &read_file(&self.mac.file),                               // error str
                &format!(" in expansion of macro `{}`", self.mac.name.0), // hint
                &self.help,                                               // help
                &self.mac.file,                                           // filename
                &m_pos,
            ),
            self.err_input.lines().collect(),
        )?;
        Ok(())
    }
}
