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
                        while !self.is_at_end() && self.peek() != Some('\n') {
                            self.advance();
                        }
                    } else if self.match_token('*') {
                        match self.block_comment() {
                            Ok(_) => (),
                            Err(e) => self.error(self.line, String::from(e)),
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

    fn block_comment(&mut self) -> Result<(), &'static str> {
        while !self.is_at_end() {
            if self.peek() == Some('*') {
                self.match_token('*');

                if self.peek() == Some('/') {
                    self.match_token('/');
                    return Ok(());
                }

                return Err("Unterminated block comment, expected '/'.");
            } else {
                self.advance();
            }
        }

        Err("Unterminated block comment, expected '*' but reached end of input.")
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

#[cfg(test)]
mod tests {
    use super::Scanner;
    use crate::literal::Literal;
    use crate::token_type::TokenType;

    #[test]
    fn test_scan_tokens() {
        let mut scanner = Scanner::new(String::from(
            "1 + 2 // This is a comment\n/* This is also a comment */",
        ));
        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('1'));

        scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            "Token { token_type: Number, lexeme: \"1\", literal: Double(1.0), line: 1 }"
        );
        assert_eq!(
            format!("{:?}", scanner.tokens[1]),
            "Token { token_type: Plus, lexeme: \"+\", literal: Null, line: 1 }"
        );
        assert_eq!(
            format!("{:?}", scanner.tokens[2]),
            "Token { token_type: Number, lexeme: \"2\", literal: Double(2.0), line: 1 }"
        );
        assert_eq!(
            format!("{:?}", scanner.tokens[3]),
            "Token { token_type: Eof, lexeme: \"\", literal: Null, line: 2 }"
        );
    }

    #[test]
    fn test_scan_token() {
        let mut scanner = Scanner::new(String::from("1 + 2"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('1'));

        scanner.scan_token();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            "Token { token_type: Number, lexeme: \"1\", literal: Double(1.0), line: 1 }"
        );

        scanner.scan_token();
        scanner.start = scanner.current;
        scanner.scan_token();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(
            format!("{:?}", scanner.tokens[1]),
            "Token { token_type: Plus, lexeme: \"+\", literal: Null, line: 1 }"
        );

        scanner.scan_token();
        scanner.start = scanner.current;
        scanner.scan_token();

        assert_eq!(scanner.tokens.len(), 3);
        assert_eq!(
            format!("{:?}", scanner.tokens[2]),
            "Token { token_type: Number, lexeme: \"2\", literal: Double(2.0), line: 1 }"
        );

        scanner = Scanner::new(String::from("1 + $"));

        scanner.scan_token();
        scanner.start = scanner.current;
        scanner.scan_token();
        scanner.start = scanner.current;
        scanner.scan_token();
        scanner.start = scanner.current;
        scanner.scan_token();
        scanner.start = scanner.current;
        scanner.scan_token();
        scanner.start = scanner.current;
        assert!(scanner.had_error);
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new(String::from("class"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('c'));

        scanner.advance();
        scanner.identifier();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            String::from("Token { token_type: Class, lexeme: \"class\", literal: Null, line: 1 }")
        );

        scanner = Scanner::new(String::from("print"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('p'));

        scanner.advance();
        scanner.identifier();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            String::from("Token { token_type: Print, lexeme: \"print\", literal: Null, line: 1 }")
        );
    }

    #[test]
    fn test_number() {
        let mut scanner = Scanner::new(String::from("1"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('1'));

        scanner.advance();
        scanner.number();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            String::from(
                "Token { token_type: Number, lexeme: \"1\", literal: Double(1.0), line: 1 }"
            )
        );

        scanner = Scanner::new(String::from("1.234"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('1'));

        scanner.advance();
        scanner.number();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            String::from(
                "Token { token_type: Number, lexeme: \"1.234\", literal: Double(1.234), line: 1 }"
            )
        );
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new(String::from("\"\""));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('"'));

        scanner.advance();
        scanner.string();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            String::from(
                "Token { token_type: String, lexeme: \"\\\"\\\"\", literal: String(\"\"), line: 1 }"
            )
        );

        scanner = Scanner::new(String::from("\"Hello!\""));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('"'));

        scanner.advance();
        scanner.string();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            String::from(
                "Token { token_type: String, lexeme: \"\\\"Hello!\\\"\", literal: String(\"Hello!\"), line: 1 }"
            )
        );

        scanner = Scanner::new(String::from("\"Hello!"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('"'));

        scanner.advance();
        scanner.string();

        assert!(scanner.had_error);
    }

    #[test]
    fn test_block_comment() {
        let mut scanner = Scanner::new(String::from("/* This is a block comment */"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('/'));

        scanner.advance();
        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('*'));

        scanner.advance();

        let mut res = scanner.block_comment();

        assert_eq!(res, Ok(()));
        assert_eq!(scanner.tokens.len(), 0);

        scanner = Scanner::new(String::from("/* This is a block\ncomment */1 + 2"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('/'));

        scanner.advance();
        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('*'));

        scanner.advance();

        res = scanner.block_comment();

        assert_eq!(res, Ok(()));
        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('1'));

        scanner = Scanner::new(String::from("/* This is not a block comment *"));

        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('/'));

        scanner.advance();
        assert_eq!(scanner.tokens.len(), 0);
        assert_eq!(scanner.peek(), Some('*'));

        scanner.advance();

        res = scanner.block_comment();
        assert_eq!(res, Err("Unterminated block comment, expected '/'."));
    }

    #[test]
    fn test_match_token() {
        let mut scanner = Scanner::new(String::from("Hello!"));

        assert!(scanner.match_token('H'));
        scanner.current = 1;
        assert!(scanner.match_token('e'));
        scanner.current = 2;
        assert!(scanner.match_token('l'));
        scanner.current = 3;
        assert!(scanner.match_token('l'));
        scanner.current = 4;
        assert!(scanner.match_token('o'));
        scanner.current = 5;
        assert!(scanner.match_token('!'));
    }

    #[test]
    #[should_panic]
    fn test_match_token_eof() {
        let mut scanner = Scanner::new(String::from(""));
        assert!(scanner.match_token('a'));
    }

    #[test]
    fn test_peek() {
        let mut scanner = Scanner::new(String::from("Hello!"));

        assert_eq!(scanner.peek(), Some('H'));
        scanner.current = 1;
        assert_eq!(scanner.peek(), Some('e'));
        scanner.current = 2;
        assert_eq!(scanner.peek(), Some('l'));
        scanner.current = 3;
        assert_eq!(scanner.peek(), Some('l'));
        scanner.current = 4;
        assert_eq!(scanner.peek(), Some('o'));
        scanner.current = 5;
        assert_eq!(scanner.peek(), Some('!'));
        scanner.current = 6;
        assert_eq!(scanner.peek(), Some('\0'));
    }

    #[test]
    fn test_peek_next() {
        let mut scanner = Scanner::new(String::from("Hello!"));

        assert_eq!(scanner.peek_next(), Some('e'));
        scanner.current = 1;
        assert_eq!(scanner.peek_next(), Some('l'));
        scanner.current = 2;
        assert_eq!(scanner.peek_next(), Some('l'));
        scanner.current = 3;
        assert_eq!(scanner.peek_next(), Some('o'));
        scanner.current = 4;
        assert_eq!(scanner.peek_next(), Some('!'));
        scanner.current = 5;
        assert_eq!(scanner.peek_next(), Some('\0'));
        scanner.current = 6;
        assert_eq!(scanner.peek_next(), Some('\0'));
    }

    #[test]
    fn test_is_alpha() {
        let mut scanner = Scanner::new(String::from(""));

        let c = 'a';
        assert!(scanner.is_alpha(&c));

        let c = 'A';
        assert!(scanner.is_alpha(&c));

        let c = 'z';
        assert!(scanner.is_alpha(&c));

        let c = 'Z';
        assert!(scanner.is_alpha(&c));

        let c = ';';
        assert!(!scanner.is_alpha(&c));

        let c = '0';
        assert!(!scanner.is_alpha(&c));
    }

    #[test]
    fn test_is_digit() {
        let mut scanner = Scanner::new(String::from(""));

        let c = '0';
        assert!(scanner.is_digit(&c));

        let c = '1';
        assert!(scanner.is_digit(&c));

        let c = 'a';
        assert!(!scanner.is_digit(&c));

        let c = ';';
        assert!(!scanner.is_digit(&c));
    }

    #[test]
    fn test_advance() {
        let mut scanner = Scanner::new(String::from("Hello!"));

        assert_eq!(scanner.advance(), Some('H'));
        assert_eq!(scanner.advance(), Some('e'));
        assert_eq!(scanner.advance(), Some('l'));
        assert_eq!(scanner.advance(), Some('l'));
        assert_eq!(scanner.advance(), Some('o'));
        assert_eq!(scanner.advance(), Some('!'));
        assert_eq!(scanner.advance(), None);
    }

    #[test]
    fn test_add_token() {
        let mut scanner = Scanner::new(String::from("1.234 \"hello\""));

        assert_eq!(scanner.tokens.len(), 0);

        scanner.start = 0;
        scanner.current = 5;
        scanner.add_token(TokenType::Number, Literal::Double(1.234));
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(
            format!("{:?}", scanner.tokens[0]),
            String::from(
                "Token { token_type: Number, lexeme: \"1.234\", literal: Double(1.234), line: 1 }"
            )
        );

        scanner.start = 6;
        scanner.current = 13;
        scanner.add_token(TokenType::String, Literal::String(String::from("hello")));
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(
            format!("{:?}", scanner.tokens[1]),
            String::from(
                "Token { token_type: String, lexeme: \"\\\"hello\\\"\", literal: String(\"hello\"), line: 1 }"
            )
        );
    }

    #[test]
    fn test_substring() {
        let mut scanner = Scanner::new(String::from("Hello!"));

        assert_eq!(scanner.substring(0, 0), String::from(""));
        assert_eq!(scanner.substring(0, 1), String::from("H"));
        assert_eq!(scanner.substring(1, 2), String::from("e"));
        assert_eq!(scanner.substring(0, 2), String::from("He"));
        assert_eq!(scanner.substring(0, 6), String::from("Hello!"));
        assert_eq!(scanner.substring(0, 7), String::from("Hello!"));
    }
}
