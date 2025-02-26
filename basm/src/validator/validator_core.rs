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

        let (ins_class, ok_lhs, ok_rhs, valid_lhs, valid_rhs) = self.valid_args()?;
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
