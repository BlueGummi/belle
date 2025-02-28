use colored::*;
use std::fmt;
use std::ops::Range;
use term_size::dimensions;

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
