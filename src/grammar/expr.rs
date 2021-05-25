use std::fmt;
use crate::token::token::Token;

#[derive(Debug, PartialEq)]
pub enum ExprLiteral {
   Bool(bool),
   AString(String),
   Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Literal(Option<ExprLiteral>),
    Grouping(Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Unary(&'a Token, Box<Expr<'a>>)
    // More efficient ? Binary(Box<(Expr<'a>, Expr<'a>)>, &'a Token),
}

impl fmt::Display for Expr<'_> {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         match self {
            Expr::Binary(left, token, right) => write!(f, "({} {} {})", left, token.lexeme, right),
            Expr::Grouping(val) => write!(f, "({})", val),
            Expr::Unary(token, right) => write!(f, "{}{}", token.lexeme, right),
            Expr::Literal(None) => write!(f, "null"),
            Expr::Literal(Some(ExprLiteral::Bool(a_bool))) => write!(f, "{}", a_bool),
            Expr::Literal(Some(ExprLiteral::Number(num))) => write!(f, "{}", num),
            Expr::Literal(Some(ExprLiteral::AString(a_string))) => write!(f, "{}", a_string),
         }
    }
}
