use crate::*;
impl Parser<'_> {
    pub fn parse_argument(&self, token: TokenKind) -> Option<InstructionArgument> {
        match token {
            TokenKind::Mem(inner) => Some(InstructionArgument::Mem(inner)),
            TokenKind::Imm(num) | TokenKind::IntLit(num) => Some(InstructionArgument::Imm(num)),
            TokenKind::IReg(reg) => Some(InstructionArgument::IReg(reg)),
            TokenKind::Register(reg) => Some(InstructionArgument::Reg(reg)),
            TokenKind::Ident(ident) => Some(InstructionArgument::Ident(ident)),
            TokenKind::MacroIdent(ident) => Some(InstructionArgument::MacroIdent(ident)),
            TokenKind::Expr(num) => Some(InstructionArgument::Imm(num)),
            _ => None,
        }
    }
}
