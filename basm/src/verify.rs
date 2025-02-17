use crate::Token;
use colored::*;

static MMAFAIL: &str = "memory address or address of label too large";
static LITFAIL: &str = "literal value too large";

impl Token {
    pub fn is_register(&self) -> bool {
        matches!(self, Token::Register(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Token::Literal(_) | Token::Ident(_))
    }

    pub fn is_memory_address(&self) -> bool {
        matches!(self, Token::MemAddr(_))
    }

    pub fn is_memory_address_indirect(&self) -> bool {
        matches!(self, Token::MemPointer(_))
    }

    pub fn is_register_indirect(&self) -> bool {
        matches!(self, Token::RegPointer(_))
    }

    pub fn is_regorptr(&self) -> bool {
        matches!(self, Token::Register(_) | Token::RegPointer(_))
    }

    pub fn is_srcall(&self) -> bool {
        matches!(self, Token::SRCall(_) | Token::Ident(_))
    }

    pub fn is_valid_arg(&self) -> bool {
        self.is_register()
            || self.is_literal()
            || self.is_srcall()
            || self.is_memory_address()
            || self.is_memory_address_indirect()
            || self.is_register_indirect()
    }
}

pub fn verify(
    ins: &Token,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    let instructions = [
        "ADD", "HLT", "BO", "POP", "DIV", "RET", "LD", "ST", "JMP", "BZ", "PUSH", "CMP", "NAND",
        "INT", "MOV", "LEA", "BE", "BNE", "BNZ", "BNO", "BG", "BL",
    ];
    let raw_token = ins.get_raw().to_uppercase();

    if let Token::Ident(_) = ins {
        if instructions.contains(&raw_token.as_str()) {
            return check_instruction(&raw_token, arg1, arg2, line_num);
        }
    } else if let Token::Directive(_) = ins {
        if ins.get_raw() == "word" {
            return Ok(());
        }
    }
    Ok(())
}

fn check_instruction(
    raw_token: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    match raw_token {
        "HLT" | "RET" => only_none(raw_token, arg1, arg2, line_num),
        "LD" | "LEA" => only_two(raw_token, arg1, arg2, line_num)
            .and_then(|_| ld_args(raw_token, arg1, arg2, line_num)),
        "ST" => only_two(raw_token, arg1, arg2, line_num)
            .and_then(|_| st_args(raw_token, arg1, arg2, line_num)),
        "MOV" | "MUL" | "DIV" | "ADD" | "CMP" => only_two(raw_token, arg1, arg2, line_num)
            .and_then(|_| mov_args(raw_token, arg1, arg2, line_num)),
        "INT" => one_none(raw_token, arg1, arg2, line_num)
            .and_then(|_| int_args(raw_token, arg1, line_num)),
        raw_token if raw_token.starts_with('j') || raw_token.starts_with('b') => {
            only_one(raw_token, arg1, arg2, line_num)
                .and_then(|_| jump_args(raw_token, arg1, line_num))
        }
        "PUSH" | "POP" => only_one(raw_token, arg1, arg2, line_num)
            .and_then(|_| push_args(raw_token, arg1, line_num)),
        _ => Ok(()),
    }
}

fn only_none(
    instruction: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if arg1.is_some() || arg2.is_some() {
        return Err((
            line_num,
            format!("{} requires no arguments", instruction.purple().bold(),),
        ));
    }
    Ok(())
}

fn only_two(
    instruction: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if arg1.is_none() || arg2.is_none() {
        return Err((
            line_num,
            format!("{} requires two arguments", instruction.purple().bold(),),
        ));
    }
    Ok(())
}

fn one_none(
    instruction: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if arg1.is_some() && arg2.is_some() {
        return Err((
            line_num,
            format!(
                "{} requires one or no arguments",
                instruction.purple().bold(),
            ),
        ));
    }
    Ok(())
}

fn only_one(
    instruction: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if arg1.is_none() || arg2.is_some() {
        return Err((
            line_num,
            format!("{} requires one argument", instruction.purple().bold(),),
        ));
    }
    Ok(())
}

fn ld_args(
    instruction: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if !arg1.is_some_and(|tok| tok.is_register()) {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a register",
                instruction.purple().bold(),
                "LHS".magenta()
            ),
        ));
    }
    if !arg2.is_some_and(|tok| tok.is_memory_address() || tok.is_srcall()) {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a memory address",
                instruction.purple().bold(),
                "RHS".magenta()
            ),
        ));
    }
    if arg1.unwrap().is_regorptr() && arg1.unwrap().get_num() > 7 {
        return Err((
            line_num,
            format!(
                "{} {} register must be 7 or less",
                instruction.purple().bold(),
                "LHS".magenta()
            ),
        ));
    }
    if arg2.unwrap().get_num() > 511 {
        return Err((line_num, MMAFAIL.to_string()));
    }
    Ok(())
}

fn st_args(
    instruction: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if !arg1
        .is_some_and(|tok| tok.is_register_indirect() || tok.is_memory_address() || tok.is_srcall())
    {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a register indirect or memory address",
                instruction.purple().bold(),
                "LHS".magenta()
            ),
        ));
    }
    if !arg2.is_some_and(|tok| tok.is_register()) {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a register",
                instruction.purple().bold(),
                "RHS".magenta()
            ),
        ));
    }
    if arg2.unwrap().is_regorptr() && arg2.unwrap().get_num() > 7 {
        return Err((
            line_num,
            format!(
                "{} {} register must be 7 or less",
                instruction.purple().bold(),
                "RHS".magenta()
            ),
        ));
    }
    if arg1.unwrap().get_num() > 255 {
        return Err((line_num, MMAFAIL.to_string()));
    }
    Ok(())
}

fn mov_args(
    instruction: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if !arg1.is_some_and(|tok| tok.is_register()) {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a register",
                instruction.purple().bold(),
                "LHS".magenta()
            ),
        ));
    }
    if arg1.unwrap().is_regorptr() && arg1.unwrap().get_num() > 7 {
        return Err((
            line_num,
            format!(
                "{} {} register must be 7 or less",
                instruction.purple().bold(),
                "LHS".magenta()
            ),
        ));
    }

    if !arg2.is_some_and(|tok| {
        tok.is_register()
            || tok.is_literal()
            || tok.is_register_indirect()
            || tok.is_memory_address_indirect()
    }) {
        return Err((line_num, format!("{} requires {} to be a register, literal, register indirect, or memory address indirect", instruction.purple().bold(), "RHS".magenta())));
    }
    if arg2.unwrap().is_regorptr() && arg2.unwrap().get_num() > 9 {
        return Err((
            line_num,
            format!(
                "{} {} register must be 9 or less",
                instruction.purple().bold(),
                "RHS".magenta()
            ),
        ));
    }
    match arg2 {
        Some(tok) => {
            if tok.get_num() > 511 {
                if !tok.is_memory_address_indirect() {
                    return Err((line_num, LITFAIL.to_string()));
                } else {
                    return Err((line_num, MMAFAIL.to_string()));
                }
            }
        }
        _ => {
            return Ok(());
        }
    }
    Ok(())
}

fn int_args(
    instruction: &str,
    arg1: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if !arg1.is_some_and(|tok| tok.is_literal()) {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a Literal",
                instruction.purple().bold(),
                "SRC".magenta()
            ),
        ));
    }
    if arg1.unwrap().get_num() > 2047 || arg1.unwrap().get_num() < -1 {
        return Err((line_num, "invalid interrupt number".to_string()));
    }
    Ok(())
}

fn push_args(
    instruction: &str,
    arg1: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if !arg1.is_some_and(|tok| tok.is_memory_address() || tok.is_register() || tok.is_literal()) {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a register or literal or memory address",
                instruction.purple().bold(),
                "SRC".magenta()
            ),
        ));
    }
    if arg1.unwrap().is_regorptr() && arg1.unwrap().get_num() > 9 {
        return Err((
            line_num,
            format!(
                "{} {} register must be 9 or less",
                instruction.purple().bold(),
                "RHS".magenta()
            ),
        ));
    }
    match arg1 {
        Some(tok) if tok.is_literal() => {
            if tok.get_num() > 2047 {
                return Err((line_num, LITFAIL.to_string()));
            }
        }
        _ => (),
    }
    Ok(())
}

fn jump_args(
    instruction: &str,
    arg1: Option<&Token>,
    line_num: usize,
) -> Result<(), (usize, String)> {
    if !arg1
        .is_some_and(|tok| tok.is_register_indirect() || tok.is_memory_address() || tok.is_srcall())
    {
        return Err((
            line_num,
            format!(
                "{} requires {} to be a register indirect, memory address, or SRCall",
                instruction.purple().bold(),
                "DEST".magenta()
            ),
        ));
    }
    if arg1.unwrap().is_regorptr() && arg1.unwrap().get_num() > 9 {
        return Err((
            line_num,
            format!(
                "{} {} register must be 9 or less",
                instruction.purple().bold(),
                "RHS".magenta()
            ),
        ));
    }
    match arg1 {
        Some(tok) if tok.get_num() > 1023 => {
            return Err((line_num, MMAFAIL.to_string()));
        }
        _ => (),
    }
    Ok(())
}
