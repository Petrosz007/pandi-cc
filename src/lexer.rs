use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // Can have value
    Identifier,
    Constant,
    // Structural element
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    Semicolon,
    // Keywords
    Int,
    Void,
    Return,
    // EOF
    Eof,
}

#[derive(Clone, Debug)]
pub struct TokenLocation {
    pub file_name: String, // ? Maybe box this or something, could have a large memory footprint
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub length: usize,
}

impl Display for TokenLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}:{}",
            self.file_name, self.line, self.column
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    pub location: TokenLocation,
}

pub struct Lexer {
    file_name: String,
    source: Vec<char>,
    pos: usize,
    current_line: usize,
    current_col: usize,
    current_token_start: usize,
    current_token_start_col: usize,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidNumber(TokenLocation),
    UnrecognisedCharacter(TokenLocation),
}

impl Lexer {
    pub fn new(file_name: impl Into<String>, source: impl Into<String>) -> Lexer {
        Lexer {
            file_name: file_name.into(),
            source: source.into().chars().collect(),
            pos: 0,
            current_line: 1,
            current_col: 1,
            current_token_start: 0,
            current_token_start_col: 1,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }

    fn next(&mut self) -> Option<char> {
        let chr = *self.source.get(self.pos)?;
        self.pos += 1;
        self.current_col += 1;
        // TODO: Move this to skip_whitespace
        if chr == '\n' {
            self.current_line += 1;
            self.current_col = 1;
            self.current_token_start_col = 1;
        }
        Some(chr)
    }

    fn check_keyword(&self, start: usize, rest: &'static str) -> bool {
        for (i, c) in rest.char_indices() {
            match self.source.get(start + i) {
                None => return false,
                Some(chr) if c != *chr => return false,
                _ => continue,
            }
        }

        true
    }

    fn current_location(&self) -> TokenLocation {
        TokenLocation {
            file_name: self.file_name.clone(),
            line: self.current_line,
            column: self.current_token_start_col,
            start: self.current_token_start,
            length: self.pos - self.current_token_start,
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let location = self.current_location();
        Token {
            token_type,
            text: self
                .source
                .get(location.start..location.start + location.length)
                .expect("a token to be in the source string, when we already parsed it")
                .iter()
                .collect(),
            location,
        }
    }

    fn identifier_type(&self) -> TokenType {
        match self
            .source
            .get(self.current_token_start)
            .expect("that a character we already checked to be in the string")
        {
            'i' if self.check_keyword(self.current_token_start + 1, "nt") => TokenType::Int,
            'v' if self.check_keyword(self.current_token_start + 1, "oid") => TokenType::Void,
            'r' if self.check_keyword(self.current_token_start + 1, "eturn") => TokenType::Return,
            _ => TokenType::Identifier,
        }
    }

    fn parse_identifier(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.next();
            } else {
                break;
            }
        }

        self.make_token(self.identifier_type())
    }

    fn parse_number(&mut self) -> Result<Token, LexerError> {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.next();
                continue;
            } else if c.is_alphabetic() {
                return Err(LexerError::InvalidNumber(self.current_location()));
            } else {
                break;
            }
        }

        Ok(self.make_token(TokenType::Constant))
    }

    fn parse_token(&mut self) -> Result<Token, LexerError> {
        // Skip whitespace
        while let Some(c) = self.peek() {
            // TODO: Skip comments? (technically the preprocessor does it for us)
            if c.is_ascii_whitespace() {
                // TODO: On Tabs I don't by how much should I increment the current_col
                self.next();
            } else {
                break;
            }
        }

        self.current_token_start = self.pos;
        self.current_token_start_col = self.current_col;

        match self.next() {
            None => Ok(self.make_token(TokenType::Eof)),
            Some(first_char) => match first_char {
                '(' => Ok(self.make_token(TokenType::ParenOpen)),
                ')' => Ok(self.make_token(TokenType::ParenClose)),
                '{' => Ok(self.make_token(TokenType::BraceOpen)),
                '}' => Ok(self.make_token(TokenType::BraceClose)),
                ';' => Ok(self.make_token(TokenType::Semicolon)),
                c if c.is_alphabetic() => Ok(self.parse_identifier()),
                c if c.is_ascii_digit() => self.parse_number(),
                _ => Err(LexerError::UnrecognisedCharacter(self.current_location())),
            },
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.parse_token()?;
            let is_eof = token.token_type == TokenType::Eof;
            tokens.push(token);
            if is_eof {
                return Ok(tokens);
            }
        }
    }
}
