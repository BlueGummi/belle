use crate::Token;

static MMAFAIL: &str = "memory address or address of label too large";
static LITFAIL: &str = "literal value too large";

impl Token {
    pub fn is_register(&self) -> bool {
        matches!(self, Token::Register(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Token::Literal(_))
    }

    pub fn is_memory_address(&self) -> bool {
        matches!(self, Token::MemAddr(_))
    }

    pub fn is_memory_address_pointer(&self) -> bool {
        matches!(self, Token::MemPointer(_))
    }

    pub fn is_register_pointer(&self) -> bool {
        matches!(self, Token::RegPointer(_))
    }

    pub fn is_srcall(&self) -> bool {
        matches!(self, Token::SRCall(_))
    }

    pub fn is_valid_arg(&self) -> bool {
        self.is_register()
            || self.is_literal()
            || self.is_srcall()
            || self.is_memory_address()
            || self.is_memory_address_pointer()
            || self.is_register_pointer()
    }
}

pub fn verify(
    ins: &Token,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    let instructions = [
        "ADD", "HLT", "JO", "POP", "DIV", "RET", "LD", "ST", "JMP", "JZ", "PUSH", "CMP", "MUL",
        "INT", "MOV", "LEA",
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
        if let Some(value) = arg1 {
            if value.get_num() > 511 {
                return Err(format!(
                    "Directive address cannot exceed 511, on line {}",
                    line_num
                ));
            }
        }
    }
    Ok(())
}

fn check_instruction(
    raw_token: &str,
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    line_num: u32,
) -> Result<(), String> {
    match raw_token {
        "HLT" | "RET" => only_none(arg1, arg2, raw_token, line_num),
        "LD" | "LEA" => {
            only_two(arg1, arg2, raw_token, line_num).and_then(|_| ld_args(arg1, arg2, line_num))
        }
        "ST" => {
            only_two(arg1, arg2, raw_token, line_num).and_then(|_| st_args(arg1, arg2, line_num))
        }
        "MOV" | "MUL" | "DIV" | "ADD" | "CMP" => {
            only_two(arg1, arg2, raw_token, line_num).and_then(|_| mov_args(arg1, arg2, line_num))
        }
        "INT" => one_none(arg1, arg2, raw_token, line_num).and_then(|_| int_args(arg1, line_num)),
        "JZ" | "JO" | "JMP" => {
            only_one(arg1, arg2, raw_token, line_num).and_then(|_| jump_args(arg1, line_num))
        }
        "PUSH" | "POP" => {
            only_one(arg1, arg2, raw_token, line_num).and_then(|_| push_args(arg1, line_num))
        }
        _ => Ok(()),
    }
}

fn only_none(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    if arg1.is_some() || arg2.is_some() {
        return Err(format!(
            "{} requires no arguments at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn only_two(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    if arg1.is_none() || arg2.is_none() {
        return Err(format!(
            "{} requires two arguments at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn one_none(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    if arg1.is_some() && arg2.is_some() {
        return Err(format!(
            "{} requires one or no arguments at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn only_one(
    arg1: Option<&Token>,
    arg2: Option<&Token>,
    instruction: &str,
    line_num: u32,
) -> Result<(), String> {
    if arg1.is_none() || arg2.is_some() {
        return Err(format!(
            "{} requires one argument at line {}",
            instruction, line_num
        ));
    }
    Ok(())
}

fn ld_args(arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !arg1.is_some_and(|tok| tok.is_register()) {
        return Err(format!(
            "LD/LEA requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !arg2.is_some_and(|tok| tok.is_memory_address() || tok.is_srcall()) {
        return Err(format!(
            "LD/LEA requires RHS to be a Memory address at line {}",
            line_num
        ));
    }
    if arg2.unwrap().get_num() > 511 {
        return Err(format!("{MMAFAIL} on line {line_num}"));
    }
    Ok(())
}

fn st_args(arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !arg1
        .is_some_and(|tok| tok.is_register_pointer() || tok.is_memory_address() || tok.is_srcall())
    {
        return Err(format!(
            "ST requires LHS to be a Register pointer or Memory address at line {}",
            line_num
        ));
    }
    if !arg2.is_some_and(|tok| tok.is_register()) {
        return Err(format!(
            "ST requires RHS to be a Register at line {}",
            line_num
        ));
    }
    if arg1.unwrap().get_num() > 255 {
        return Err(format!("{MMAFAIL} on line {line_num}"));
    }
    Ok(())
}

fn mov_args(arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !arg1.is_some_and(|tok| tok.is_register()) {
        return Err(format!(
            "MOV requires LHS to be a Register at line {}",
            line_num
        ));
    }
    if !arg2.is_some_and(|tok| {
        tok.is_register()
            || tok.is_literal()
            || tok.is_register_pointer()
            || tok.is_memory_address_pointer()
    }) {
        return Err(format!(
            "MOV requires RHS to be a Register, literal, register pointer, or memory address pointer at line {}",
            line_num
        ));
    }
    match arg2 {
        Some(tok) => {
            if tok.get_num() > 511 {
                if !tok.is_memory_address_pointer() {
                    return Err(LITFAIL.to_string());
                } else {
                    return Err(format!("{MMAFAIL} on line {line_num}"));
                }
            }
        }
        _ => {
            return Ok(());
        }
    }
    Ok(())
}

fn int_args(arg1: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !arg1.is_some_and(|tok| tok.is_literal()) {
        return Err(format!(
            "INT requires SRC to be a Literal at line {}",
            line_num
        ));
    }
    if arg1.unwrap().get_num() > 2047 || arg1.unwrap().get_num() < -1 {
        return Err("invalid interrupt number".to_string());
    }
    Ok(())
}

fn push_args(arg1: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !arg1.is_some_and(|tok| tok.is_memory_address() || tok.is_register() || tok.is_literal()) {
        return Err(format!(
            "PUSH/POP requires SRC to be a Register or Literal or Memory address at line {}",
            line_num
        ));
    }
    match arg1 {
        Some(tok) if tok.is_literal() => {
            if tok.get_num() > 2047 {
                return Err(LITFAIL.to_string());
            }
        }
        _ => (),
    }
    Ok(())
}

fn jump_args(arg1: Option<&Token>, line_num: u32) -> Result<(), String> {
    if !arg1
        .is_some_and(|tok| tok.is_register_pointer() || tok.is_memory_address() || tok.is_srcall())
    {
        return Err(format!(
            "JMP/JZ/JO requires DEST to be a Register pointer, Memory address, or SRCall at line {}",
            line_num
        ));
    }
    match arg1 {
        Some(tok) if tok.get_num() > 2047 => {
            return Err(format!("{MMAFAIL} on line {line_num}"));
        }
        _ => (),
    }
    Ok(())
}
