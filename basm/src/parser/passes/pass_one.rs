use crate::*;
use colored::*;
use std::ops::Range; // chain
type PassResult = Result<Vec<(Result<TokenKind, ()>, Range<usize>)>, Vec<(ParserError, bool)>>;
impl<'a> Parser<'a> {
    pub fn first_pass(
        file: &String,
        input: &String,
        lexer: logos::SpannedIter<'a, TokenKind>,
    ) -> PassResult {
        let mut tokens = Vec::new();
        let mut lexer = lexer.peekable();
        let mut errors = Vec::new();
        let mut const_names = Vec::new();
        let mut prev_was_const = false;
        let mut saw_amp = false;
        let mut cspan = 0..0; // (C)onstant span
        while let Some((token, span)) = lexer.next() {
            match token {
                Ok(TokenKind::Amp) => saw_amp = true,
                Ok(TokenKind::MacroDef(m)) => {
                    saw_amp = false;
                    // this here to make sure leftparen doesn't
                    // accidentally start reading a macro
                    prev_was_const = false;
                    tokens.push((Ok(TokenKind::MacroDef(m)), span));
                    'mdl: loop {
                        match lexer.next() {
                            Some((Ok(TokenKind::LeftBrace), l)) => {
                                tokens.push((Ok(TokenKind::LeftBrace), l));
                                break 'mdl;
                            }
                            Some((Ok(TokenKind::Ident(ident)), span)) => {
                                if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                                    let (_, _) = lexer.next().unwrap();
                                    tokens.push((Ok(TokenKind::Label(ident)), span));
                                } else {
                                    tokens.push((Ok(TokenKind::Ident(ident)), span));
                                }
                            }
                            Some(v) => tokens.push(v),
                            _ => break 'mdl,
                        }
                    }
                }
                Ok(TokenKind::Constant(name)) => {
                    saw_amp = false;
                    const_names.push(name);
                    prev_was_const = true;
                    cspan = span.clone();
                    if let Some((Ok(TokenKind::Equal), _)) = lexer.peek() {
                        lexer.next();
                    } else {
                        errors.push((
                            ParserError {
                                file: file.to_string(),
                                help: None,
                                input: input.to_string(),
                                message: String::from(
                                    "constant requires equal sign to denote assignment",
                                ),
                                start_pos: span.start,
                                last_pos: span.end,
                            },
                            false,
                        ));
                    }
                }
                Ok(TokenKind::Ident(ident)) => {
                    saw_amp = false;
                    if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                        let (_, _) = lexer.next().unwrap();
                        tokens.push((Ok(TokenKind::Label(ident)), span));
                    } else {
                        tokens.push((Ok(TokenKind::Ident(ident)), span));
                    }
                    prev_was_const = false;
                }
                Ok(TokenKind::IntLit(v)) => {
                    saw_amp = false;
                    if prev_was_const {
                        if let Some(n) = const_names.pop() {
                            check_vmap(span, file, input, &n)?;
                            let mut vmap = V_MAP.lock().unwrap();
                            vmap.insert(n, (file.to_string(), cspan.clone(), v));
                            std::mem::drop(vmap);
                        } else {
                            errors.push((
                                ParserError {
                                    file: file.to_string(),
                                    help: None,
                                    input: input.to_string(),
                                    message: String::from(
                                        "could not find associated constant for literal",
                                    ),
                                    start_pos: span.start,
                                    last_pos: span.end,
                                },
                                false,
                            ));
                        }
                    } else {
                        tokens.push((Ok(TokenKind::IntLit(v)), span));
                    }
                    prev_was_const = false;
                }
                Ok(TokenKind::LeftBracket) => {
                    // memory addresses are also not instructions
                    prev_was_const = false;
                    let mut addr_toks = Vec::new();
                    'mdl: loop {
                        match lexer.next() {
                            // let's try to do math in it
                            Some((Ok(TokenKind::LeftParen), span)) => {
                                match parse_expression_after_left_paren(file, input, &mut lexer) {
                                    Ok(Some((value, new_span))) => {
                                        addr_toks.push((TokenKind::IntLit(value), new_span));
                                    }
                                    Ok(None) => {
                                        addr_toks.push((TokenKind::LeftParen, span));
                                        break 'mdl;
                                    }
                                    Err(e) => {
                                        errors.push((e, false));
                                    }
                                }
                            }
                            Some((Ok(TokenKind::RightBracket), _)) => {
                                break 'mdl;
                            }
                            Some((Ok(TokenKind::Ident(ident)), span)) => {
                                if let Some((Ok(TokenKind::Colon), _)) = lexer.peek() {
                                    let (_, _) = lexer.next().unwrap();
                                    addr_toks.push(((TokenKind::Label(ident)), span));
                                } else {
                                    addr_toks.push(((TokenKind::Ident(ident)), span));
                                }
                            }
                            Some((Ok(TokenKind::RightParen), _)) => (),
                            Some((Ok(v), span)) => addr_toks.push((v, span)),
                            _ => break 'mdl,
                        }
                    }
                    tokens.push((
                        Ok(TokenKind::Mem(MemAddr {
                            indirect: saw_amp,
                            data: addr_toks,
                        })),
                        span,
                    ));
                    saw_amp = false;
                }
                Ok(TokenKind::LeftParen) => 'lpn: {
                    saw_amp = false;
                    match parse_expression_after_left_paren(file, input, &mut lexer) {
                        Ok(Some((value, new_span))) => {
                            if prev_was_const {
                                if let Some(n) = const_names.pop() {
                                    check_vmap(span, file, input, &n)?;
                                    let mut vmap = V_MAP.lock().unwrap();
                                    vmap.insert(n, (file.to_string(), cspan.clone(), value));
                                    std::mem::drop(vmap);
                                } else {
                                    errors.push((
                                        ParserError {
                                            file: file.to_string(),
                                            help: None,
                                            input: input.to_string(),
                                            message: String::from(
                                                "could not find associated constant name",
                                            ),
                                            start_pos: new_span.start,
                                            last_pos: new_span.end,
                                        },
                                        false,
                                    ));
                                }
                            } else {
                                tokens.push((Ok(TokenKind::IntLit(value)), new_span));
                            }
                        }
                        Ok(None) => {
                            tokens.push((Ok(TokenKind::LeftParen), span));
                            break 'lpn;
                        }
                        Err(e) => {
                            errors.push((e, false));
                        }
                    }
                    if let Some((Ok(TokenKind::RightParen), _)) = lexer.peek() {
                        lexer.next();
                    }
                    prev_was_const = false;
                }
                _ => {
                    saw_amp = false;
                    tokens.push((token, span));
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tokens)
    }
}

fn check_vmap(
    span: Range<usize>,
    file: &String,
    input: &String,
    n: &str,
) -> Result<(), Vec<(ParserError, bool)>> {
    let vmap = V_MAP.lock().unwrap();
    let mut errors = Vec::new();
    if let Some((f, s, _)) = vmap.get(n) {
        errors.push((
            ParserError {
                file: file.to_string(),
                help: Some(format!("{} previous declaration here", "╮".bright_red())),
                input: input.to_string(),
                message: format!("constant {n} was declared twice"),
                start_pos: span.start,
                last_pos: span.end,
            },
            true,
        ));
        errors.push((
            ParserError {
                file: f.to_string(),
                help: None,
                input: read_file(f),
                message: String::new(),
                start_pos: s.start,
                last_pos: s.end,
            },
            false,
        ));
        std::mem::drop(vmap);
    }
    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(())
    }
}
