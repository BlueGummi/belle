use crate::Error::*;
use crate::*;

pub struct Lexer<'a> {
    pub location: u32,
    pub line_number: u32,
    pub tokens: Vec<Token>,
    pub chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub fn new(line: &'a str, line_number: u32) -> Self {
        Self {
            location: 1,
            line_number,
            tokens: Vec::new(),
            chars: line.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Result<&Vec<Token>, Error<'a>> {
        while let Some(c) = self.chars.next() {
            self.location += 1;
            match c {
                ' ' => continue,
                '\t' => {
                    self.location += 3;
                    continue;
                }
                '\n' => self.tokens.push(Token::NewLine),
                ',' => {
                    self.location += 1;
                    self.tokens.push(Token::Comma);
                }
                ';' => break,
                '&' => {
                    self.location += 1;
                    self.lex_pointer(c)?;
                }
                '@' => self.lex_subroutine_call(),
                'r' | 'R' => {
                    if let Some(next) = self.chars.peek() {
                        if next.is_ascii_digit() {
                            self.lex_register(c)?;
                        } else {
                            self.lex_identifier(c)?;
                        }
                    } else {
                        self.lex_identifier(c)?;
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    self.lex_identifier(c)?;
                }
                '#' => {
                    self.location += 1;
                    self.lex_literal(c)?;
                }
                '$' => {
                    self.location += 1;
                    self.lex_memory_address(c)?;
                }
                '.' => self.lex_label()?,
                '\'' => {
                    self.location += 1;
                    self.lex_ascii()?;
                }
                '-' => {
                    self.lex_literal(c)?;
                }
                '0'..='9' => {
                    self.lex_literal(c)?;
                }
                '[' => {
                    self.location += 1;
                    self.lex_memory_address(c)?;
                }
                '\"' => {
                    self.location += 1;
                    self.lex_asciiz(c)?;
                }
                '=' => {
                    self.tokens.push(Token::EqualSign);
                }
                _ => {
                    return Err(UnknownCharacter(
                        c.to_string(),
                        self.line_number,
                        Some(self.location),
                    ));
                }
            }
        }

        Ok(&self.tokens)
    }

    fn lex_ascii(&mut self) -> Result<(), Error<'a>> {
        let mut ascii_char = String::new();

        for next_char in self.chars.by_ref() {
            self.location += 1;

            match next_char {
                '\'' => {
                    if ascii_char.is_empty() {
                        return Err(Error::InvalidSyntax(
                            "ASCII  value is empty",
                            self.line_number,
                            Some(self.location),
                        ));
                    }

                    if ascii_char.len() > 1 {
                        return Err(Error::InvalidSyntax(
                            "ASCII value has more than one character",
                            self.line_number,
                            Some(self.location),
                        ));
                    }
                    let ascii_value = ascii_char.chars().next().unwrap() as i16;
                    self.tokens.push(Token::Literal(ascii_value));
                    return Ok(());
                }
                _ => {
                    ascii_char.push(next_char);
                }
            }
        }
        Err(Error::InvalidSyntax(
            "ASCII value is missing closing quote",
            self.line_number,
            Some(self.location),
        ))
    }

    fn lex_pointer(&mut self, c: char) -> Result<(), Error<'a>> {
        let mut pointer = String::new();
        pointer.push(c);

        let is_reg = match self.chars.peek() {
            Some(&'r' | &'R') => {
                self.location += 1;
                pointer.push(self.chars.next().unwrap());
                true
            }
            Some(&('0'..='9')) => {
                self.location += 1;
                pointer.push(self.chars.next().unwrap());
                false
            }
            Some(&'$') => {
                self.location += 1;
                pointer.push(self.chars.next().unwrap());
                false
            }
            _ => {
                self.location += 1;
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
        }

        if is_reg {
            self.handle_register(pointer)?;
        } else {
            self.handle_memory(pointer)?;
        }
        Ok(())
    }

    fn handle_register(&mut self, pointer: String) -> Result<(), Error<'a>> {
        if pointer.len() > 2 {
            self.location += 1;
            if let Ok(reg) = pointer.trim()[2..].parse::<i16>() {
                if !(0..=7).contains(&reg) {
                    return Err(Error::InvalidSyntax(
                        "invalid register pointer number",
                        self.line_number,
                        Some(self.location),
                    ));
                }
                self.tokens.push(Token::RegPointer(reg));
            } else {
                return Err(InvalidSyntax(
                    "invalid register number",
                    self.line_number,
                    Some(self.location),
                ));
            }
        } else {
            return Err(InvalidSyntax(
                "register must have a number",
                self.line_number,
                Some(self.location),
            ));
        }
        Ok(())
    }

    fn handle_memory(&mut self, pointer: String) -> Result<(), Error<'a>> {
        let pointer_trimmed = pointer.trim();
        if !pointer_trimmed.contains('$') {
            if let Ok(mem) = pointer_trimmed[1..].parse::<i16>() {
                self.tokens.push(Token::MemPointer(mem));
            } else {
                return self.handle_invalid_character(pointer_trimmed);
            }
            return Ok(());
        }
        if pointer_trimmed.len() > 2 {
            if let Ok(mem) = pointer_trimmed[2..].parse::<i16>() {
                self.tokens.push(Token::MemPointer(mem));
            } else {
                return self.handle_invalid_character(pointer_trimmed);
            }
        } else {
            return Err(Error::InvalidSyntax(
                "memory must have a number",
                self.line_number,
                Some(self.location),
            ));
        }
        Ok(())
    }

    fn lex_register(&mut self, c: char) -> Result<(), Error<'a>> {
        let mut reg = String::new();

        let remaining_chars: String = self.chars.clone().collect();
        if remaining_chars.eq_ignore_ascii_case("ret") {
            return Ok(());
        }
        reg.push(c);
        if let Some(&next) = self.chars.peek() {
            if next == 'r' || next == 'R' {
                reg.push(self.chars.next().unwrap());
            } else if !c.is_alphanumeric() {
                return Err(Error::ExpectedArgument(
                    "expected alphanumeric argument after 'r'",
                    self.line_number,
                    Some(self.location),
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
                if !(0..=7).contains(&reg_num) {
                    return Err(Error::InvalidSyntax(
                        "invalid register number",
                        self.line_number,
                        Some(self.location),
                    ));
                }
                self.tokens.push(Token::Register(reg_num));
            } else {
                return Err(Error::InvalidSyntax(
                    "invalid register number",
                    self.line_number,
                    Some(self.location),
                ));
            }
        }
        Ok(())
    }

    fn lex_subroutine_call(&mut self) {
        let mut subroutine_call = String::new();
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                subroutine_call.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        self.tokens.push(Token::SRCall(subroutine_call));
    }

    fn lex_identifier(&mut self, c: char) -> Result<(), Error<'a>> {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                ident.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        if let Some(&next) = self.chars.peek() {
            if next == ':' {
                self.chars.next();
                return Ok(());
            }
        }
        self.tokens.push(Token::Ident(ident));
        Ok(())
    }

    fn lex_literal(&mut self, c: char) -> Result<(), Error<'a>> {
        let mut number = c.to_string();
        if let Some(&next) = self.chars.peek() {
            if next == '-' {
                number.push(self.chars.next().unwrap());
            }
        }

        while let Some(&next) = self.chars.peek() {
            if next.is_ascii_digit() || next.is_alphanumeric() {
                number.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        let num_value = if !number.contains('#') {
            if let Ok(value) = number.parse::<i16>() {
                value
            } else {
                return self.handle_invalid_character(&number);
            }
        } else if let Ok(value) = number[1..].parse::<i16>() {
            value
        } else {
            return self.handle_invalid_character(&number[1..]);
        };

        let stored_value = if num_value < 0 {
            let positive_value = num_value.unsigned_abs() as u8;
            (positive_value & 0x7F) | 0x80
        } else {
            num_value as u8
        };
        self.tokens.push(Token::Literal(i16::from(stored_value)));
        Ok(())
    }
    fn handle_invalid_character(&mut self, input: &str) -> Result<(), Error<'a>> {
        let variable = if input.starts_with('$') || input.starts_with('#') || input.starts_with('&')
        {
            &input[1..]
        } else if input.starts_with('[') {
            &input[1..input.len() - 1]
        } else {
            input
        };
        let map = VARIABLE_MAP.lock().unwrap();
        if let Some(&replacement) = map.get(variable.trim()) {
            if input.starts_with('$') || input.starts_with('[') {
                self.tokens.push(Token::MemAddr(replacement as i16));
            } else if input.starts_with('#') {
                self.tokens.push(Token::Literal(replacement as i16));
            } else if input.starts_with('&') {
                self.tokens.push(Token::MemPointer(replacement as i16));
            }
            Ok(())
        } else {
            Err(Error::InvalidSyntax(
                "Invalid character or value",
                self.line_number,
                Some(self.location),
            ))
        }
    }

    fn lex_asciiz(&mut self, c: char) -> Result<(), Error<'a>> {
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
                    return Err(InvalidSyntax(
                        "expected a closing \"",
                        self.line_number,
                        Some(self.location),
                    ));
                }
            }
        }
        Ok(())
    }
    fn lex_memory_address(&mut self, c: char) -> Result<(), Error<'a>> {
        let mut addr = c.to_string();

        if addr == "[" {
            while let Some(&next) = self.chars.peek() {
                if next.is_alphanumeric() {
                    addr.push(self.chars.next().unwrap());
                } else if next == ']' {
                    addr.push(self.chars.next().unwrap());
                    break;
                } else {
                    return Err(InvalidSyntax(
                        "expected closing bracket or digit",
                        self.line_number,
                        Some(self.location),
                    ));
                }
            }

            if addr.len() < 3 || addr[1..addr.len() - 1].parse::<i16>().is_err() {
                return self.handle_invalid_character(&addr);
            }

            let addr_val = addr[1..addr.len() - 1].parse::<i16>().unwrap();
            self.tokens.push(Token::MemAddr(addr_val));
        } else {
            while let Some(&next) = self.chars.peek() {
                if next.is_alphanumeric() {
                    addr.push(self.chars.next().unwrap());
                } else {
                    break;
                }
            }

            if addr[1..].parse::<i16>().is_err() {
                return self.handle_invalid_character(&addr);
            }

            let addr_val = addr[1..].parse::<i16>().unwrap();
            self.tokens.push(Token::MemAddr(addr_val));
        }

        Ok(())
    }

    fn lex_label(&mut self) -> Result<(), Error<'a>> {
        let mut label = String::new();
        while let Some(&next) = self.chars.peek() {
            if next.is_alphanumeric() || next == '_' {
                label.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        self.tokens.push(Token::Label(label));
        Ok(())
    }
}

pub fn print_subroutine_map() {
    let map = SUBROUTINE_MAP.lock().unwrap();
    for (name, counter) in map.iter() {
        if CONFIG.verbose || CONFIG.debug {
            println!("Subroutine: {name}, Counter: {counter}");
        }
    }
    let vmap = VARIABLE_MAP.lock().unwrap();
    for (name, counter) in vmap.iter() {
        if CONFIG.verbose || CONFIG.debug {
            println!("Variable: {name}, Thing: {counter}");
        }
    }
}
