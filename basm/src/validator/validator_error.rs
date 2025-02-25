use crate::*;
use colored::*;
use std::fmt;
use std::ops::Range;
use term_size::dimensions;

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
pub fn print_err_and_line(
    f: &mut fmt::Formatter<'_>,
    indents: usize,
    data: (
        &str,
        &String,
        &String,
        &Option<String>,
        &String,
        &Range<usize>,
    ),
    lines: Vec<&str>,
) -> fmt::Result {
    let (title, text, msg, help, file, pos) = data;
    let terminal_width = dimensions().map(|(w, _)| w).unwrap_or(80);

    for (line_number, line) in lines.iter().enumerate() {
        let line_start = text
            .lines()
            .take(line_number)
            .map(|l| l.len() + 1)
            .sum::<usize>();
        let line_end = line_start + line.len();

        if (line_start <= pos.start && pos.start < line_end)
            || (line_start <= pos.end && pos.end < line_end)
        {
            let error_start = pos.start.saturating_sub(line_start);
            let error_end = (pos.end).min(line_end) - line_start;
            let start_spaces = " ".repeat(indents);
            let msg_vec: Vec<String> = msg.lines().map(|l| l.to_string()).collect();
            let mut msg = String::from("");
            for (index, line) in msg_vec.iter().enumerate() {
                if index == 0 {
                    msg = line.to_string();
                } else {
                    msg = format!(
                        "{msg}\n{}{}{line}",
                        "│".bright_red(),
                        " ".repeat(title.len() + 1)
                    );
                }
            }
            if title == "error" {
                writeln!(
                    f,
                    "{start_spaces}{}: {}",
                    title.bright_red().underline(),
                    msg
                )?;
            } else if title.is_empty() {
                writeln!(f, "{}", msg.bold())?;
            } else {
                writeln!(f, "{start_spaces}{}: {}", title.yellow().underline(), msg)?;
            }

            let left_char = if help.is_some() { "├" } else { "╰" };
            writeln!(
                f,
                "{start_spaces}{}{} {}:{}:{}",
                left_char.bright_red(),
                "─".bright_red(),
                file.green(),
                line_number + 1,
                error_start
            )?;

            let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
            let left_char = if help.is_some() { "│" } else { " " };

            if line.len() + 9 > terminal_width {
                let start = error_start.saturating_sub(terminal_width / 2 - 7);
                let end = (error_end + terminal_width / 2 - 7).min(line.len());
                let truncated_line = &line[start..end];

                writeln!(
                    f,
                    "{start_spaces}{}{:^6} {} {}",
                    left_char.bright_red(),
                    (line_number + 1).to_string().blue(),
                    "│".blue(),
                    truncated_line.trim()
                )?;

                let spaces = " ".repeat(error_start - start + 9 - leading_spaces);
                let arrows = "^".repeat(error_end.saturating_sub(error_start));
                write!(
                    f,
                    "{start_spaces}{}{}{}",
                    left_char.bright_red(),
                    spaces,
                    arrows.bright_red()
                )?;
            } else {
                writeln!(
                    f,
                    "{start_spaces}{}{:^6} {} {}",
                    left_char.bright_red(),
                    (line_number + 1).to_string().blue(),
                    "│".blue(),
                    line.trim()
                )?;

                let spaces = " ".repeat(error_start + 9 - leading_spaces);
                let arrows = "^".repeat(error_end.saturating_sub(error_start));
                write!(
                    f,
                    "{start_spaces}{}{}{}",
                    left_char.bright_red(),
                    spaces,
                    arrows.bright_red()
                )?;
            }

            if let Some(s) = &help {
                write!(
                    f,
                    "\n{start_spaces}{}{} {}: {s}",
                    "╰".bright_red(),
                    ">".yellow(),
                    "note".yellow()
                )?;
            }
        }
    }
    Ok(())
}
