pub mod literal;
pub mod lox;
pub mod scanner;
pub mod token;
pub mod token_type;

#[cfg(test)]
mod tests {
    use super::literal::Literal;
    use super::token::Token;
    use super::token_type::TokenType;

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
