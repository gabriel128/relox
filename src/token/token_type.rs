use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Slash Or Comment
    Slash,

    // One Or two Char
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // SingleChar
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

    Skip,
    NewLine,
    ErrorToken,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    OneOrTwoChar,
    SingleChar,
    SlashOrComment,
    NoOp,
    Rest,
}

impl TokenType {
    pub fn from_single_char(a_char: char) -> Option<(TokenType, TokenKind)> {
        let mut char_to_enum = HashMap::<char, (TokenType, TokenKind)>::new();
        char_to_enum.insert('(', (TokenType::LeftParen, TokenKind::SingleChar));
        char_to_enum.insert(')', (TokenType::RightParen, TokenKind::SingleChar));
        char_to_enum.insert('{', (TokenType::LeftBrace, TokenKind::SingleChar));
        char_to_enum.insert('}', (TokenType::RightBrace, TokenKind::SingleChar));
        char_to_enum.insert(',', (TokenType::Comma, TokenKind::SingleChar));
        char_to_enum.insert('.', (TokenType::Dot, TokenKind::SingleChar));
        char_to_enum.insert('-', (TokenType::Minus, TokenKind::SingleChar));
        char_to_enum.insert('+', (TokenType::Plus, TokenKind::SingleChar));
        char_to_enum.insert(';', (TokenType::Semicolon, TokenKind::SingleChar));
        char_to_enum.insert('*', (TokenType::Star, TokenKind::SingleChar));

        // One or More Chars
        //
        char_to_enum.insert('!', (TokenType::Bang, TokenKind::OneOrTwoChar));
        char_to_enum.insert('=', (TokenType::Equal, TokenKind::OneOrTwoChar));
        char_to_enum.insert('<', (TokenType::Less, TokenKind::OneOrTwoChar));
        char_to_enum.insert('>', (TokenType::Greater, TokenKind::OneOrTwoChar));

        // Slash
        char_to_enum.insert('/', (TokenType::Slash, TokenKind::SlashOrComment));

        // Skip
        char_to_enum.insert(' ', (TokenType::Skip, TokenKind::NoOp));
        char_to_enum.insert('\r', (TokenType::Skip, TokenKind::NoOp));
        char_to_enum.insert('\t', (TokenType::Skip, TokenKind::NoOp));

        // New Line
        char_to_enum.insert('\n', (TokenType::NewLine, TokenKind::Rest));

        // Sring
        char_to_enum.insert('"', (TokenType::String, TokenKind::Rest));
        // Number
        char_to_enum.insert('0', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('1', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('2', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('3', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('4', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('5', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('6', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('7', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('8', (TokenType::Number, TokenKind::Rest));
        char_to_enum.insert('9', (TokenType::Number, TokenKind::Rest));

        char_to_enum.get(&a_char).copied()
    }

    pub fn from_two_chars(
        first_char: char,
        second_char: char,
        fallback_token_type: TokenType,
    ) -> (TokenType, String, usize) {
        let mut str_to_enum = HashMap::<String, TokenType>::new();
        str_to_enum.insert("!=".to_string(), TokenType::BangEqual);
        str_to_enum.insert("==".to_string(), TokenType::EqualEqual);
        str_to_enum.insert("<=".to_string(), TokenType::LessEqual);
        str_to_enum.insert(">=".to_string(), TokenType::GreaterEqual);

        let mut as_str = String::new();
        as_str.push(first_char);
        as_str.push(second_char);

        if let Some(token_type) = str_to_enum.get(&as_str) {
            (*token_type, as_str, 2)
        } else {
            (fallback_token_type, first_char.to_string(), 1)
        }
    }

    pub fn keyword(a_string: &str) -> Option<TokenType> {
        let mut keywords = HashMap::<String, TokenType>::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        keywords.get(a_string).copied()
    }

    pub fn is_comment(first_char: char, second_char: char) -> bool {
        first_char == '/' && second_char == '/'
    }
}
