use crate::*;
use std::iter::Peekable;
type Evalex<'a> = Peekable<logos::SpannedIter<'a, tokens::TokenKind>>;

pub fn parse_expression(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    /*if let Ok(ref d) = v {
        let e = d.evaluate();
        println!("{e}\n{d}");
    }*/
    parse_bitwise(file, input, token_iter)
}

pub fn parse_primary(
    file: String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut last_loc = 0..0;

    if let Some((_, loc)) = token_iter.peek() {
        last_loc = loc.clone();
    }
    if let Some((token, l)) = token_iter.next() {
        last_loc = l.clone();

        match token {
            Ok(TokenKind::IntLit(num)) => Ok(Expr::Int(num)),
            Ok(TokenKind::LeftParen) => {
                let expr = parse_expression(file.to_string(), input.to_string(), token_iter)?;
                if let Some((Ok(TokenKind::RightParen), _)) = token_iter.next() {
                    Ok(expr)
                } else {
                    Err(ParserError {
                        file: file.to_string(),
                        help: Some(String::from("close this parenthesis")),
                        input: input.to_string(),
                        message: "unmatched parenthesis".to_string(),
                        start_pos: last_loc.start,
                        last_pos: last_loc.end,
                    })
                }
            }
            Ok(TokenKind::Ident(val)) => {
                let vmap = V_MAP.lock().unwrap();
                if let Some((_, _, v)) = vmap.get(&val) {
                    let val = Ok(Expr::Int(*v));
                    std::mem::drop(vmap);
                    val
                } else {
                    Err(ParserError {
                        file: file.to_string(),
                        help: None,
                        input: input.to_string(),
                        message: format!("constant with name {val} not found"),
                        start_pos: last_loc.start,
                        last_pos: last_loc.end,
                    })
                }
            }
            Ok(v) => Err(ParserError {
                file: file.to_string(),
                help: Some(String::from(
                    "valid expressions allow operators and constant names",
                )),
                input: input.to_string(),
                message: format!("unexpected {v} in expression"),
                start_pos: last_loc.start,
                last_pos: last_loc.end,
            }),
            _ => Err(ParserError {
                file: file.to_string(),
                help: Some(String::from("there is likely an invalid character")),
                input: input.to_string(),
                message: String::from(
                    "reached an error while parsing expression\nmaybe the expression is invalid?",
                ),
                start_pos: last_loc.start,
                last_pos: last_loc.end,
            }),
        }
    } else {
        Err(ParserError {
            file: file.to_string(),
            help: Some(String::from(
                "could not continue iterating over tokens here",
            )),
            input: input.to_string(),
            message: "unexpected end of expression".to_string(),
            start_pos: last_loc.start,
            last_pos: last_loc.end,
        })
    }
}

pub fn evaluate_expression(
    file: &String,
    input: String,
    token_iter: &mut Evalex,
) -> Result<i64, ParserError> {
    let expr = parse_expression(file.to_string(), input, token_iter)?;
    if CONFIG.verbose {
        print_msg!("BEGINNING AST EXPRESSION EVALUATION\n\nRAW EXPR:\n{expr:?}");
        println!();
        print_msg!("CONSTRUCTED AST");
        println!("{}", expr.evaluate());
        println!("{expr}");
    }
    Ok(expr.evaluate())
}
pub fn parse_expression_after_left_paren(
    file: &str,
    input: String,
    lexer: &mut std::iter::Peekable<logos::SpannedIter<'_, TokenKind>>,
) -> Result<Option<(i64, logos::Span)>, ParserError> {
    let mut peek_iter = lexer.clone();
    while let Some((peek_token, _)) = peek_iter.peek() {
        match peek_token {
            Ok(TokenKind::Newline) => break,
            Ok(TokenKind::Colon) | Ok(TokenKind::LeftBrace) => {
                return Ok(None);
            }
            _ => {
                peek_iter.next();
            }
        }
    }

    let next_token = lexer.peek().cloned();
    match next_token {
        Some((Ok(_), span)) => {
            let value = evaluate_expression(&file.to_string(), input.to_string(), lexer)?;
            return Ok(Some((value, span.clone())));
        }
        Some((Err(_), span)) => {
            return Err(ParserError {
                file: file.to_string(),
                help: Some(String::from(
                    "valid characters are math symbols and constant names",
                )),
                input: input.to_string(),
                message: String::from("invalid character in expression"),
                start_pos: span.start,
                last_pos: span.end,
            });
        }
        None => {}
    }

    Err(ParserError {
        file: file.to_string(),
        help: Some(String::from("expression might be empty")),
        input: input.to_string(),
        message: String::from("failed to parse expression"),
        start_pos: 0,
        last_pos: 0,
    })
}
