use crate::grammar::expr::Expr;
use crate::grammar::expr::ExprLiteral;
use crate::token::token::Token;
use crate::token::token_type::TokenType;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, PartialEq)]
pub struct EvalError {
    pub line: usize,
    pub message: String,
}

impl EvalError {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalResult::Number(val) => write!(f, "{}", val),
            EvalResult::String(val) => write!(f, "{}", val),
            EvalResult::Bool(val) => write!(f, "{}", val),
            EvalResult::Nil => write!(f, "nil"),
        }
    }
}

pub trait Eval {
    fn eval(&self) -> Result<EvalResult, EvalError>;
}

impl Eval for Expr<'_> {
    fn eval(&self) -> Result<EvalResult, EvalError> {
        match self {
            Expr::Binary(left, token, right) => handle_binary(token, left.eval()?, right.eval()?),
            Expr::Grouping(val) => val.eval(),
            Expr::Unary(token, right) => handle_unary(*token, right.eval()?),
            Expr::Literal(ExprLiteral::Nil) => Ok(EvalResult::Nil),
            Expr::Literal(ExprLiteral::Bool(a_bool)) => Ok(EvalResult::Bool(*a_bool)),
            Expr::Literal(ExprLiteral::Number(num)) => Ok(EvalResult::Number(*num)),
            Expr::Literal(ExprLiteral::String(a_string)) => {
                Ok(EvalResult::String(a_string.to_string()))
            }
        }
    }
}

fn handle_unary(token: &Token, evaled_expr: EvalResult) -> Result<EvalResult, EvalError> {
    match (token.token_type, evaled_expr) {
        (TokenType::Minus, EvalResult::Number(the_num)) => Ok(EvalResult::Number(-the_num)),
        (TokenType::Bang, EvalResult::Bool(a_bool)) => Ok(EvalResult::Bool(!a_bool)),
        (token_type, result) => {
            let message = format!("{:?} {}", token_type, result);
            Err(EvalError::new(token.line, message))
        }
    }
}
fn handle_binary(
    token: &Token,
    evaled_left: EvalResult,
    evaled_right: EvalResult,
) -> Result<EvalResult, EvalError> {
    match (token.token_type, evaled_left, evaled_right) {
        (TokenType::Plus, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Number(x + y))
        }
        (TokenType::Plus, EvalResult::String(x), EvalResult::String(ref y)) => {
            Ok(EvalResult::String(x + y))
        }
        (TokenType::Plus, _, _) => {
            let message =
                "sum parameters must be both numbers or both strings".to_string();
            Err(EvalError::new(token.line, message))
        }
        (TokenType::Minus, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Number(x - y))
        }
        (TokenType::Minus, _, _) => {
            let message =
                "substraction parameters must be both numbers or both strings"
                    .to_string();
            Err(EvalError::new(token.line, message))
        }
        (TokenType::Star, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Number(x * y))
        }
        (TokenType::Star, _, _) => {
            let message =
                "Multiplication parameters must be both numbers or both strings"
                    .to_string();
            Err(EvalError::new(token.line, message))
        }
        (TokenType::Slash, EvalResult::Number(x), EvalResult::Number(y)) => {
            if y == 0.0 {
                let message = "division by zero is undefined bro".to_string();
                Err(EvalError::new(token.line, message))
            } else {
                Ok(EvalResult::Number(x / y))
            }
        }
        (TokenType::Slash, _, _) => {
            let message = "division parameters must be both numbers or both strings"
                .to_string();
            Err(EvalError::new(token.line, message))
        }
        (TokenType::Greater, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Bool(x > y))
        }
        (TokenType::GreaterEqual, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Bool(x >= y))
        }
        (TokenType::Less, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Bool(x < y))
        }
        (TokenType::LessEqual, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Bool(x <= y))
        }
        (TokenType::BangEqual, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Bool(x != y))
        }
        (TokenType::EqualEqual, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Bool(x == y))
        }
        (TokenType::EqualEqual, EvalResult::String(x), EvalResult::String(y)) => {
            Ok(EvalResult::Bool(x == y))
        }
        (TokenType::EqualEqual, EvalResult::Nil, EvalResult::Nil) => Ok(EvalResult::Bool(true)),
        (TokenType::EqualEqual, EvalResult::Nil, _) => Ok(EvalResult::Bool(false)),
        (TokenType::EqualEqual, _, EvalResult::Nil) => Ok(EvalResult::Bool(false)),
        (TokenType::EqualEqual, _, _) => {
            let message = "you can't compare pears with apples".to_string();
            Err(EvalError::new(token.line, message))
        }
        (TokenType::Nil, _, _) => Ok(EvalResult::Nil),
        (token_type, result, result2) => {
            let message = format!("{:?} can't handle {} {}", token_type, result, result2);
            Err(EvalError::new(token.line, message))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;
    use crate::Scanner;

    #[test]
    fn test_binary_eval() {
        let mut scanner = Scanner::new("1 + 2".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(3.0), res.eval().unwrap());

        let mut scanner = Scanner::new("\"a\" + \"b\"".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::String("ab".to_string()), res.eval().unwrap());
    }

    #[test]
    fn test_unary_number_eval() {
        let mut scanner = Scanner::new("-1".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(-1.0), res.eval().unwrap());

        let mut scanner = Scanner::new("-30.0".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(-30.0), res.eval().unwrap());

        let mut scanner = Scanner::new("-true".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalError::new(1, "Minus true".to_string()), res.eval().expect_err(""));
    }

    #[test]
    fn test_unary_bool_eval() {
        let mut scanner = Scanner::new("!true".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Bool(false), res.eval().unwrap());

        let mut scanner = Scanner::new("!false".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Bool(true), res.eval().unwrap());

        let mut scanner = Scanner::new("!2".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalError::new(1, "Bang 2".to_string()), res.eval().expect_err(""));
    }

    #[test]
    fn test_grouping_eval() {
        let mut scanner = Scanner::new("(1)".to_string());
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(1.0), res.eval().unwrap());
    }
}
