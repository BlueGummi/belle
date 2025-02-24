use crate::*;
use colored::*;
impl InstructionData {
    pub fn is_valid(&self) -> Result<(), String> {
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

        let (ins_class, ok_val, valid_args) = match self.name.to_lowercase().as_str() {
            "add" | "mov" | "nand" | "div" | "cmp" => (
                0,
                self.operands
                    .first()
                    .is_some_and(|x| x.0.is_reg() && x.0.get_value() <= 7)
                    && self.operands.get(1).is_some_and(|expr| {
                        ((expr.0.is_reg() || expr.0.is_ireg()) && expr.0.get_value() <= 9)
                            || (expr.0.is_imm()
                                && expr.0.get_value() <= 255
                                && expr.0.get_value() >= -127)
                            || (expr.0.is_imem() && expr.0.get_value() <= 127)
                    }),
                self.operands.len() == 2
                    && self.operands.first().is_some_and(|x| x.0.is_reg())
                    && self.operands.get(1).is_some_and(|expr| {
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
                self.operands.len() == 1
                    && self
                        .operands
                        .first()
                        .is_some_and(|arg| arg.0.is_ident() || arg.0.is_mem() || arg.0.is_ireg()),
            ),
            "ret" | "hlt" => (2, self.operands.is_empty(), self.operands.is_empty()),
            "ld" | "lea" => (
                3,
                self.operands
                    .first()
                    .is_some_and(|x| x.0.is_reg() && x.0.get_value() <= 7)
                    && self.operands.get(1).is_some_and(|expr| {
                        (expr.0.is_mem() && expr.0.get_value() <= 511) || expr.0.is_ident()
                    }),
                self.operands.len() == 2
                    && self.operands.first().is_some_and(|x| x.0.is_reg())
                    && self
                        .operands
                        .get(1)
                        .is_some_and(|expr| expr.0.is_mem() || expr.0.is_ident()),
            ),
            "st" => (
                4,
                self.operands.first().is_some_and(|x| {
                    (x.0.is_mem() && x.0.get_value() <= 255)
                        || (x.0.is_ireg() && x.0.get_value() <= 9)
                        || x.0.is_ident()
                }) && self
                    .operands
                    .get(1)
                    .is_some_and(|x| x.0.is_reg() && x.0.get_value() <= 7),
                self.operands.len() == 2
                    && self
                        .operands
                        .first()
                        .is_some_and(|x| x.0.is_mem() || x.0.is_ireg() || x.0.is_ident())
                    && self.operands.get(1).is_some_and(|expr| expr.0.is_reg()),
            ),
            "int" => (
                5,
                self.operands.first().is_some_and(|x| {
                    x.0.is_imm() && x.0.get_value() <= 255 && x.0.get_value() >= -127
                }),
                self.operands.len() == 1 && self.operands.first().is_some_and(|x| x.0.is_imm()),
            ),
            "push" => (
                6,
                self.operands.first().is_some_and(|x| {
                    (x.0.is_imm() && x.0.get_value() <= 255 && x.0.get_value() >= -127)
                        || (x.0.is_reg() && x.0.get_value() <= 9)
                }),
                self.operands.len() == 1
                    && self
                        .operands
                        .first()
                        .is_some_and(|x| x.0.is_imm() || x.0.is_reg()),
            ),
            "pop" => (
                7,
                self.operands.first().is_some_and(|x| {
                    (x.0.is_mem() && x.0.get_value() <= 2047)
                        || (x.0.is_reg() && x.0.get_value() <= 9)
                }),
                self.operands.len() == 1
                    && self
                        .operands
                        .first()
                        .is_some_and(|x| x.0.is_mem() || x.0.is_reg()),
            ),
            _ => {
                return Err(format!(
                    "instruction {} does not exist",
                    self.name.to_uppercase()
                ))
            }
        };
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
        let ovfm = if !ok_val && valid_args {
            format!(
                "{}: max LHS for {} is {}, max RHS is {}\n{}: found LHS and RHS values are {} and {}\n ",
                "value too large".bold(),
                self.name.to_uppercase().magenta(),
                LHS_MAXES[ins_class].cyan(), RHS_MAXES[ins_class].cyan(), "note".yellow(), lhs_val.blue(), rhs_val.blue()
            )
        } else {
            String::from("")
        };
        if !valid_args {
            Err(format!(
                "{}: {} requires {} LHS and\n{} RHS\n{}: found {} LHS and {} RHS\n\n{ovfm}",
                "invalid instruction".bold(),
                self.name.to_uppercase().magenta(),
                LHS_DETAIL[ins_class].cyan(),
                RHS_DETAIL[ins_class].cyan(),
                "note".yellow(),
                lhs.blue(),
                rhs.blue(),
            ))
        } else {
            Err(ovfm)
        }
    }
}

const LHS_DETAIL: [&str; 8] = [
    "register",
    "memory address or register indirect",
    "no",
    "register",
    "memory address or register indirect",
    "immediate",
    "immediate or register",
    "memory address or register",
];
const RHS_DETAIL: [&str; 8] = [
    "register, register indirect, memory indirect, or immediate",
    "no",
    "no",
    "memory address or register",
    "register",
    "no",
    "no",
    "no",
];

const LHS_MAXES: [&str; 8] = [
    "reg. < 8",                      // mov
    "mem. < 1024 or reg. ind. < 10", // bcc
    "no",                            // ret/hlt
    "reg. < 8",                      // ld
    "mem. < 256 or reg. ind. < 10",  // st
    "imm < 128 and > -128",          // int
    "imm < 128 and > -128",          // push
    "mem. < 2048 or reg. < 10",      // pop
];

const RHS_MAXES: [&str; 8] = [
    "reg. (ind.) < 10 or imm > -128 and < 128 or mem. ind. < 128",
    "no",
    "no",
    "mem. < 512",
    "reg < 8",
    "no",
    "no",
    "no",
];
