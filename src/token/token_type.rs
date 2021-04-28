use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    pub fn from_char(a_char: char) -> Option<TokenType> {
        let mut char_to_enum = HashMap::<char, TokenType>::new();
        char_to_enum.insert('(', TokenType::LeftParen);
        char_to_enum.insert(')', TokenType::RightParen);
        char_to_enum.insert('{', TokenType::LeftBrace);
        char_to_enum.insert('}', TokenType::RightParen);
        char_to_enum.insert(',', TokenType::Comma);
        char_to_enum.insert('.', TokenType::Dot);
        char_to_enum.insert('-', TokenType::Minus);
        char_to_enum.insert('+', TokenType::Plus);
        char_to_enum.insert(';', TokenType::Semicolon);
        char_to_enum.insert('*', TokenType::Star);

        char_to_enum.get(&a_char).map (|the_type| the_type.clone())
    }
}
