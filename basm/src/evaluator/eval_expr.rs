use crate::*;
use std::iter::Peekable;

type Evalex<'a> = Peekable<logos::SpannedIter<'a, tokens::TokenKind>>;

pub fn parse_bitwise(
    file: &String,
    input: &String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut expr = parse_add_sub(file, input, token_iter)?;

    while let Some((token, _)) = token_iter.peek() {
        match token {
            Ok(TokenKind::Amp) => {
                token_iter.next();
                expr = Expr::BitAnd(
                    Box::new(expr),
                    Box::new(parse_add_sub(file, input, token_iter)?),
                );
            }
            Ok(TokenKind::Pipe) => {
                token_iter.next();
                expr = Expr::BitOr(
                    Box::new(expr),
                    Box::new(parse_add_sub(file, input, token_iter)?),
                );
            }
            Ok(TokenKind::Xor) => {
                token_iter.next();
                expr = Expr::Xor(
                    Box::new(expr),
                    Box::new(parse_add_sub(file, input, token_iter)?),
                );
            }
            _ => break,
        }
    }
    Ok(expr)
}

pub fn parse_add_sub(
    file: &String,
    input: &String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut expr = parse_mul_shift(file, input, token_iter)?;

    while let Some((token, _)) = token_iter.peek() {
        match token {
            Ok(TokenKind::Plus) => {
                token_iter.next();
                expr = Expr::Add(
                    Box::new(expr),
                    Box::new(parse_mul_shift(file, input, token_iter)?),
                );
            }
            Ok(TokenKind::Minus) => {
                token_iter.next();
                expr = Expr::Sub(
                    Box::new(expr),
                    Box::new(parse_mul_shift(file, input, token_iter)?),
                );
            }
            _ => break,
        }
    }
    Ok(expr)
}

pub fn parse_mul_shift(
    file: &String,
    input: &String,
    token_iter: &mut Evalex,
) -> Result<Expr, ParserError> {
    let mut expr = parse_primary(file, input, token_iter)?;

    while let Some((token, _)) = token_iter.peek() {
        match token {
            Ok(TokenKind::Star) => {
                token_iter.next();
                expr = Expr::Mul(
                    Box::new(expr),
                    Box::new(parse_primary(file, input, token_iter)?),
                );
            }
            Ok(TokenKind::LessLess) => {
                token_iter.next();
                expr = Expr::Shl(
                    Box::new(expr),
                    Box::new(parse_primary(file, input, token_iter)?),
                );
            }
            Ok(TokenKind::GreaterGreater) => {
                token_iter.next();
                expr = Expr::Shr(
                    Box::new(expr),
                    Box::new(parse_primary(file, input, token_iter)?),
                );
            }
            _ => break,
        }
    }
    Ok(expr)
}
