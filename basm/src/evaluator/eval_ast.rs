use std::fmt;
#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),
    BitAnd(Box<Expr>, Box<Expr>),
    BitOr(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::Int(n) => *n,
            Expr::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expr::Sub(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
            Expr::Mul(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Expr::Shl(lhs, rhs) => lhs.evaluate() << rhs.evaluate() as u32,
            Expr::Shr(lhs, rhs) => lhs.evaluate() >> rhs.evaluate() as u32,
            Expr::BitAnd(lhs, rhs) => lhs.evaluate() & rhs.evaluate(),
            Expr::BitOr(lhs, rhs) => lhs.evaluate() | rhs.evaluate(),
            Expr::Xor(lhs, rhs) => lhs.evaluate() ^ rhs.evaluate(),
        }
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_expr(
            expr: &Expr,
            prefix: &str,
            is_tail: bool,
            f: &mut fmt::Formatter<'_>,
        ) -> fmt::Result {
            let (new_prefix, current_prefix) = if is_tail {
                (format!("{}    ", prefix), format!("{}└── ", prefix))
            } else {
                (format!("{}│   ", prefix), format!("{}├── ", prefix))
            };

            match expr {
                Expr::Int(n) => writeln!(f, "{}{}", current_prefix, n),
                Expr::Add(lhs, rhs) => {
                    writeln!(f, "{}+", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
                Expr::Sub(lhs, rhs) => {
                    writeln!(f, "{}-", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
                Expr::Mul(lhs, rhs) => {
                    writeln!(f, "{}*", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
                Expr::Shl(lhs, rhs) => {
                    writeln!(f, "{}<<", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
                Expr::Shr(lhs, rhs) => {
                    writeln!(f, "{}>>", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
                Expr::BitAnd(lhs, rhs) => {
                    writeln!(f, "{}&", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
                Expr::BitOr(lhs, rhs) => {
                    writeln!(f, "{}|", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
                Expr::Xor(lhs, rhs) => {
                    writeln!(f, "{}^", current_prefix)?;
                    format_expr(lhs, &new_prefix, false, f)?;
                    format_expr(rhs, &new_prefix, true, f)
                }
            }
        }

        format_expr(self, "", true, f)
    }
}
