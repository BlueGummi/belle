use crate::Error::*;
use crate::*;
use colored::*;

pub struct Lexer<'a> {
    pub position: usize,
    pub line_number: usize,
    pub tokens: Vec<Token>,
    pub chars: std::iter::Peekable<std::str::Chars<'a>>,
    pub errors: Vec<Error<'a>>,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn new(line: &'a str, line_number: usize) -> Self {
        Self {
            position: 1,
            line_number,
            tokens: Vec::new(),
            chars: line.chars().peekable(),
            errors: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<&Vec<Token>, &Vec<Error<'a>>> {
        while let Some(c) = self.chars.next() {
            match c {
                ' ' | ']' => {
                    self.position += 1;
                }
                '\t' => {
                    self.position += 3;
                }
                '\n' => self.tokens.push(Token::NewLine),
                ',' => {
                    self.position += 1;
                    self.tokens.push(Token::Comma);
                }
                ';' => break,
                '&' => {
                    self.position += 1;
                    self.lex_pointer(c);
                }
                'r' | 'R' => {
                    if let Some(next) = self.chars.peek() {
                        if next.is_ascii_digit() {
                            self.lex_register(c);
                        } else {
                            self.position += 1;
                            self.lex_identifier(c);
                        }
                    } else {
                        self.position += 1;
                        self.lex_identifier(c);
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    self.position += 1;
                    self.lex_identifier(c);
                }
                '.' => {
                    self.position += 1;
                    self.lex_directive();
                }
                '\'' => {
                    self.position += 1;
                    self.lex_ascii();
                }
                '-' => {
                    self.position += 1;
                    self.lex_literal(c);
                }
                '0'..='9' => {
                    self.position += 1;
                    self.lex_literal(c);
                }
                '[' => {
                    self.position += 1;
                    self.lex_memory_address(c);
                }
                '\"' => {
                    self.position += 1;
                    self.lex_asciiz(c);
                }
                '=' => {
                    self.position += 1;
                    self.tokens.push(Token::EqualSign);
                }
                _ => {
                    self.position += 1;
                    self.errors.push(UnknownCharacter(
                        format!("character '{}' not recognized", c,),
                        self.line_number,
                        Some(self.position),
                        None,
                    ));
                }
            }
        }
        if !self.errors.is_empty() {
            return Err(&self.errors);
        }
        Ok(&self.tokens)
    }

    fn lex_ascii(&mut self) {
        let mut ascii_char = String::new();

        for next_char in self.chars.by_ref() {
            self.position += 1;

            match next_char {
                '\'' => {
                    if ascii_char.is_empty() {
                        self.errors.push(Error::InvalidSyntax(
                            String::from("ASCII character is empty"),
                            self.line_number,
                            Some(self.position),
                            Some(String::from("try a correct ASCII character, like 'a'")),
                        ));
                    }

                    if ascii_char.len() > 1 {
                        self.errors.push(Error::InvalidSyntax(
                            format!(
                                "ASCII character '{}' has more than one character",
                                ascii_char.magenta()
                            ),
                            self.line_number,
                            Some(self.position),
                            Some(String::from("try a correct ASCII character, like 'a'")),
                        ));
                    }
                    let ascii_value = match ascii_char.chars().next() {
                        Some(v) => v as i16,
                        None => {
                            self.errors.push(Error::InvalidSyntax(
                                format!("ASCII character '{}' invalid", ascii_char.magenta()),
                                self.line_number,
                                Some(self.position),
                                Some(String::from("try a correct ASCII character, like 'a'")),
                            ));
                            return;
                        }
                    };
                    self.tokens.push(Token::Literal(ascii_value));
                }
                _ => {
                    ascii_char.push(next_char);
                }
            }
        }
        self.errors.push(Error::InvalidSyntax(
            format!(
                "ASCII character '{}' is missing closing quote",
                ascii_char.magenta()
            ),
            self.line_number,
            Some(self.position),
            None,
        ));
    }
    fn lex_number(&self, complete_number: &str) -> Result<i32, String> {
        let complete_number = complete_number.trim();
        if let Some(stripped) = complete_number.strip_prefix("0x") {
            i32::from_str_radix(stripped, 16).map_err(|e| e.to_string())
        } else if let Some(stripped) = complete_number.strip_prefix("0b") {
            i32::from_str_radix(stripped, 2).map_err(|e| e.to_string())
        } else {
            complete_number.parse::<i32>().map_err(|e| e.to_string())
        }
    }
    fn lex_pointer(&mut self, c: char) {
        let mut pointer = String::new();
        pointer.push(c);

        let is_reg = match self.chars.peek() {
            Some(&'r' | &'R') => {
                self.position += 1;
                pointer.push(self.chars.next().unwrap());
                true
            }
            Some(&('0'..='9')) => {
                self.position += 1;
                pointer.push(self.chars.next().unwrap());
                false
            }
            Some(&'[') => {
                self.position += 1;
                pointer.push(self.chars.next().unwrap());
                false
            }
            _ => {
                self.position += 1;
                pointer.push(self.chars.next().unwrap());
                false
            }
        };

        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() {
                pointer.push(self.chars.next().unwrap());
            } else {
                break;
            }
            self.position += 1;
        }

        if is_reg {
            self.handle_register(pointer);
        } else {
            self.handle_memory(pointer[1..].to_string());
        }
    }

    fn handle_register(&mut self, pointer: String) {
        if pointer.len() > 2 {
            self.position += 1;
            if let Ok(reg) = pointer.trim()[2..].parse::<i16>() {
                if !(0..=9).contains(&reg) {
                    self.errors.push(Error::InvalidSyntax(
                        format!("register indirect {} is invalid", reg.to_string().magenta()),
                        self.line_number,
                        Some(self.position),
                        Some(format!("valid registers are {}", "r0-r9".magenta())),
                    ));
                }
                self.tokens.push(Token::RegPointer(reg));
            } else {
                self.errors.push(InvalidSyntax(
                    format!(
                        "register number {} is invalid",
                        pointer.trim()[2..].magenta()
                    ),
                    self.line_number,
                    Some(self.position),
                    Some(format!("valid registers are {}", "r0-r9".magenta())),
                ));
            }
        } else {
            self.errors.push(InvalidSyntax(
                format!("register indirect {} must have a number", pointer.magenta()),
                self.line_number,
                Some(self.position),
                Some(format!("valid registers are {}", "r0-r9".magenta())),
            ));
        }
    }

    fn handle_memory(&mut self, pointer: String) {
        let pointer = pointer.trim();

        let pointer_trimmed = if let Some(v) = pointer.strip_prefix('[') {
            v
        } else {
            pointer
        };
        let pointer_trimmed = if pointer_trimmed.ends_with(']') {
            &pointer_trimmed[0..pointer_trimmed.len() - 1]
        } else {
            pointer_trimmed
        };
        if pointer_trimmed.len() > 1 {
            if let Ok(mem) = self.lex_number(pointer_trimmed) {
                self.tokens.push(Token::MemPointer(mem as i16));
            } else {
                self.handle_invalid_character(pointer_trimmed)
            }
        } else {
            self.errors.push(Error::InvalidSyntax(
                format!(
                    "memory indirect {} must have a number",
                    pointer.to_string().magenta()
                ),
                self.line_number,
                Some(self.position),
                Some(format!(
                    "valid addresses range from &{} to &{}",
                    "[0x0]".magenta(),
                    "[0xFFFF]".magenta()
                )),
            ));
        }
    }

    fn lex_register(&mut self, c: char) {
        let mut reg = String::new();

        let remaining_chars: String = self.chars.clone().collect();
        if remaining_chars.eq_ignore_ascii_case("ret") {
            return;
        }
        reg.push(c);
        if let Some(&next) = self.chars.peek() {
            if next == 'r' || next == 'R' {
                reg.push(self.chars.next().unwrap());
            } else if !c.is_alphanumeric() {
                self.errors.push(Error::ExpectedArgument(
                    "register must have a value after 'r'",
                    self.line_number,
                    Some(self.position),
                    Some(format!("valid registers are {}", "r0-r9".magenta())),
                ));
            }
        }

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() {
                reg.push(self.chars.next().unwrap());
            } else {
                break;
            }
            self.position += 1;
        }

        if reg.len() > 1 {
            if let Ok(reg_num) = reg[1..].parse::<i16>() {
                if !(0..=9).contains(&reg_num) {
                    self.errors.push(Error::InvalidSyntax(
                        format!("register number {} invalid", reg_num),
                        self.line_number,
                        Some(self.position),
                        Some(format!("valid registers are {}", "r0-r9".magenta())),
                    ));
                }
                self.tokens.push(Token::Register(reg_num));
            } else {
                self.errors.push(Error::InvalidSyntax(
                    format!("register {} invalid", reg),
                    self.line_number,
                    Some(self.position),
                    Some(format!("valid registers are {}", "r0-r9".magenta())),
                ));
            }
        }
    }

    fn lex_identifier(&mut self, c: char) {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                self.position += 1;
                ident.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        if let Some(&next) = self.chars.peek() {
            if next == ':' {
                self.position += 1;
                self.chars.next();
                return;
            }
        }
        self.tokens.push(Token::Ident(ident));
    }

    fn lex_literal(&mut self, c: char) {
        let mut number = c.to_string();
        if let Some(&next) = self.chars.peek() {
            if next == '-' {
                self.position += 1;
                number.push(self.chars.next().unwrap());
            }
        }

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() || next.is_alphanumeric() || next == '_' {
                self.position += 1;
                number.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        let num_value = if !number.contains('#') {
            if let Ok(value) = self.lex_number(&number) {
                value
            } else {
                return self.handle_invalid_character(&number);
            }
        } else if let Ok(value) = self.lex_number(&number) {
            value
        } else {
            return self.handle_invalid_character(&number);
        };

        let stored_value = if num_value < 0 {
            let positive_value = num_value.unsigned_abs() as u16;
            ((positive_value & 0x7F) | 0x80) as i16
        } else {
            num_value as i16
        };
        self.tokens.push(Token::Literal(stored_value));
    }
    fn handle_invalid_character(&mut self, input: &str) {
        let variable = if input.starts_with('#') || input.starts_with('&') {
            &input[1..]
        } else if input.starts_with('[') {
            if input.len() - 1 > 1 {
                &input[1..input.len() - 1]
            } else {
                self.errors.push(Error::InvalidSyntax(
                    format!("variable couldn't be parsed \"{}\"", input.trim().magenta()),
                    self.line_number,
                    Some(self.position),
                    None,
                ));

                return;
            }
        } else {
            input
        };
        let map = VARIABLE_MAP.lock().unwrap();
        if let Some(&replacement) = map.get(variable.trim()) {
            if input.starts_with('[') {
                self.tokens.push(Token::MemAddr(replacement as i16));
            } else if input.starts_with('#') {
                self.tokens.push(Token::Literal(replacement as i16));
            } else if input.starts_with('&') {
                self.tokens.push(Token::MemPointer(replacement as i16));
            }
        } else {
            self.errors.push(Error::InvalidSyntax(
                format!("could not find variable \"{}\"", variable.trim().magenta()),
                self.line_number,
                Some(self.position),
                None,
            ));
        }
    }

    fn lex_asciiz(&mut self, c: char) {
        let mut ascii_string = c.to_string();
        if ascii_string == "\"" {
            ascii_string = String::new();
            while let Some(&next) = self.chars.peek() {
                if next.is_ascii() && next != '\"' {
                    ascii_string.push(self.chars.next().unwrap());
                } else if next == '\"' {
                    self.tokens.push(Token::Asciiz(ascii_string));
                    break;
                } else {
                    self.errors.push(InvalidSyntax(
                        String::from("expected a closing \" in ASCII string"),
                        self.line_number,
                        Some(self.position),
                        Some(format!(
                            "add a closing quote in {}, e.g. \"ascii\"",
                            ascii_string
                        )),
                    ));
                }
                self.position += 1;
            }
        }
    }
    fn lex_memory_address(&mut self, c: char) {
        let mut addr = c.to_string();

        if addr == "[" {
            while let Some(&next) = self.chars.peek() {
                self.position += 1;
                if next.is_alphanumeric() {
                    addr.push(self.chars.next().unwrap());
                } else if next == ']' {
                    addr.push(self.chars.next().unwrap());
                    break;
                } else {
                    self.position -= 1;
                    self.errors.push(InvalidSyntax(
                        String::from("did not find closing bracket"),
                        self.line_number,
                        Some(self.position),
                        Some(format!("close the bracket of {}", addr.magenta())),
                    ));
                    return;
                }
            }

            if addr.len() < 3 || self.lex_number(&addr[1..addr.len() - 1]).is_err() {
                return self.handle_invalid_character(&addr);
            }
            let addr_val = self.lex_number(&addr[1..addr.len() - 1]);
            if let Ok(address) = addr_val {
                self.tokens.push(Token::MemAddr(address as i16));
            } else if addr_val.is_err() {
                self.errors.push(InvalidSyntax(
                    format!("could not parse {}", addr.red()),
                    self.line_number,
                    Some(self.position),
                    Some(format!(
                        "try a valid address, {} - {}",
                        "[0x0]".magenta(),
                        "[0xFFFF]".magenta()
                    )),
                ));
            }
        } else {
            while let Some(&next) = self.chars.peek() {
                if next.is_alphanumeric() || next == '_' {
                    self.position += 1;
                    addr.push(self.chars.next().unwrap());
                } else {
                    break;
                }
            }

            if self.lex_number(&addr[1..]).is_err() {
                return self.handle_invalid_character(&addr);
            }

            let addr_val = self.lex_number(&addr[1..]);
            if let Ok(address) = addr_val {
                self.tokens.push(Token::MemAddr(address as i16));
            } else if addr_val.is_err() {
                self.errors.push(InvalidSyntax(
                    format!("could not parse {}", addr.red()),
                    self.line_number,
                    Some(self.position),
                    Some(format!(
                        "try a valid address, {} - {}",
                        "[0x0]".magenta(),
                        "[0xFFFF]".magenta()
                    )),
                ));
            }
        }
    }

    fn lex_directive(&mut self) {
        let mut directive = String::new();
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                directive.push(self.chars.next().unwrap());
            } else {
                break;
            }
            self.position += 1;
        }
        self.tokens.push(Token::Directive(directive));
    }
}

pub fn print_label_map() {
    let map = LABEL_MAP.lock().unwrap();
    for (name, counter) in map.iter() {
        if CONFIG.verbose {
            println!("Label: {name}, Address: {counter}");
        }
    }
    let vmap = VARIABLE_MAP.lock().unwrap();
    for (name, counter) in vmap.iter() {
        if CONFIG.verbose {
            println!("Variable: {name}, Value: {counter}");
        }
    }
}
