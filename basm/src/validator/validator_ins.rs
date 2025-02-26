use crate::*;
use colored::*;
use std::ops::Range;

impl InstructionData {
    // location         // msg
    pub fn is_valid(&self) -> Result<(), (Option<Range<usize>>, String)> {
        // Ident is for matching labels - they will be memory addresses
        // Boolean lambda bonanza begins
        let lhs_val = match self.operands.first() {
            Some(v) => v.0.get_value().to_string(),
            None => String::from("none"),
        };
        let rhs_val = match self.operands.get(1) {
            Some(v) => v.0.get_value().to_string(),
            None => String::from("none"),
        };

        let (ins_class, ok_lhs, ok_rhs, valid_lhs, valid_rhs) =
            match self.name.to_lowercase().as_str() {
                "add" | "mov" | "nand" | "div" | "cmp" => (
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
                ),
                "jmp" | "bo" | "bno" | "bg" | "bl" | "bz" | "bnz" => (
                    1,
                    self.operands.first().is_some_and(|a| {
                        a.0.is_ident()
                            || (a.0.is_mem() && a.0.get_value() <= 1023)
                            || (a.0.is_ireg() && a.0.get_value() <= 9)
                    }),
                    true,
                    self.operands.len() == 1
                        && self.operands.first().is_some_and(|arg| {
                            arg.0.is_ident() || arg.0.is_mem() || arg.0.is_ireg()
                        }),
                    true,
                ),
                "ret" | "hlt" => (
                    2,
                    self.operands.is_empty(),
                    self.operands.is_empty(),
                    self.operands.is_empty(),
                    self.operands.is_empty(),
                ),
                "ld" | "lea" => (
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
                ),
                "st" => (
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
                ),
                "int" => (
                    5,
                    self.operands.first().is_some_and(|x| {
                        x.0.is_imm() && x.0.get_value() <= 255 && x.0.get_value() >= -127
                    }),
                    true,
                    self.operands.len() == 1 && self.operands.first().is_some_and(|x| x.0.is_imm()),
                    true,
                ),
                "push" => (
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
                ),
                "pop" => (
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
                ),
                _ => {
                    return Err((
                        None,
                        format!("instruction {} does not exist", self.name.to_uppercase()),
                    ))
                }
            };
        let ok_val = ok_lhs && ok_rhs;
        let valid_args = valid_lhs && valid_rhs;
        let lhs = if let Some((v, _)) = self.operands.first() {
            v.get_raw()
        } else {
            String::from("no")
        };
        let rhs = if let Some((v, _)) = self.operands.get(1) {
            v.get_raw()
        } else {
            String::from("no")
        };

        if valid_args && ok_val {
            return Ok(());
        }
        let span = if !ok_lhs || !valid_lhs {
            self.operands.first().map(|v| v.1.clone())
        } else if !ok_rhs || !valid_rhs {
            self.operands.get(1).map(|v| v.1.clone())
        } else {
            None
        };
        let ovfm = if !ok_lhs && !ok_rhs && valid_args {
            format!(
                "{}: max LHS for {} is {}, max RHS is {}\n{}: found LHS and RHS values are {} and {}\n ",
                "value overflow".bold(),
                self.name.to_uppercase().magenta(),
                LHS_MAXES[ins_class], RHS_MAXES[ins_class], "note".yellow(), lhs_val, rhs_val
            )
        } else if !ok_lhs && valid_args {
            format!(
                "{}: max LHS for {} is {}\n{}: found LHS is {}\n ",
                "value overflow".bold(),
                self.name.to_uppercase().magenta(),
                LHS_MAXES[ins_class],
                "note".yellow(),
                lhs_val
            )
        } else if !ok_rhs && valid_args {
            format!(
                "{}: max RHS for {} is {}\n{}: found RHS is {}\n ",
                "value overflow".bold(),
                self.name.to_uppercase().magenta(),
                RHS_MAXES[ins_class],
                "note".yellow(),
                rhs_val
            )
        } else {
            String::from("")
        };
        if !valid_lhs && !valid_rhs {
            Err((
                span,
                format!(
                    "{}: expected {} LHS, {} RHS\n{}: found {} LHS and {} RHS\n\n{ovfm}",
                    "invalid operands".bold(),
                    LHS_DETAIL[ins_class],
                    RHS_DETAIL[ins_class],
                    "note".yellow(),
                    lhs,
                    rhs,
                ),
            ))
        } else if !valid_rhs {
            Err((
                span,
                format!(
                    "{}: expected {} on RHS\n{}: found {} \n\n{ovfm}",
                    "invalid operands".bold(),
                    RHS_DETAIL[ins_class],
                    "note".yellow(),
                    rhs,
                ),
            ))
        } else if !valid_lhs {
            Err((
                span,
                format!(
                    "{}: expected {} on LHS\n{}: found {}\n\n{ovfm}",
                    "invalid operands".bold(),
                    LHS_DETAIL[ins_class],
                    "note".yellow(),
                    lhs,
                ),
            ))
        } else {
            Err((span, ovfm))
        }
    }
}

const LHS_DETAIL: [&str; 8] = [
    "reg",
    "mem addr or reg i.",
    "no",
    "reg",
    "mem addr or reg i.",
    "imm",
    "imm or reg",
    "mem addr or reg",
];
const RHS_DETAIL: [&str; 8] = [
    "reg, reg i., mem i., or imm",
    "no",
    "no",
    "mem addr or reg",
    "reg",
    "no",
    "no",
    "no",
];

const LHS_MAXES: [&str; 8] = [
    "reg < 8",                   // mov
    "mem < 1024 or reg i. < 10", // bcc
    "no",                        // ret/hlt
    "reg < 8",                   // ld
    "mem < 256 or reg i. < 10",  // st
    "imm < 128 and > -128",      // int
    "imm < 128 and > -128",      // push
    "mem < 2048 or reg < 10",    // pop
];

const RHS_MAXES: [&str; 8] = [
    "reg (i.) < 10 or imm > -128 and < 128 or mem i. < 128",
    "no",
    "no",
    "mem < 512",
    "reg < 8",
    "no",
    "no",
    "no",
];
