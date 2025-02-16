use crate::CONFIG;
use crate::*;
use colored::Colorize;
use std::fmt;
pub const HLT_OP: i16 = 0b0000; // we need this
pub const ADD_OP: i16 = 0b0001; // we also need this
pub const BO_OP: i16 = 0b00100; // maybe optional ?
pub const BNO_OP: i16 = 0b00101;
pub const POP_OP: i16 = 0b0011; // maybe optional ?
pub const DIV_OP: i16 = 0b0100; // we need this
pub const RET_OP: i16 = 0b0101; // we need this
pub const BL_OP: i16 = 0b01010;
pub const BG_OP: i16 = 0b01011;
pub const LD_OP: i16 = 0b0110; // we need this
pub const ST_OP: i16 = 0b0111; // we need this
pub const JMP_OP: i16 = 0b10000; // we need this
pub const BZ_OP: i16 = 0b10010; // maybe optional ?
pub const BNZ_OP: i16 = 0b10011;
pub const CMP_OP: i16 = 0b1010; // we need this
pub const NAND_OP: i16 = 0b1011; // we need this
pub const PUSH_OP: i16 = 0b1100; // we need this
pub const INT_OP: i16 = 0b1101; // we need this
pub const MOV_OP: i16 = 0b1110; // we need this
pub const LEA_OP: i16 = 0b1111; // we need this
                                // self explanatory, you got this
#[derive(Debug)]
pub enum Token {
    Ident(String),
    Register(i16),
    Comma,
    Literal(i16),
    NewLine,
    Eol,
    SRCall(String),
    MemAddr(i16),
    Directive(String),
    RegPointer(i16),
    MemPointer(i16),
    EqualSign,
    Asciiz(String),
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Ident(s1), Token::Ident(s2)) => s1 == s2,
            (Token::Register(r1), Token::Register(r2)) => r1 == r2,
            (Token::Comma, Token::Comma) => true,
            (Token::Literal(l1), Token::Literal(l2)) => l1 == l2,
            (Token::NewLine, Token::NewLine) => true,
            (Token::Eol, Token::Eol) => true,
            (Token::SRCall(s1), Token::SRCall(s2)) => s1 == s2,
            (Token::MemAddr(m1), Token::MemAddr(m2)) => m1 == m2,
            (Token::Directive(l1), Token::Directive(l2)) => l1 == l2,
            (Token::RegPointer(rp1), Token::RegPointer(rp2)) => rp1 == rp2,
            (Token::MemPointer(mp1), Token::MemPointer(mp2)) => mp1 == mp2,
            (Token::EqualSign, Token::EqualSign) => true,
            (Token::Asciiz(az1), Token::Asciiz(az2)) => az1 == az2,
            _ => false,
        }
    }
}
impl Token {
    #[must_use]
    pub fn get_raw(&self) -> String {
        match self {
            Token::EqualSign => "equals".to_string(),
            Token::Ident(s) => s.to_string(),
            Token::Register(n) => n.to_string(),
            Token::Comma => "comma".to_string(),
            Token::Literal(n) => n.to_string(),
            Token::NewLine => "newline".to_string(),
            Token::Eol => "eol".to_string(),
            Token::SRCall(s) => s.to_string(),
            Token::MemAddr(n) => n.to_string(),
            Token::Directive(s) => s.to_string(),
            Token::RegPointer(n) => n.to_string(),
            Token::MemPointer(n) => n.to_string(),
            Token::Asciiz(s) => s.to_string(),
        }
    }
    pub fn get_num(&self) -> i16 {
        match self {
            Token::Register(n) => *n,
            Token::Literal(n) => *n,
            Token::MemAddr(n) => *n,
            Token::RegPointer(n) => *n,
            Token::MemPointer(n) => *n,
            Token::SRCall(sr) => {
                let map = LABEL_MAP.lock().unwrap();
                if let Some((_, address)) = map.get(sr) {
                    *address as i16
                } else {
                    0
                }
            }
            Token::Ident(ident) => {
                let map = LABEL_MAP.lock().unwrap();
                let vmap = VARIABLE_MAP.lock().unwrap();
                if let Some((_, address)) = map.get(ident) {
                    *address as i16
                } else if let Some((_, value)) = vmap.get(ident) {
                    *value as i16
                } else {
                    0
                }
            }
            _ => -1,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if CONFIG.verbose {
            match self {
                Token::Ident(s) => {
                    write!(f, "{} (\"{}\") Length: [{}]", "Ident".green(), s, s.len())
                }
                Token::EqualSign => write!(f, "{}", "Equal Sign".cyan()),
                Token::Register(n) => write!(f, "{} ({})", "Register".red(), n),
                Token::Comma => write!(f, "{}", "Comma".blue()),
                Token::Literal(n) => write!(f, "{} ({})", "Number Literal".yellow(), n),
                Token::NewLine => write!(f, "{}", "Newline".magenta()),
                Token::Eol => writeln!(f, "{}", "Eol".cyan()),
                Token::SRCall(s) => write!(f, "{} ({})", "SRCall".purple(), s),
                Token::MemAddr(n) => write!(f, "{} ({})", "MemAddr".bright_red(), n),
                Token::Directive(s) => write!(f, "{} ({})", "Directive".bright_yellow(), s),
                Token::RegPointer(n) => write!(f, "{} ({})", "Reg Pointer".bright_green(), n),
                Token::MemPointer(n) => write!(f, "{} ({})", "Mem Pointer".bold().yellow(), n),
                Token::Asciiz(s) => write!(f, "{} ({})", "Asciiz".bold().green(), s),
            }
        } else {
            Ok(())
        }
    }
}
