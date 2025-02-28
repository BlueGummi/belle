use crate::*;
use colored::*;
use std::ops::Range;

impl InstructionData {
    // location         // msg
    pub fn is_valid(&self) -> Result<(), (Option<Range<usize>>, String)> {
        // Ident is for matching labels - they will be memory addresses
        // registers are blue, addresses magenta, 'indirect' is underlined
        // 'imm' is green
        let reg = "reg".blue().to_string();
        let mem = "mem".magenta().to_string();
        let ind = "i".underline().to_string();
        let imm = "imm".green().to_string();
        let no = String::from("no");
        let lhs_detail: [&String; 8] = [
            &reg,
            &format!("{reg} or {mem} {ind}"),
            &no,
            &reg,
            &format!("{reg} {ind} or {mem}"),
            &imm,
            &format!("{imm} or {reg}"),
            &format!("{mem} or {reg}"),
        ];
        let rhs_detail: [&String; 8] = [
            &format!("{reg}, {reg} {ind}, {mem} {ind}, or {imm}"),
            &no,
            &no,
            &format!("{mem} or {reg}"),
            &reg,
            &no,
            &no,
            &no,
        ];

        let lhs_maxes: [&str; 8] = [
            "3 bit reg",                // mov
            "10 bit addr or 4 bit reg", // bcc
            "no",                       // ret/hlt
            "3 bit reg",                // ld
            "7 bit addr or 4 bit reg",  // st
            "8 bit imm",                // int
            "8 bit imm",                // push
            "11 bit addr or 4 bit reg", // pop
        ];

        let rhs_maxes: [&str; 8] = [
            "4 bit reg 8 bit imm or 7 bit mem",
            "no",
            "no",
            "9 bit mem",
            "3 bit reg",
            "no",
            "no",
            "no",
        ];

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
                lhs_maxes[ins_class], rhs_maxes[ins_class], "note".yellow(), lhs_val, rhs_val
            )
        } else if !ok_lhs && valid_args {
            format!(
                "{}: max LHS for {} is {}\n{}: found LHS is {}\n ",
                "value overflow".bold(),
                self.name.to_uppercase().magenta(),
                lhs_maxes[ins_class],
                "note".yellow(),
                lhs_val
            )
        } else if !ok_rhs && valid_args {
            format!(
                "{}: max RHS for {} is {}\n{}: found RHS is {}\n ",
                "value overflow".bold(),
                self.name.to_uppercase().magenta(),
                rhs_maxes[ins_class],
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
                    lhs_detail[ins_class],
                    rhs_detail[ins_class],
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
                    rhs_detail[ins_class],
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
                    lhs_detail[ins_class],
                    "note".yellow(),
                    lhs,
                ),
            ))
        } else {
            Err((span, ovfm))
        }
    }
}
