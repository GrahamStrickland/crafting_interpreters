use crate::literal::Literal;
use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: i32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
