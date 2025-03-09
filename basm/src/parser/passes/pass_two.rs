use crate::*;
use std::ops::Range;
type PassResult = Result<Vec<(Result<TokenKind, ()>, Range<usize>)>, Vec<(ParserError, bool)>>;
impl Parser<'_> {
    pub fn second_pass(
        &mut self,
        tokens: Vec<(Result<TokenKind, ()>, Range<usize>)>,
    ) -> PassResult {
        let mut new_tokens = Vec::new();
        let mut token_iter = tokens.clone().into_iter().peekable();
        let mut errors = Vec::new();
        let mut iter_count = 0;
        while let Some((token, span)) = token_iter.next() {
            iter_count += 1;
            match token {
                Ok(TokenKind::MacroCall(m)) => {
                    // macro calls are not instructions
                    new_tokens.push((Ok(TokenKind::MacroCall(m)), span));
                    'mdl: loop {
                        match token_iter.next() {
                            Some((Ok(TokenKind::Newline), l)) => {
                                new_tokens.push((Ok(TokenKind::Newline), l));
                                break 'mdl;
                            }
                            Some((Ok(TokenKind::Ident(ident)), span)) => {
                                if let Some((Ok(TokenKind::Colon), _)) = token_iter.peek() {
                                    let (_, _) = token_iter.next().unwrap();
                                    iter_count += 1;
                                    new_tokens.push((Ok(TokenKind::Label(ident)), span));
                                } else {
                                    new_tokens.push((Ok(TokenKind::Ident(ident)), span));
                                }
                            }
                            Some(v) => new_tokens.push(v),
                            _ => break 'mdl,
                        }
                        iter_count += 1;
                    }
                }

                Ok(TokenKind::Ident(name)) => {
                    let mut has_colon = false;
                    let mut ind = iter_count;
                    while let Some((peek_token, _)) = tokens.get(ind) {
                        match peek_token {
                            Ok(TokenKind::Newline) => break,
                            Ok(TokenKind::Label(_)) => {
                                has_colon = true;
                                break;
                            }
                            Ok(TokenKind::LeftBrace) | Ok(TokenKind::StringLit(_)) => {
                                has_colon = true;
                                break;
                            }
                            _ => {
                                ind += 1;
                            }
                        }
                    }

                    if has_colon {
                        new_tokens.push((Ok(TokenKind::Ident(name)), span));
                    } else {
                        let mut args = Vec::new();
                        while let Some((token, loc)) = token_iter.peek() {
                            match token {
                                Ok(TokenKind::Comma) => {
                                    token_iter.next();
                                    iter_count += 1;
                                }
                                Ok(TokenKind::Newline) => {
                                    break;
                                }
                                Ok(t) => {
                                    if let Some(v) = self.parse_argument(t) {
                                        args.push((v, loc.clone()));
                                    }
                                    token_iter.next();
                                    iter_count += 1;
                                }
                                _ => {
                                    token_iter.next();
                                    iter_count += 1;
                                }
                            }
                        }
                        let ins = InstructionData {
                            expanded: false,
                            name: name.to_string(),
                            operands: args.clone(),
                            location: span.clone(),
                        };
                        if let Err(f) = ins.is_valid() {
                            let start = match f.0 {
                                Some(ref v) => v.start,
                                None => span.start,
                            };
                            let end = match f.0 {
                                Some(ref v) => v.end,
                                None => span.end,
                            };
                            errors.push((
                                ParserError {
                                    file: self.file.to_string(),
                                    help: f.2,
                                    input: self.input.to_string(),
                                    message: f.1.to_string(),
                                    start_pos: start,
                                    last_pos: end,
                                },
                                false,
                            ));
                            return Err(errors);
                        }
                        if args.len() > 3 {
                            errors.push((
                                ParserError {
                                    file: self.file.to_string(),
                                    help: None,
                                    input: self.input.to_string(),
                                    message: format!(
                                        "instructions cannot have {} arguments",
                                        args.len()
                                    ),
                                    start_pos: span.start,
                                    last_pos: span.end,
                                },
                                false,
                            ));
                        }
                        new_tokens.push((
                            Ok(TokenKind::Instruction(InstructionData {
                                expanded: false,
                                name,
                                operands: args,
                                location: span.clone(),
                            })),
                            span,
                        ));
                    }
                }
                _ => {
                    new_tokens.push((token, span));
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(new_tokens)
    }
}
