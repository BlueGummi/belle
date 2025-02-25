use crate::*;
use logos::Logos;
use std::iter::Peekable;
use std::ops::Range;
use std::vec::IntoIter;
type ParsingLexer = Peekable<IntoIter<(Result<TokenKind, ()>, Range<usize>)>>;
type ParserResult<'a> = Result<Vec<(String, TokenKind, Range<usize>)>, &'a Vec<ParserError>>;
pub struct Parser<'a> {
    pub file: String,
    pub lexer: ParsingLexer,
    pub input: &'a str,
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &String, input: &'a str) -> Result<Self, Vec<ParserError>> {
        let errors = Vec::new();
        let lexer = TokenKind::lexer(input).spanned();

        let first_pass_tokens = Self::first_pass(file, &String::from(input), lexer);
        let toks = match first_pass_tokens {
            Err(e) => return Err(e),
            Ok(ref v) => v,
        };
        let second_pass_tokens = Self::second_pass(
            &mut Parser {
                file: file.to_string(),
                lexer: toks.clone().into_iter().peekable(),
                input,
                errors: Vec::new(),
            },
            first_pass_tokens?,
        );
        let toks = match second_pass_tokens {
            Err(e) => return Err(e),
            Ok(ref v) => v,
        };
        Ok(Parser {
            file: file.to_string(),
            lexer: toks.clone().into_iter().peekable(),
            input,
            errors,
        })
    }
    pub fn parse(&mut self) -> ParserResult {
        let mut tokens = Vec::new();

        while let Some((token, span)) = self.lexer.next() {
            match token {
                Ok(TokenKind::Whitespace) | Ok(TokenKind::Tab) => {}
                Ok(TokenKind::MacroDef(_)) => tokens.extend(self.parse_single_macro()),
                Ok(t) => {
                    tokens.push((self.file.to_string(), t, span));
                }
                Err(()) => {
                    self.errors.push(ParserError {
                        file: self.file.to_string(),
                        help: Some(String::from(
                            "this is likely an internal error, please report it",
                        )),
                        input: self.input.to_string(),
                        message: "Unknown error encountered whilst parsing".to_string(),
                        start_pos: span.start,
                        last_pos: span.end,
                    });
                }
            }
        }

        if !self.errors.is_empty() {
            return Err(&self.errors);
        }

        Ok(tokens)
    }
}
pub fn create_parser<'a>(
    file: &'a str,
    input_string: &'a str,
    error_count: &mut i32,
) -> Option<Parser<'a>> {
    if CONFIG.verbose {
        print_msg!("PARSER CREATION");
    }
    match Parser::new(&String::from(file), input_string) {
        Ok(parser) => Some(parser),
        Err(errors) => {
            for error in errors {
                *error_count += 1;
                println!("{error}\n");
            }
            None
        }
    }
}

pub fn parse_tokens(
    parser: &mut Parser,
    _input_string: &str,
    error_count: &mut i32,
) -> Option<Vec<(String, TokenKind, Range<usize>)>> {
    match parser.parse() {
        Ok(tokens) => {
            if CONFIG.verbose {
                print_msg!("INITIAL TOKENS (UNEXPANDED MACROS AND DIRECTIVES)");
                for (_, element, _) in &tokens {
                    println!("{}", element);
                }
            }
            Some(tokens)
        }
        Err(errors) => {
            for error in errors {
                *error_count += 1;
                println!("{error}\n");
            }
            None
        }
    }
}
