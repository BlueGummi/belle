use crate::*;
use std::fmt;

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::IncludeFile(name) => write!(f, "include file \"{name}\""),
            TokenKind::Newline => write!(f, "newline"),
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Tab => write!(f, "tab"),
            TokenKind::LeftParen => write!(f, "left parenthesis"),
            TokenKind::RightParen => write!(f, "right parenthesis"),
            TokenKind::LeftBrace => write!(f, "left brace"),
            TokenKind::RightBrace => write!(f, "right brace"),
            TokenKind::Comma => write!(f, "comma"),
            TokenKind::Tilde => write!(f, "tilde"),
            TokenKind::Grave => write!(f, "grave accent"),
            TokenKind::Pound => write!(f, "pound sign"),
            TokenKind::Plus => write!(f, "plus sign"),
            TokenKind::PlusPlus => write!(f, "plus plus"),
            TokenKind::Minus => write!(f, "minus sign"),
            TokenKind::MinusMinus => write!(f, "minus minus"),
            TokenKind::Star => write!(f, "asterisk"),
            TokenKind::Slash => write!(f, "slash"),
            TokenKind::Mod => write!(f, "modulus"),
            TokenKind::Bang => write!(f, "exclamation mark"),
            TokenKind::Equal => write!(f, "equal sign"),
            TokenKind::Greater => write!(f, "greater than sign"),
            TokenKind::GreaterGreater => write!(f, "bitshift right"),
            TokenKind::Less => write!(f, "less than sign"),
            TokenKind::LessLess => write!(f, "bitshift left"),
            TokenKind::LeftBracket => write!(f, "left bracket"),
            TokenKind::RightBracket => write!(f, "right bracket"),
            TokenKind::Amp => write!(f, "ampersand"),
            TokenKind::AmpAmp => write!(f, "double ampersand"),
            TokenKind::Pipe => write!(f, "pipe"),
            TokenKind::PipePipe => write!(f, "double pipe"),
            TokenKind::Xor => write!(f, "caret"),
            TokenKind::Colon => write!(f, "colon"),
            TokenKind::Register(value) => write!(f, "register {} ", value),
            TokenKind::StringLit(value) => write!(f, "string literal(\"{}\")", value),
            TokenKind::IntLit(value) => write!(f, "integer literal({})", value),
            TokenKind::MacroDef(value) => write!(f, "macro definition({})", value),
            TokenKind::Constant(value) => write!(f, "constant({})", value),
            TokenKind::Ident(value) => write!(f, "identifier({})", value),
            TokenKind::Directive(value) => write!(f, "directive({})", value),
            TokenKind::MacroIdent(value) => write!(f, "macro identifier({})", value),
            TokenKind::MacroLabel(value) => write!(f, "macro label({})", value),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::Macro(content) => write!(f, "{}", content),
            TokenKind::Instruction(data) => write!(f, "{}", data),
            TokenKind::Label(value) => write!(f, "label({})", value),
            TokenKind::Mem(token) => write!(f, "memory address\n{}", token),
            TokenKind::IIdent(value) => write!(f, "indirect identifier({})", value),
            TokenKind::IReg(value) => write!(f, "indirect register({})", value),
            TokenKind::Imm(value) => write!(f, "immediate value({})", value),
            TokenKind::Expr(value) => write!(f, "expression value({})", value),
            TokenKind::MacroCall(name) => write!(f, "macro call ({name})"),
        }
    }
}

impl fmt::Display for MemAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "├── Indirect: {}", self.indirect)?;
        for (i, (arg, _)) in self.data.iter().enumerate() {
            if i != self.data.len() - 1 {
                writeln!(f, "    │   ├── {}", arg)?;
            } else {
                write!(f, "    │   └── {}", arg)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for MacroContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Macro: {}", self.name.0)?;
        writeln!(f, "├── Args:")?;
        for (i, (_, arg, _)) in self.parameters.iter().enumerate() {
            if i != self.parameters.len() - 1 {
                writeln!(f, "│   ├── {}", arg)?;
            } else {
                writeln!(f, "│   └── {}", arg)?;
            }
        }
        writeln!(f, "└── Tokens:")?;
        for (i, (token, _)) in self.body.iter().enumerate() {
            if i != self.body.len() - 1 {
                writeln!(f, "    ├── {}", token)?;
            } else {
                write!(f, "    └── {}", token)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for FullArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.arg_type)
    }
}

impl fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgumentType::Mem => write!(f, "memory direct"),
            ArgumentType::Imem => write!(f, "memory indirect"),
            ArgumentType::Ireg => write!(f, "register indirect"),
            ArgumentType::Imm => write!(f, "immediate"),
            ArgumentType::Reg => write!(f, "register"),
            ArgumentType::Label => write!(f, "label"),
        }
    }
}
impl InstructionArgument {
    pub fn get_raw(&self) -> String {
        match self {
            InstructionArgument::Mem(_) => String::from("memory direct"),
            InstructionArgument::Reg(_) => String::from("register"),
            InstructionArgument::IReg(_) => String::from("register indirect"),
            InstructionArgument::Imm(_) => String::from("immediate"),
            InstructionArgument::Ident(_) => String::from("identifier"),
            InstructionArgument::MacroIdent(_) => String::from("macro identifier"),
        }
    }
}
impl fmt::Display for InstructionArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionArgument::Mem(token) => write!(f, "Mem\n    {}", token),
            InstructionArgument::Reg(reg) => write!(f, "Reg({})", reg),
            InstructionArgument::IReg(reg) => write!(f, "IReg({})", reg),
            InstructionArgument::Imm(imm) => write!(f, "Imm({})", imm),
            InstructionArgument::Ident(ident) => write!(f, "Ident({})", ident),
            InstructionArgument::MacroIdent(ident) => write!(f, "MacroIdent({})", ident),
        }
    }
}

impl fmt::Display for InstructionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Instruction: {}, Expanded from macro {}",
            self.name, self.expanded
        )?;
        writeln!(f, "    └─┐ Args:")?;
        for (i, (arg, _)) in self.operands.iter().enumerate() {
            if i != self.operands.len() - 1 {
                writeln!(f, "      ├── {}", arg)?;
            } else {
                write!(f, "    ┌─┴── {}", arg)?;
            }
        }
        Ok(())
    }
}
