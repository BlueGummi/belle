use crate::*;
use std::ops::Range;
impl Parser<'_> {
    fn parse_single_macro_argument(
        &mut self,
        arg_name: String,
    ) -> Vec<(String, FullArgument, Range<usize>)> {
        let input_str = self.input.to_string();
        let (val, loc) = match self.lexer.next() {
            Some((v, l)) => (v, l),
            None => return Vec::new(),
        };
        let mut args = Vec::new();
        match val {
            Ok(TokenKind::Ident(arg_type_str)) => {
                let mut leave = false;
                let arg_type = ArgumentType::from_string(&arg_type_str).unwrap_or_else(|| {
                    self.errors.push(ParserError {
                        file: self.file.to_string(),
                        help: Some(String::from("valid argument types are\n         reg, ireg, mem, imem, imm, and label")),
                        input: input_str,
                        message: format!("argument type: {} is not valid", arg_type_str),
                        start_pos: loc.start,
                        last_pos: loc.end,
                    });
                    leave = true;
                    ArgumentType::Reg
                });
                if leave {
                    return args;
                }
                args.push((
                    self.file.to_string(),
                    FullArgument {
                        name: arg_name.to_string(),
                        arg_type,
                    },
                    loc,
                ));
            }
            _ => {
                self.errors.push(ParserError {
                    file: self.file.to_string(),
                    help: Some(String::from("add a type after the ':'")),
                    input: input_str,
                    message: "expected argument type".to_string(),
                    start_pos: loc.start,
                    last_pos: loc.end,
                });
                return args;
            }
        }
        args
    }

    fn parse_macro_arguments(
        &mut self,
        name: (String, Range<usize>),
    ) -> Vec<(String, TokenKind, Range<usize>)> {
        let input_str = self.input.to_string();
        let mut tokens = Vec::new();
        let mut args = Vec::new();
        loop {
            let (val, l) = match self.lexer.next() {
                Some((v, l)) => (v, l),
                None => return tokens,
            };
            match val {
                Ok(TokenKind::Comma) => {
                    continue;
                }
                Ok(TokenKind::Label(arg_name)) => {
                    args.extend(self.parse_single_macro_argument(arg_name));
                }
                Ok(TokenKind::RightParen) => break,
                _ => {
                    self.errors.push(ParserError {
                        file: self.file.to_string(),
                        help: Some(String::from(
                            "macro arguments should go between the '(' ')'",
                        )),
                        input: self.input.to_string(),
                        message: "expected a macro argument".to_string(),
                        start_pos: l.start,
                        last_pos: l.end,
                    });
                    break;
                }
            }
        }
        let (val, loc) = match self.lexer.next() {
            Some((v, l)) => (v, l),
            None => return tokens,
        };
        match val {
            Ok(TokenKind::LeftBrace) => {
                let mut brace_count = 1;
                let mut macro_tokens = Vec::new();

                for (tok, span) in self.lexer.by_ref() {
                    match tok {
                        Ok(TokenKind::LeftBrace) => brace_count += 1,
                        Ok(TokenKind::RightBrace) => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                break;
                            }
                        }
                        Ok(TokenKind::MacroDef(_)) | Ok(TokenKind::MacroCall(_)) => {
                            self.errors.push(ParserError {
                                file: self.file.to_string(),
                                help: None,
                                input: self.input.to_string(),
                                message: "cannot declare or call macro in macro".to_string(),
                                start_pos: span.start,
                                last_pos: span.end,
                            });
                        }
                        Ok(t) => macro_tokens.push((t, span)),
                        _ => {
                            self.errors.push(ParserError {
                                file: self.file.to_string(),
                                help: Some(String::from("close the macro with a '}'")),
                                input: self.input.to_string(),
                                message: "error/reached EOF in macro body".to_string(),
                                start_pos: span.start,
                                last_pos: span.end,
                            });
                        }
                    }
                }
                tokens.push((
                    self.file.to_string(),
                    TokenKind::Macro(MacroContent {
                        full_data: self.input.to_string(),
                        file: self.file.to_string(),
                        name,
                        parameters: args,
                        body: macro_tokens,
                    }),
                    loc,
                ));
            }
            _ => {
                self.errors.push(ParserError {
                    file: self.file.to_string(),
                    help: None,
                    input: input_str,
                    message: "did not find open brace for macro body".to_string(),
                    start_pos: loc.start,
                    last_pos: loc.end,
                });
            }
        }
        tokens
    }

    pub fn parse_single_macro(&mut self) -> Vec<(String, TokenKind, Range<usize>)> {
        let input_str = self.input.to_string();
        let mut tokens = Vec::new();
        let (val, loc) = match self.lexer.next() {
            Some((v, l)) => (v, l),
            None => return tokens,
        };
        let name = if let (Ok(TokenKind::Ident(v)), r) = (val, loc.clone()) {
            (v, r)
        } else {
            self.errors.push(ParserError {
                file: self.file.to_string(),
                help: Some(String::from("add a macro name after the declaration")),
                input: input_str,
                message: "macro name required".to_string(),
                start_pos: loc.start,
                last_pos: loc.end,
            });
            return tokens;
        };

        let (val, loc) = match self.lexer.next() {
            Some((v, l)) => (v, l),
            None => return tokens,
        };
        match val {
            Ok(TokenKind::LeftParen) => {
                tokens.extend(self.parse_macro_arguments(name));
            }
            _ => {
                self.errors.push(ParserError {
                    file: self.file.to_string(),
                    help: None,
                    input: input_str,
                    message: "didn't find open parantheses after macro name".to_string(),
                    start_pos: loc.start,
                    last_pos: loc.end,
                });
            }
        }
        tokens
    }
}
