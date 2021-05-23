use super::*;
use crate::token::token_type::*;

#[test]
fn single_chars() {
    let mut scanner = Scanner::new("(".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(
            TokenType::LeftParen,
            "(",
            None,
            1,
        ),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);

    let mut scanner = Scanner::new("()! \n  /  ".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(
            TokenType::LeftParen,
            "(",
            None,
            1,
        ),
        Token::new(
            TokenType::RightParen,
            ")",
            None,
            1,
        ),
        Token::new(
            TokenType::Bang,
            "!",
            None,
            1,
        ),
        Token::new(
            TokenType::Slash,
            "/",
            None,
            2,
        ),
        Token::new(TokenType::Eof, "", None, 2),
    ];
    assert_eq!(*tokens, result);
}

#[test]
fn multiple_char() {
    let mut scanner = Scanner::new("!<// blah blah blah".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(
            TokenType::Bang,
            "!",
            None,
            1,
        ),
        Token::new(
            TokenType::Less,
            "<",
            None,
            1,
        ),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);

    let mut scanner = Scanner::new("<= // blah \n !".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(
            TokenType::LessEqual,
            "<=",
            None,
            1,
        ),
        Token::new(
            TokenType::Bang,
            "!",
            None,
            2,
        ),
        Token::new(TokenType::Eof, "", None, 2),
    ];
    assert_eq!(*tokens, result);
}

#[test]
fn strings() {
    let mut scanner = Scanner::new("\"whatever )\"".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(TokenType::String, "whatever )", None, 1),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);

    let mut scanner = Scanner::new("\"whatever ) \n \"".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(TokenType::String, "whatever ) \n ", None, 2),
        Token::new(TokenType::Eof, "", None, 2),
    ];
    assert_eq!(*tokens, result);
}

#[test]
fn numbers() {
    let mut scanner = Scanner::new("11".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(TokenType::Number, "11", Some(Literal::Double(11.0)), 1),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);

    let mut scanner = Scanner::new("11.32".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(TokenType::Number, "11.32", Some(Literal::Double(11.32)), 1),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);

    let mut scanner = Scanner::new("11.".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(TokenType::Number, "11", Some(Literal::Double(11.0)), 1),
        Token::new(TokenType::Dot, ".", None, 1),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);

    let mut scanner = Scanner::new("11.12.".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(TokenType::Number, "11.12", Some(Literal::Double(11.12)), 1),
        Token::new(TokenType::Dot, ".", None, 1),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);
}

#[test]
fn number_followed_by_something() {
    let mut scanner = Scanner::new("11(".to_string());
    let tokens = scanner.scan_tokens();
    let result = vec![
        Token::new(TokenType::Number, "11", Some(Literal::Double(11.0)), 1),
        Token::new(TokenType::LeftParen, "(", None, 1),
        Token::new(TokenType::Eof, "", None, 1),
    ];
    assert_eq!(*tokens, result);

}

#[test]
fn keywords_and_identifiers() {
    let mut scanner = Scanner::new("or and     \n orfelia caca".to_string());
    let tokens = scanner.scan_tokens();

    let result = vec![
        Token::new(TokenType::Or, "or", None, 1),
        Token::new(TokenType::And, "and", None, 1),
        Token::new(TokenType::Identifier, "orfelia", None, 2),
        Token::new(TokenType::Identifier, "caca", None, 2),
        Token::new(TokenType::Eof, "", None, 2),
    ];
    assert_eq!(*tokens, result);
}

#[test]
fn mix_of_stuff() {
    let mut scanner = Scanner::new("  42 \"sdfsdf\" // nope \n )".to_string());
    let tokens = scanner.scan_tokens();

    let result = vec![
        Token::new(TokenType::Number, "42", Some(Literal::Double(42.0)), 1),
        Token::new(TokenType::String, "sdfsdf", None, 1),
        Token::new(TokenType::RightParen, ")", None, 2),
        Token::new(TokenType::Eof, "", None, 2),
    ];
    assert_eq!(*tokens, result);

    let mut scanner = Scanner::new("or \"sdfsdf\")//nope\n}(".to_string());
    let tokens = scanner.scan_tokens();

    let result = vec![
        Token::new(TokenType::Or, "or", None, 1),
        Token::new(TokenType::String, "sdfsdf", None, 1),
        Token::new(TokenType::RightParen, ")", None, 1),
        Token::new(TokenType::RightBrace, "}", None, 2),
        Token::new(TokenType::LeftParen, "(", None, 2),
        Token::new(TokenType::Eof, "", None, 2),
    ];
    assert_eq!(*tokens, result);
}
