use crate::token::token::Token;
use crate::token::token_type::TokenType;
use crate::grammar::expr::Expr;
use crate::grammar::expr::ExprLiteral;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Number(f64),
    String(String),
    Bool(bool),
    Null
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
            Expr::Literal(None) => Ok(EvalResult::Null),
            Expr::Literal(Some(ExprLiteral::Bool(a_bool))) => Ok(EvalResult::Bool(*a_bool)),
            Expr::Literal(Some(ExprLiteral::Number(num))) => Ok(EvalResult::Number(*num)),
            Expr::Literal(Some(ExprLiteral::String(a_string))) => Ok(EvalResult::String(a_string.to_string())),
         }
    }

}

fn handle_unary(token: &Token, evaled_expr: EvalResult) -> Result<EvalResult, String> {
    match (token.token_type, evaled_expr) {
       (TokenType::Minus, EvalResult::Number(the_num)) => Ok(EvalResult::Number(-the_num)),
       (TokenType::Bang, EvalResult::Bool(a_bool)) => Ok(EvalResult::Bool(!a_bool)),
        non_matched=> Err(format!("Fatal error, unary not matched {:?}", non_matched))
    }
}
fn handle_binary(token: &Token, evaled_left: EvalResult, evaled_right: EvalResult) -> Result<EvalResult, String> {
    match (token.token_type, evaled_left, evaled_right) {
       (TokenType::Plus, EvalResult::Number(x), EvalResult::Number(y)) => Ok(EvalResult::Number(x + y)),
        non_matched=> Err(format!("Fatal error, binary not matched {:?}", non_matched))
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
        assert_eq!("Fatal error, unary not matched (Minus, Bool(true))", res.eval().expect_err(""));
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
        assert_eq!("Fatal error, unary not matched (Bang, Number(2.0))", res.eval().expect_err(""));
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
