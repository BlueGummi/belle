use crate::*;
use std::ops::Range;
#[derive(Debug, PartialEq, Clone)]
pub struct MemAddr {
    pub indirect: bool,
    pub data: Vec<(TokenKind, Range<usize>)>,
}

impl MemAddr {
    pub fn is_indirect(&self) -> bool {
        self.indirect
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MacroContent {
    pub full_data: String,
    pub file: String,
    pub name: (String, Range<usize>),
    pub parameters: Vec<(String, FullArgument, Range<usize>)>,
    pub body: Vec<(TokenKind, Range<usize>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FullArgument {
    pub name: String,
    pub arg_type: ArgumentType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentType {
    // for macros
    Mem,
    Imem,
    Ireg,
    Imm,
    Reg,
    Label,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionArgument {
    Mem(MemAddr),
    Reg(u8),
    IReg(u8),
    Imm(i64),
    Ident(String),
    MacroIdent(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstructionData {
    pub expanded: bool,
    pub name: String,
    pub operands: Vec<(InstructionArgument, Range<usize>)>,
}
