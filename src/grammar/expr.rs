use std::fmt;
use crate::token::token::Token;

#[derive(Debug, PartialEq)]
pub enum ExprLiteral {
   Bool(bool),
   String(String),
   Number(f32),
   Nil
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(ExprLiteral),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>)
    // More efficient ? Binary(Box<(Expr<'a>, Expr<'a>)>, &'a Token),
}

impl fmt::Display for Expr {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         match self {
            Expr::Binary(left, token, right) => write!(f, "({} {} {})", left, token.lexeme, right),
            Expr::Grouping(val) => write!(f, "(grouping {})", val),
            Expr::Unary(token, right) => write!(f, "{}{}", token.lexeme, right),
            Expr::Literal(ExprLiteral::Nil) => write!(f, "null"),
            Expr::Literal(ExprLiteral::Bool(a_bool)) => write!(f, "{}", a_bool),
            Expr::Literal(ExprLiteral::Number(num)) => write!(f, "{}", num),
            Expr::Literal(ExprLiteral::String(a_string)) => write!(f, "\"{}\"", a_string),
         }
    }
}
