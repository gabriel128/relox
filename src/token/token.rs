use crate::token::token_type::TokenType;
use std::fmt;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token{ token_type, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
