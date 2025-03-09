use crate::*;
use colored::*;
use std::ops::Range;

type InsValidatorResult =
    Result<(usize, bool, bool, bool, bool, usize), (Option<Range<usize>>, String, Option<String>)>;

impl InstructionData {
    pub fn valid_args(&self) -> InsValidatorResult {
        match self.name.to_lowercase().as_str() {
            "add" | "mov" | "nand" | "div" | "cmp" => Ok((
                0,
                self.operands
                    .first()
                    .is_some_and(|x| x.0.is_reg() && x.0.get_value() <= 7),
                self.operands.get(1).is_some_and(|expr| {
                    ((expr.0.is_reg() || expr.0.is_ireg()) && expr.0.get_value() <= 9)
                        || (expr.0.is_imm()
                            && expr.0.get_value() <= 255
                            && expr.0.get_value() >= -127)
                        || (expr.0.is_imem() && expr.0.get_value() <= 127)
                }),
                self.operands.len() == 2 && self.operands.first().is_some_and(|x| x.0.is_reg()),
                self.operands.get(1).is_some_and(|expr| {
                    expr.0.is_reg() || expr.0.is_ireg() || expr.0.is_imm() || expr.0.is_imem()
                }),
                2,
            )),
            "jmp" | "bo" | "bno" | "bg" | "bl" | "bz" | "bnz" => Ok((
                1,
                self.operands.first().is_some_and(|a| {
                    a.0.is_ident()
                        || (a.0.is_mem() && a.0.get_value() <= 1023)
                        || (a.0.is_ireg() && a.0.get_value() <= 9)
                }),
                true,
                self.operands.len() == 1
                    && self
                        .operands
                        .first()
                        .is_some_and(|arg| arg.0.is_ident() || arg.0.is_mem() || arg.0.is_ireg()),
                true,
                1,
            )),
            "ret" | "hlt" => Ok((
                2,
                self.operands.is_empty(),
                self.operands.is_empty(),
                self.operands.is_empty(),
                self.operands.is_empty(),
                0,
            )),
            "ld" | "lea" => Ok((
                3,
                self.operands
                    .first()
                    .is_some_and(|x| x.0.is_reg() && x.0.get_value() <= 7),
                self.operands.get(1).is_some_and(|expr| {
                    (expr.0.is_mem() && expr.0.get_value() <= 511) || expr.0.is_ident()
                }),
                self.operands.len() == 2 && self.operands.first().is_some_and(|x| x.0.is_reg()),
                self.operands
                    .get(1)
                    .is_some_and(|expr| expr.0.is_mem() || expr.0.is_ident()),
                2,
            )),
            "st" => Ok((
                4,
                self.operands.first().is_some_and(|x| {
                    (x.0.is_mem() && x.0.get_value() <= 255)
                        || (x.0.is_ireg() && x.0.get_value() <= 9)
                        || x.0.is_ident()
                }),
                self.operands
                    .get(1)
                    .is_some_and(|x| x.0.is_reg() && x.0.get_value() <= 7),
                self.operands.len() == 2
                    && self
                        .operands
                        .first()
                        .is_some_and(|x| x.0.is_mem() || x.0.is_ireg() || x.0.is_ident()),
                self.operands.get(1).is_some_and(|expr| expr.0.is_reg()),
                2,
            )),
            "int" => Ok((
                5,
                self.operands.first().is_some_and(|x| {
                    x.0.is_imm() && x.0.get_value() <= 255 && x.0.get_value() >= -127
                }),
                true,
                self.operands.len() == 1 && self.operands.first().is_some_and(|x| x.0.is_imm()),
                true,
                1,
            )),
            "push" => Ok((
                6,
                self.operands.first().is_some_and(|x| {
                    (x.0.is_imm() && x.0.get_value() <= 255 && x.0.get_value() >= -127)
                        || (x.0.is_reg() && x.0.get_value() <= 9)
                }),
                true,
                self.operands.len() == 1
                    && self
                        .operands
                        .first()
                        .is_some_and(|x| x.0.is_imm() || x.0.is_reg()),
                true,
                1,
            )),
            "pop" => Ok((
                7,
                self.operands.first().is_some_and(|x| {
                    (x.0.is_mem() && x.0.get_value() <= 2047)
                        || (x.0.is_reg() && x.0.get_value() <= 9)
                }),
                true,
                self.operands.len() == 1
                    && self
                        .operands
                        .first()
                        .is_some_and(|x| x.0.is_mem() || x.0.is_reg()),
                true,
                1,
            )),
            _ => Err((
                None,
                format!(
                    "instruction {} does not exist",
                    self.name.to_uppercase().magenta()
                ),
                None,
            )),
        }
    }
}
