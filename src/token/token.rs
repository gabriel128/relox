use crate::token::token_type::TokenType;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    pub literal: Option<Literal>,
    line: usize
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Double(f64),
    AString(String),
}


impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        Token{ token_type, lexeme: lexeme.to_string(), literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
