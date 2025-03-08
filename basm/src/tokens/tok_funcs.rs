use crate::*;

impl ArgumentType {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "mem" => Some(ArgumentType::Mem),
            "imem" => Some(ArgumentType::Imem),
            "ireg" => Some(ArgumentType::Ireg),
            "imm" => Some(ArgumentType::Imm),
            "reg" => Some(ArgumentType::Reg),
            "ident" | "label" => Some(ArgumentType::Label),
            _ => None,
        }
    }
    pub fn equals(&self, t: TokenKind) -> bool {
        use crate::ArgumentType::*;
        (*self == Mem && t.is_mem())
            || (*self == Reg && t.is_reg())
            || (*self == Ireg && t.is_ireg())
            || (*self == Label && t.is_ident())
    }
}

impl TokenKind {
    pub fn get_str(&self) -> String {
        if let TokenKind::StringLit(s) = self {
            return s.to_string();
        }
        panic!()
    }
    pub fn is_imm(&self) -> bool {
        matches!(self, TokenKind::IntLit(_) | TokenKind::MacroIdent(_))
    }

    pub fn is_mem(&self) -> bool {
        matches!(self, TokenKind::Mem(mem_addr) if !mem_addr.indirect | matches!(self, TokenKind::MacroIdent(_)))
    }

    pub fn is_imem(&self) -> bool {
        matches!(self, TokenKind::Mem(mem_addr) if mem_addr.indirect | matches!(self, TokenKind::MacroIdent(_)))
    }

    pub fn is_reg(&self) -> bool {
        matches!(self, TokenKind::Register(_) | TokenKind::MacroIdent(_))
    }

    pub fn is_ireg(&self) -> bool {
        matches!(self, TokenKind::IReg(_) | TokenKind::MacroIdent(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(
            self,
            TokenKind::Ident(_) | TokenKind::MacroIdent(_) | TokenKind::MacroLabel(_)
        )
    }
}
impl InstructionArgument {
    pub fn get_imm(&self) -> i16 {
        if let InstructionArgument::Imm(v) = self {
            if *v < 0 {
                (1 << 7) | (v.unsigned_abs() as u16 as i16)
            } else {
                *v as i16
            }
        } else {
            panic!("can't get here anyways :P");
        }
    }
    pub fn get_value(&self) -> i64 {
        use crate::InstructionArgument::*;
        match self {
            Reg(v) => *v as i64,
            IReg(v) => *v as i64,
            Imm(v) => *v,
            Ident(s) => {
                let vmap = V_MAP.lock().unwrap();
                if let Some((_, _, v)) = vmap.get(s) {
                    *v
                } else {
                    0
                }
            }
            Mem(m) => {
                if let Some((v, _)) = m.data.first() {
                    v.get_value()
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
    pub fn is_imm(&self) -> bool {
        matches!(
            self,
            InstructionArgument::Imm(_) | InstructionArgument::MacroIdent(_)
        )
    }
    pub fn is_mem(&self) -> bool {
        matches!(self, InstructionArgument::Mem(mem_addr) if !mem_addr.indirect | matches!(self, InstructionArgument::MacroIdent(_)))
    }
    pub fn is_imem(&self) -> bool {
        matches!(self, InstructionArgument::Mem(mem_addr) if mem_addr.indirect | matches!(self, InstructionArgument::MacroIdent(_)))
    }

    pub fn is_reg(&self) -> bool {
        matches!(
            self,
            InstructionArgument::Reg(_) | InstructionArgument::MacroIdent(_)
        )
    }
    pub fn is_ireg(&self) -> bool {
        matches!(
            self,
            InstructionArgument::IReg(_) | InstructionArgument::MacroIdent(_)
        )
    }
    pub fn is_ident(&self) -> bool {
        matches!(
            self,
            InstructionArgument::Ident(_) | InstructionArgument::MacroIdent(_)
        )
    }
}
impl TokenKind {
    pub fn to_tok_kind(&self) -> InstructionArgument {
        use crate::TokenKind::*;
        match self {
            Mem(v) => InstructionArgument::Mem(v.clone()),
            Register(v) => InstructionArgument::Reg(*v),
            IReg(v) => InstructionArgument::IReg(*v),
            IntLit(v) => InstructionArgument::Imm(*v),
            Ident(v) => InstructionArgument::Ident(v.clone()),
            MacroIdent(v) => InstructionArgument::MacroIdent(v.clone()),
            _ => panic!(":3"), // we never call it like this so we good B)))
        }
    }
    pub fn get_value(&self) -> i64 {
        match self {
            TokenKind::IntLit(v) => *v,
            _ => 0,
        }
    }
}

impl InstructionArgument {
    pub fn to_tok_kind(&self) -> TokenKind {
        use crate::InstructionArgument::*;
        match self {
            Mem(v) => TokenKind::Mem(v.clone()),
            Reg(v) => TokenKind::Register(*v),
            IReg(v) => TokenKind::IReg(*v),
            Imm(v) => TokenKind::IntLit(*v),
            Ident(v) => TokenKind::Ident(v.clone()),
            MacroIdent(v) => TokenKind::MacroIdent(v.clone()),
        }
    }
}
