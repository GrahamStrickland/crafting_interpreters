use std::collections::HashMap;

use crate::literal::Literal;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    pub had_error: bool,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            had_error: false,
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::Null,
            self.line,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let current_char = self.advance();

        if let Some(c) = current_char {
            match c {
                '(' => self.add_token(TokenType::LeftParen, Literal::Null),
                ')' => self.add_token(TokenType::RightParen, Literal::Null),
                '{' => self.add_token(TokenType::LeftBrace, Literal::Null),
                '}' => self.add_token(TokenType::RightBrace, Literal::Null),
                ',' => self.add_token(TokenType::Comma, Literal::Null),
                '.' => self.add_token(TokenType::Dot, Literal::Null),
                '-' => self.add_token(TokenType::Minus, Literal::Null),
                '+' => self.add_token(TokenType::Plus, Literal::Null),
                ';' => self.add_token(TokenType::Semicolon, Literal::Null),
                '*' => self.add_token(TokenType::Star, Literal::Null),
                '!' => {
                    let token = if self.match_token('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(token, Literal::Null);
                }
                '=' => {
                    let token = if self.match_token('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token, Literal::Null);
                }
                '<' => {
                    let token = if self.match_token('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(token, Literal::Null);
                }
                '>' => {
                    let token = if self.match_token('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(token, Literal::Null);
                }
                '/' => {
                    if self.match_token('/') {
                        // A comment goes until the end of the line.
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, Literal::Null);
                    }
                }
                ' ' | '\r' | '\t' => {
                    // Ignore whitespace.
                }
                '\n' => {
                    self.line += 1;
                }
                '"' => self.string(),
                _ => {
                    if self.is_digit(&c) {
                        self.number();
                    } else if c.is_alphabetic() {
                        self.identifier();
                    } else {
                        self.error(self.line, String::from("Unexpected character."));
                    }
                }
            }
        }
    }

    fn identifier(&mut self) {
        let mut c = self.peek();
        while self.is_alphanumeric(&c.unwrap()) {
            self.advance();
            c = self.peek();
        }

        let text = self.substring(self.start, self.current);
        let token_type = self.keywords.get(text.as_str());

        self.add_token(
            match token_type {
                Some(t) => *t,
                None => TokenType::Identifier,
            },
            Literal::Null,
        );
    }

    fn number(&mut self) {
        let mut c = self.peek();
        while self.is_digit(&c.unwrap()) {
            self.advance();
            c = self.peek();
        }

        // Look for a fractional part.
        if self.peek() == Some('.') {
            let next = self.peek_next();
            if self.is_digit(&next.unwrap()) {
                // Consume the "."
                self.advance();

                c = self.peek();
                while self.is_digit(&c.unwrap()) {
                    self.advance();
                    c = self.peek();
                }
            }
        }

        let value = self
            .substring(self.start, self.current)
            .parse::<f64>()
            .unwrap();
        self.add_token(TokenType::Number, Literal::Double(value));
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, String::from("Unterminated string."));
            return;
        }

        self.advance(); // The closing ".

        // Trim the surrounding quotes.
        let value = self.substring(self.start + 1, self.current - 1);
        self.add_token(TokenType::String, Literal::String(value));
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }
        self.source.chars().nth(self.current)
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            Some('\0')
        } else {
            self.source.chars().nth(self.current + 1)
        }
    }

    fn is_alpha(&mut self, c: &char) -> bool {
        (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z') || *c == '_'
    }

    fn is_alphanumeric(&mut self, c: &char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&mut self, c: &char) -> bool {
        *c >= '0' && *c <= '9'
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.substring(self.start, self.current);

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn substring(&mut self, start: usize, end: usize) -> String {
        let mut text = String::new();

        for i in start..end {
            match self.source.chars().nth(i) {
                Some(c) => text.push(c),
                None => continue,
            }
        }

        text
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn report(&mut self, line: usize, location: String, message: String) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}
