use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleCharTokens {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashOrComment {
    Slash
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OneOrTwoCharTokens {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    SingleChar(SingleCharTokens),
    OneOrTwoChar(OneOrTwoCharTokens),
    SlashOrComment(SlashOrComment),

    // Literals.
    Identifier,
    String,
    Number,

    //

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

    // Skip
    Skip,
    // New Line
    NewLine,
}

impl TokenType {
    pub fn from_single_char(a_char: &char) -> Option<TokenType> {
        let mut char_to_enum = HashMap::<char, TokenType>::new();
        char_to_enum.insert('(', TokenType::SingleChar(SingleCharTokens::LeftParen));
        char_to_enum.insert(')', TokenType::SingleChar(SingleCharTokens::RightParen));
        char_to_enum.insert('{', TokenType::SingleChar(SingleCharTokens::LeftBrace));
        char_to_enum.insert('}', TokenType::SingleChar(SingleCharTokens::RightParen));
        char_to_enum.insert(',', TokenType::SingleChar(SingleCharTokens::Comma));
        char_to_enum.insert('.', TokenType::SingleChar(SingleCharTokens::Dot));
        char_to_enum.insert('-', TokenType::SingleChar(SingleCharTokens::Minus));
        char_to_enum.insert('+', TokenType::SingleChar(SingleCharTokens::Plus));
        char_to_enum.insert(';', TokenType::SingleChar(SingleCharTokens::Semicolon));
        char_to_enum.insert('*', TokenType::SingleChar(SingleCharTokens::Star));

        // One or More Chars
        char_to_enum.insert('!', TokenType::OneOrTwoChar(OneOrTwoCharTokens::Bang));
        char_to_enum.insert('=', TokenType::OneOrTwoChar(OneOrTwoCharTokens::Equal));
        char_to_enum.insert('<', TokenType::OneOrTwoChar(OneOrTwoCharTokens::Less));
        char_to_enum.insert('>', TokenType::OneOrTwoChar(OneOrTwoCharTokens::Greater));

        // Slash
        char_to_enum.insert('/', TokenType::SlashOrComment(SlashOrComment::Slash));

        // Skip
        char_to_enum.insert(' ', TokenType::Skip);
        char_to_enum.insert('\r', TokenType::Skip);
        char_to_enum.insert('\t', TokenType::Skip);

        // New Line
        char_to_enum.insert('\n', TokenType::NewLine);

        // Sring
        char_to_enum.insert('"', TokenType::String);

        char_to_enum.get(a_char).map (|the_type| the_type.clone())
    }

    pub fn from_two_chars(first_char: &char, second_char: &char, fallback_token_type: TokenType) -> (TokenType, String, usize) {
        let mut str_to_enum = HashMap::<String, TokenType>::new();
        str_to_enum.insert("!=".to_string(), TokenType::OneOrTwoChar(OneOrTwoCharTokens::BangEqual));
        str_to_enum.insert("==".to_string(), TokenType::OneOrTwoChar(OneOrTwoCharTokens::EqualEqual));
        str_to_enum.insert("<=".to_string(), TokenType::OneOrTwoChar(OneOrTwoCharTokens::LessEqual));
        str_to_enum.insert(">=".to_string(), TokenType::OneOrTwoChar(OneOrTwoCharTokens::GreaterEqual));

        let mut as_str = String::new();
        as_str.push(*first_char);
        as_str.push(*second_char);


        if let Some(token_type) = str_to_enum.get(&as_str) {
            (*token_type, as_str, 2)
        } else {
            (fallback_token_type, first_char.to_string(), 1)
        }
    }

    pub fn is_comment(first_char: char, second_char: char) -> bool {
        let res = first_char == '/' && second_char == '/';
        // eprintln!("Got here {}, {}, {}", res, first_char, second_char);
        res
    }
}
