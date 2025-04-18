use crate::*;
use colored::*;
use std::collections::HashMap;
use std::ops::Range;
type ExpandResult = Result<Vec<(String, TokenKind, Range<usize>)>, Vec<MacroValidatorError>>;
impl MacroContent {
    pub fn expand(
        &self,
        default_span: &Range<usize>,
        err_file: &String,
        orig_data: &String,
        toks: &[(TokenKind, Range<usize>)], // incoming macro args
    ) -> ExpandResult {
        // okay... here, I need to check first if the token types of the input
        // match the tokens inside of the macro.
        // what I can do, is I can iterate through the input tokens, and iterate through the arguments

        let mut parsed_toks = Vec::new();
        let mut argument_indices = Vec::new();
        let mut errs = Vec::new();
        for (index, (token, span)) in toks.iter().enumerate() {
            if index == 0 {
                continue;
            }
            // this loop will clean up the toks and parse it into types
            let data = match token {
                token if token.is_reg() => Some(ArgumentType::Reg),
                token if token.is_ireg() => Some(ArgumentType::Ireg),
                token if token.is_mem() => Some(ArgumentType::Mem),
                token if token.is_imem() => Some(ArgumentType::Imem),
                token if token.is_imm() => Some(ArgumentType::Imm),
                token if token.is_ident() => Some(ArgumentType::Label),
                TokenKind::Comma => None,
                _ => {
                    errs.push(MacroValidatorError {
                        err_file: err_file.to_string(),
                        err_input: read_file(err_file),
                        err_message: format!("a {token} is not a valid macro argument"),
                        help: None,
                        orig_input: orig_data.to_string(),
                        orig_pos: span.clone(),
                        mac: self.clone(),
                    });
                    return Err(errs);
                }
            };
            if let Some(v) = data {
                parsed_toks.push((v, span));
                argument_indices.push(index);
            }
        }
        let mut current_args = Vec::new();
        for (_, arg, e) in &self.parameters {
            current_args.push((arg.arg_type.clone(), e));
        }
        let f = if let Some((_, s)) = toks.first() {
            s
        } else {
            default_span
        };
        if parsed_toks.len() != self.parameters.len() {
            let word = if self.parameters.len() == 1 {
                "argument"
            } else {
                "arguments"
            };
            errs.push(MacroValidatorError {
                err_file: err_file.to_string(),
                err_input: read_file(err_file),
                err_message: format!(
                    "expected {} {word}, found {}",
                    self.parameters.len(),
                    parsed_toks.len()
                ),
                help: None,
                orig_input: orig_data.to_string(),
                orig_pos: f.clone(),
                mac: self.clone(),
            });
        }
        for (index, (_, arg, _)) in self.parameters.iter().enumerate() {
            if let Some((d, _)) = parsed_toks.get(index) {
                if *d == arg.arg_type {
                    continue;
                } else {
                    errs.push(MacroValidatorError {
                        err_file: err_file.to_string(),
                        err_input: read_file(err_file),
                        err_message: format!("expected {}, found {d}", arg.arg_type),
                        help: None,
                        orig_input: orig_data.to_string(), // this shouldn't panic
                        orig_pos: parsed_toks.get(index).unwrap().1.clone(),
                        mac: self.clone(),
                    });
                    return Err(errs);
                }
            } else {
                errs.push(MacroValidatorError {
                    err_file: err_file.to_string(),
                    err_input: read_file(err_file),
                    err_message: String::from("an incorrect number of arguments were supplied"),
                    help: None, // borrow checker is yappin
                    orig_input: orig_data.to_string(),
                    orig_pos: f.clone(),
                    mac: self.clone(),
                });
            }
        } // we need a hashmap of type ident names, TokenKind to record arguments
        if !errs.is_empty() {
            return Err(errs);
        } // don't try to expand it if we have problems
          //
          //
          // macro expandation   name       value
        let mut arg_map: HashMap<String, crate::TokenKind> = HashMap::new();
        let mut count = 0;
        for element in argument_indices {
            // we no longer need to keep track of argument locations
            if let Some((v, _)) = toks.get(element) {
                if let Some((_, l, _)) = self.parameters.get(count) {
                    arg_map.insert(l.name.to_string(), v.clone());
                    count += 1;
                }
            }
        }

        // whenever the err input and orig input are the same, it is because the error cannot
        // occur across files
        let mut new_elems = Vec::new();
        for (element, span) in &self.body {
            if let TokenKind::MacroIdent(name) = element {
                if let Some(v) = arg_map.get(name) {
                    new_elems.push((self.file.to_string(), v.clone(), span.clone()));
                    continue;
                } else {
                    errs.push(MacroValidatorError {
                        err_file: self.file.to_string(),
                        err_input: read_file(&self.file.to_string()),
                        err_message: format!(
                            "{} was not an argument supplied in the macro parameters",
                            name.magenta()
                        ),
                        help: None, // borrow checker is yappin
                        orig_input: read_file(&self.file.to_string()),
                        orig_pos: span.clone(),
                        mac: self.clone(),
                    });
                }
            } else if let TokenKind::Instruction(contents) = element {
                let mut ins_args = Vec::new();
                for (thing, place) in &contents.operands {
                    if let InstructionArgument::MacroIdent(name) = thing {
                        if let Some(v) = arg_map.get(name) {
                            ins_args.push((v.to_tok_kind(), place.clone()));
                            continue;
                        } else {
                            // we could make this a variable earlier and just send it
                            errs.push(MacroValidatorError {
                                err_file: self.file.to_string(),
                                err_input: read_file(&self.file.to_string()),
                                err_message: format!(
                                    "{} was not an argument supplied in the macro parameters",
                                    name.magenta()
                                ),
                                help: None, // borrow checker is yappin
                                orig_input: read_file(&self.file.to_string()),
                                orig_pos: place.clone(),
                                mac: self.clone(),
                            });
                        }
                    }
                    ins_args.push((thing.clone(), place.clone()));
                }
                let reconstruct = InstructionData {
                    expanded: true,
                    name: contents.name.to_string(),
                    operands: ins_args,
                    location: span.clone(),
                };
                if let Err(e) = reconstruct.is_valid() {
                    errs.push(MacroValidatorError {
                        err_file: self.file.to_string(),
                        err_input: read_file(&self.file.to_string()), // these are dup'd as it is
                        err_message: e.1,
                        help: e.2,
                        orig_input: read_file(&self.file.to_string()), // these are dup'd as it is
                        // something in the macro
                        orig_pos: e.0.unwrap_or_else(|| span.clone()),
                        mac: self.clone(),
                    });
                }
                new_elems.push((
                    self.file.to_string(),
                    TokenKind::Instruction(reconstruct),
                    span.clone(),
                ));
                continue;
            } else if let TokenKind::MacroLabel(label_name) = element {
                if let Some(TokenKind::Ident(v)) = arg_map.get(label_name) {
                    new_elems.push((
                        self.file.to_string(),
                        TokenKind::Label(v.to_string()),
                        span.clone(),
                    ));
                    continue;
                } else {
                    errs.push(MacroValidatorError {
                        err_file: self.file.to_string(),
                        err_input: read_file(&self.file.to_string()),
                        err_message: format!(
                            "`{}` must be an identifier specified in macro arguments",
                            label_name.magenta()
                        ),
                        help: None, // borrow checker is yappin
                        orig_input: read_file(&self.file.to_string()),
                        orig_pos: span.clone(),
                        mac: self.clone(),
                    });
                }
            }
            new_elems.push((self.file.to_string(), element.clone(), span.clone()));
        }
        if !errs.is_empty() {
            return Err(errs);
        }
        Ok(new_elems)
    }
}
