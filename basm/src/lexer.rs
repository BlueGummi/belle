use crate::Error::*;
use crate::*;

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
                    continue;
                }
                '\t' => {
                    self.position += 3;
                    continue;
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
                            self.lex_identifier(c);
                        }
                    } else {
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
                    self.lex_literal(c);
                }
                '0'..='9' => {
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
                    self.tokens.push(Token::EqualSign);
                }
                _ => {
                    self.position += 1;
                    self.errors.push(UnknownCharacter(
                        c.to_string(),
                        self.line_number,
                        Some(self.position),
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
                            "ASCII value is empty",
                            self.line_number,
                            Some(self.position),
                        ));
                    }

                    if ascii_char.len() > 1 {
                        self.errors.push(Error::InvalidSyntax(
                            "ASCII value has more than one character",
                            self.line_number,
                            Some(self.position),
                        ));
                    }
                    let ascii_value = ascii_char.chars().next().unwrap() as i16;
                    self.tokens.push(Token::Literal(ascii_value));
                }
                _ => {
                    ascii_char.push(next_char);
                }
            }
        }
        self.errors.push(Error::InvalidSyntax(
            "ASCII value is missing closing quote",
            self.line_number,
            Some(self.position),
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
                self.position += 1;
                pointer.push(self.chars.next().unwrap());
            } else {
                break;
            }
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
                        "invalid register pointer number",
                        self.line_number,
                        Some(self.position),
                    ));
                }
                self.tokens.push(Token::RegPointer(reg));
            } else {
                self.errors.push(InvalidSyntax(
                    "invalid register number",
                    self.line_number,
                    Some(self.position),
                ));
            }
        } else {
            self.errors.push(InvalidSyntax(
                "register must have a number",
                self.line_number,
                Some(self.position),
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
                "memory must have a number",
                self.line_number,
                Some(self.position),
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
                    "expected alphanumeric argument after 'r'",
                    self.line_number,
                    Some(self.position),
                ));
            }
        }

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() {
                reg.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        if reg.len() > 1 {
            if let Ok(reg_num) = reg[1..].parse::<i16>() {
                if !(0..=9).contains(&reg_num) {
                    self.errors.push(Error::InvalidSyntax(
                        "invalid register number",
                        self.line_number,
                        Some(self.position),
                    ));
                }
                self.tokens.push(Token::Register(reg_num));
            } else {
                self.errors.push(Error::InvalidSyntax(
                    "invalid register number",
                    self.line_number,
                    Some(self.position),
                ));
            }
        }
    }

    fn lex_identifier(&mut self, c: char) {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                ident.push(self.chars.next().unwrap());
            } else {
                break;
            }
            self.position += 1;
        }
        if let Some(&next) = self.chars.peek() {
            if next == ':' {
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
                number.push(self.chars.next().unwrap());
            }
        }

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() || next.is_alphanumeric() || next == '_' {
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
            &input[1..input.len() - 1]
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
                "Invalid character or value",
                self.line_number,
                Some(self.position),
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
                        "expected a closing \"",
                        self.line_number,
                        Some(self.position),
                    ));
                }
            }
        }
    }
    fn lex_memory_address(&mut self, c: char) {
        let mut addr = c.to_string();

        if addr == "[" {
            while let Some(&next) = self.chars.peek() {
                if next.is_alphanumeric() {
                    addr.push(self.chars.next().unwrap());
                } else if next == ']' {
                    addr.push(self.chars.next().unwrap());
                    break;
                } else {
                    self.errors.push(InvalidSyntax(
                        "expected closing bracket or digit",
                        self.line_number,
                        Some(self.position),
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
                    "Error parsing integer: {}",
                    self.line_number,
                    Some(self.position),
                ));
            }
        } else {
            while let Some(&next) = self.chars.peek() {
                if next.is_alphanumeric() || next == '_' {
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
                    "Error parsing integer",
                    self.line_number,
                    Some(self.position),
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
