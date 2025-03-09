use crate::*;
use colored::*;
use std::ops::Range;

pub fn process_start(toks: &mut Vec<(String, TokenKind, Range<usize>)>, error_count: &mut i32) {
    use crate::TokenKind::*;
    let mut toks_iter = toks.clone().into_iter().peekable();
    let mut start_addr = 100;
    let mut seen_start = false;
    while let Some((fname, tok, span)) = toks_iter.next() {
        if let Directive(data) = tok {
            if data.as_str() == "start" {
                if seen_start {
                    handle_core_error(
                        &fname,
                        &span,
                        error_count,
                        ".start directive can only be declared once",
                        None,
                    );
                    break;
                }
                if let Some((_, TokenKind::IntLit(val), _)) = toks_iter.peek() {
                    start_addr = *val;
                    let mut st_gl = START_LOCATION.lock().unwrap();
                    *st_gl = *val;
                    seen_start = true;
                } else if let Some((_, TokenKind::Mem(addr), _)) = toks_iter.peek() {
                    if let Some((val, _)) = addr.data.first() {
                        start_addr = val.get_value();
                        let mut st_gl = START_LOCATION.lock().unwrap();
                        *st_gl = val.get_value();
                        seen_start = true;
                    } else {
                        handle_core_error(
                            &fname,
                            &span,
                            error_count,
                            ".start directive must be succeeded by memory literal",
                            None,
                        );

                        break;
                    }
                } else {
                    handle_core_error(
                        &fname,
                        &span,
                        error_count,
                        ".start directive must be succeeded by memory address",
                        None,
                    );

                    break;
                }
            }
        }
    }
    process_directives(toks, error_count, start_addr);
    let mut new_toks = Vec::new();
    {
        for (fname, tok, span) in &mut *toks {
            if let TokenKind::Label(_) = tok {
            } else {
                new_toks.push((fname.to_string(), tok.clone(), span.clone()));
            }
        }
    }
    *toks = new_toks;
}

fn process_directives(
    toks: &mut [(String, TokenKind, Range<usize>)],
    error_count: &mut i32,
    start_addr: i64,
) {
    use crate::TokenKind::*;
    let mut toks_iter = toks.iter().cloned().peekable();
    let mut l_map = LABEL_MAP.lock().unwrap();
    let mut loc_counter = start_addr;
    while let Some((fname, tok, span)) = toks_iter.next() {
        match tok {
            Label(name) => {
                if let Some((file, location, _)) = l_map.get(&name) {
                    handle_core_error(
                        &fname,
                        &span,
                        error_count,
                        &format!("label `{}` has already been declared", name.magenta()),
                        Some(format!("{} previous declaration here", "╮".bright_red())),
                    );
                    let (num, data) = highlight_range_in_file(&fname, location);
                    println!(
                        "         {}{} in {} {}{} {:^6} {} {}\n",
                        "╰".bright_red(),
                        ">".yellow(),
                        file.green(),
                        "-".bright_red(),
                        ">".yellow(),
                        num.to_string().blue(),
                        "│".blue(),
                        data
                    );
                } else {
                    l_map.insert(
                        name,
                        (fname.to_string(), span.clone(), loc_counter as usize),
                    );
                }
            }
            Directive(data) => match data.trim() {
                "start" => {
                    if let Some((_, TokenKind::IntLit(_), _)) = toks_iter.peek() {
                        toks_iter.next();
                    } else if let Some((_, TokenKind::Mem(_), _)) = toks_iter.peek() {
                        toks_iter.next();
                    }
                }
                "pad" => {
                    if let Some((_, TokenKind::IntLit(v), _)) = toks_iter.peek() {
                        loc_counter += v;
                        toks_iter.next();
                    } else {
                        handle_core_error(
                            &fname,
                            &span,
                            error_count,
                            ".pad directive must be succeeded by literal",
                            None,
                        );
                        break;
                    }
                }
                "word" => {
                    if toks_iter
                        .peek()
                        .is_some_and(|v| matches!(v.1, TokenKind::Ident(_) | TokenKind::IntLit(_)))
                    {
                        loc_counter += 1;
                        toks_iter.next();
                    } else {
                        handle_core_error(
                            &fname,
                            &span,
                            error_count,
                            &format!("{} must be succeeded by literal or identifier", data.trim()),
                            None,
                        );
                        break;
                    }
                }
                "asciiz" => {
                    if let Some((_, TokenKind::StringLit(val), _)) = toks_iter.peek() {
                        loc_counter += val.len() as i64;
                        toks_iter.next();
                    } else {
                        handle_core_error(
                            &fname,
                            &span,
                            error_count,
                            &format!("{} must be succeeded by string", data.trim()),
                            None,
                        );
                        break;
                    }
                }
                "data" => {
                    if let Some((_, TokenKind::StringLit(val), _)) = toks_iter.peek() {
                        let mut glob_str = METADATA_STR.lock().unwrap();
                        *glob_str = format!("{}{}", glob_str, val);
                        toks_iter.next();
                    } else {
                        handle_core_error(
                            &fname,
                            &span,
                            error_count,
                            &format!("{} must be succeeded by string", data.trim()),
                            None,
                        );
                        break;
                    }
                }
                _ => {
                    handle_core_error(
                        &fname,
                        &span,
                        error_count,
                        &format!("unrecognized directive {data}"),
                        None,
                    );
                    break;
                }
            },
            Instruction(_) => loc_counter += 1,
            Newline | LeftBrace | RightBrace => (),
            _ => {
                handle_core_error(
                    &fname,
                    &span,
                    error_count,
                    &format!("unrecognized {tok}"),
                    None,
                );
                break;
            }
        }
    }
}
