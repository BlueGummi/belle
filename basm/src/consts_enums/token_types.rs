use crate::CONFIG;
use colored::Colorize;
use std::fmt;

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
    SR(String),
    MemAddr(i16),
    Label(String),
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
            (Token::SR(s1), Token::SR(s2)) => s1 == s2,
            (Token::MemAddr(m1), Token::MemAddr(m2)) => m1 == m2,
            (Token::Label(l1), Token::Label(l2)) => l1 == l2,
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
            Token::SR(s) => s.to_string(),
            Token::MemAddr(n) => n.to_string(),
            Token::Label(s) => s.to_string(),
            Token::RegPointer(n) => n.to_string(),
            Token::MemPointer(n) => n.to_string(),
            Token::Asciiz(s) => s.to_string(),
        }
    }
    pub fn get_num(&self) -> i16 {
        match *self {
            Token::Register(n) => n,
            Token::Literal(n) => n,
            Token::MemAddr(n) => n,
            Token::RegPointer(n) => n,
            Token::MemPointer(n) => n,
            _ => -1,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if CONFIG.verbose || CONFIG.debug {
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
                Token::SR(s) => write!(f, "{} ({})", "Subroutine".bright_purple(), s),
                Token::MemAddr(n) => write!(f, "{} ({})", "MemAddr".bright_red(), n),
                Token::Label(s) => write!(f, "{} ({})", "Label".bright_yellow(), s),
                Token::RegPointer(n) => write!(f, "{} ({})", "Reg Pointer".bright_green(), n),
                Token::MemPointer(n) => write!(f, "{} ({})", "Mem Pointer".bold().yellow(), n),
                Token::Asciiz(s) => write!(f, "{} ({})", "Asciiz".bold().green(), s),
            }
        } else {
            Ok(())
        }
    }
}
