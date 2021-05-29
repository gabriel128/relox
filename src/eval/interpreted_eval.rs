use std::fmt;
use crate::grammar::expr::Expr;
use crate::grammar::expr::ExprLiteral;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
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
    fn eval(&self) -> Result<EvalResult, String>;
}

impl Eval for Expr<'_> {
    fn eval(&self) -> Result<EvalResult, String> {
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

fn handle_unary(token: &Token, evaled_expr: EvalResult) -> Result<EvalResult, String> {
    match (token.token_type, evaled_expr) {
        (TokenType::Minus, EvalResult::Number(the_num)) => Ok(EvalResult::Number(-the_num)),
        (TokenType::Bang, EvalResult::Bool(a_bool)) => Ok(EvalResult::Bool(!a_bool)),
        (token_type, result) => Err(format!("{:?} {}", token_type, result)),
    }
}
fn handle_binary(
    token: &Token,
    evaled_left: EvalResult,
    evaled_right: EvalResult,
) -> Result<EvalResult, String> {
    match (token.token_type, evaled_left, evaled_right) {
        (TokenType::Plus, EvalResult::Number(x), EvalResult::Number(y)) => {
            Ok(EvalResult::Number(x + y))
        }
        (TokenType::Plus, EvalResult::String(x), EvalResult::String(ref y)) => {
            // TODO
            Ok(EvalResult::String(x + y))
        }
        (TokenType::Plus, _, _) => {
            Err("RuntimeError: sum parameters must be both numbers or both strings".to_string())
        }
        (token_type, result, result2) => Err(format!("{:?} can't handle {} {}", token_type, result, result2)),
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
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(3.0), res.eval().unwrap());

        let mut scanner = Scanner::new("\"a\" + \"b\"".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::String("ab".to_string()), res.eval().unwrap());
    }

    #[test]
    fn test_unary_number_eval() {
        let mut scanner = Scanner::new("-1".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(-1.0), res.eval().unwrap());

        let mut scanner = Scanner::new("-30.0".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(-30.0), res.eval().unwrap());

        let mut scanner = Scanner::new("-true".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(
            "Fatal error, unary not matched (Minus, Bool(true))",
            res.eval().expect_err("")
        );
    }

    #[test]
    fn test_unary_bool_eval() {
        let mut scanner = Scanner::new("!true".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Bool(false), res.eval().unwrap());

        let mut scanner = Scanner::new("!false".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Bool(true), res.eval().unwrap());

        let mut scanner = Scanner::new("!2".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(
            "Fatal error, unary not matched (Bang, Number(2.0))",
            res.eval().expect_err("")
        );
    }

    #[test]
    fn test_grouping_eval() {
        let mut scanner = Scanner::new("(1)".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().unwrap();
        assert_eq!(EvalResult::Number(1.0), res.eval().unwrap());
    }
}
