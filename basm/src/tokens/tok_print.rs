use crate::*;
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
use prettytable::{format, row, Table};
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
            TokenKind::StringLit(value) => write!(f, "string literal `\"{}\"`", value),
            TokenKind::IntLit(value) => write!(f, "integer literal `{}`", value),
            TokenKind::MacroDef(value) => write!(f, "macro definition `{}`", value),
            TokenKind::Constant(value) => write!(f, "constant `{}`", value),
            TokenKind::Ident(value) => write!(f, "identifier `{}`", value),
            TokenKind::Directive(value) => write!(f, "directive `{}`", value),
            TokenKind::MacroIdent(value) => write!(f, "macro identifier `{}`", value),
            TokenKind::MacroLabel(value) => write!(f, "macro label `{}`", value),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::MultiLineComment => write!(f, "comment"),
            TokenKind::Macro(content) => write!(f, "{}", content),
            TokenKind::Instruction(data) => write!(f, "{}", data),
            TokenKind::Label(value) => write!(f, "label `{}`", value),
            TokenKind::Mem(token) => write!(f, "memory address\n{}", token),
            TokenKind::IIdent(value) => write!(f, "indirect identifier `{}`", value),
            TokenKind::IReg(value) => write!(f, "indirect register `{}`", value),
            TokenKind::Imm(value) => write!(f, "immediate value `{}`", value),
            TokenKind::Expr(value) => write!(f, "expression value `{}`", value),
            TokenKind::MacroCall(name) => write!(f, "macro call `{name}`"),
            TokenKind::CarriageReturn => write!(f, "carriage return"),
        }
    }
}

impl fmt::Display for MemAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table.set_format(get_custom_format());

        table.add_row(row!["Indirect", self.indirect]);
        for (arg, _) in &self.data {
            table.add_row(row!["Data", arg]);
        }
        write!(f, "{}", table)
    }
}

impl fmt::Display for MacroContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table.set_format(get_custom_format());

        table.add_row(row![format!("Macro Name: {}", self.name.0)]);
        if !self.parameters.is_empty() {
            table.add_row(row![]);
            table.add_row(row!["Macro Args"]);
            for (_, arg, _) in &self.parameters {
                table.add_row(row![arg]);
            }
        }
        if !self.body.is_empty() {
            table.add_row(row![]);
            table.add_row(row!["Body Tokens"]);
            for (token, _) in &self.body {
                table.add_row(row![token]);
            }
        }
        write!(f, "{}", table)
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
        let mut table = Table::new();
        table.set_format(get_custom_format());

        table.add_row(row!["Instruction Name", self.name]);
        table.add_row(row!["Was expanded", self.expanded]);
        for (i, (arg, _)) in self.operands.clone().into_iter().enumerate() {
            table.add_row(row![format!("Operand {}", i + 1), arg]);
        }
        write!(f, "{}", table)
    }
}

fn get_custom_format() -> format::TableFormat {
    FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separator(LinePosition::Top, LineSeparator::new('─', '┬', '╭', '╮'))
        .separator(LinePosition::Title, LineSeparator::new('─', '┼', '├', '┤'))
        .separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '╰', '╯'))
        .separator(LinePosition::Intern, LineSeparator::new('─', '┼', '├', '┤'))
        .padding(1, 1)
        .build()
}
