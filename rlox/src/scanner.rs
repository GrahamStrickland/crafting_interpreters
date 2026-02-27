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
                _ => self.error(self.line, String::from("Unexpected character.")),
            }
        }
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

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let mut text = String::new();
        for i in self.start..self.current {
            match self.source.chars().nth(i) {
                Some(c) => text.push(c),
                None => continue,
            }
        }

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
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
