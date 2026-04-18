use crate::literal::Literal;
use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::TokenType;
    use crate::literal::Literal;

    #[test]
    fn test_token_format() {
        let token = Token::new(
            TokenType::Number,
            String::from("1.234"),
            Literal::Double(1.234),
            1,
        );
        assert_eq!(
            format!("{:?}", token),
            String::from(
                "Token { token_type: Number, lexeme: \"1.234\", literal: Double(1.234), line: 1 }"
            )
        );

        let token = Token::new(
            TokenType::String,
            String::from("\"hello\""),
            Literal::String(String::from("hello")),
            1,
        );
        assert_eq!(
            format!("{:?}", token),
            String::from(
                "Token { token_type: String, lexeme: \"\\\"hello\\\"\", literal: String(\"hello\"), line: 1 }"
            )
        );

        let token = Token::new(TokenType::And, String::from("and"), Literal::Null, 1);
        assert_eq!(
            format!("{:?}", token),
            String::from("Token { token_type: And, lexeme: \"and\", literal: Null, line: 1 }")
        );
    }
}
