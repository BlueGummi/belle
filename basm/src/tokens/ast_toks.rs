use crate::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MemAddr {
    pub indirect: bool,
    pub data: Vec<(TokenKind, std::ops::Range<usize>)>,
}

impl MemAddr {
    pub fn is_indirect(&self) -> bool {
        self.indirect
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct MacroContent {
    pub full_data: String,
    pub file: String,
    pub name: (String, std::ops::Range<usize>),
    pub parameters: Vec<(String, FullArgument, std::ops::Range<usize>)>,
    pub body: Vec<(TokenKind, std::ops::Range<usize>)>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct FullArgument {
    pub name: String,
    pub arg_type: ArgumentType,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ArgumentType {
    // for macros
    Mem,
    Imem,
    Ireg,
    Imm,
    Reg,
    Label,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum InstructionArgument {
    Mem(MemAddr),
    Reg(u8),
    IReg(u8),
    Imm(i64),
    Ident(String),
    MacroIdent(String),
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct InstructionData {
    pub expanded: bool,
    pub name: String,
    pub operands: Vec<(InstructionArgument, std::ops::Range<usize>)>,
}
