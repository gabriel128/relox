use crate::token::token::Token;

pub trait Expr {}

// Grouping def
#[derive(Debug)]
pub struct Grouping<T: Expr> {
    expression: T
}

impl<T: Expr> Expr for Grouping<T> {}

impl<T: Expr> Grouping<T> {
    pub fn new(expression: T) -> Self {
       Self{expression: expression}
    }
}

// Literal def
#[derive(Debug)]
pub struct Literal {
    value: String
}

impl Expr for Literal {}

impl Literal {
    pub fn new(value: String) -> Self {
       Literal{value}
    }
}

#[derive(Debug)]
struct Unary<T: Expr> {
    value: T,
    operator: Token
}

impl<T> Expr for Unary<T> where T: Expr {}

impl<T: Expr> Unary <T> {
    pub fn new(value: T, operator: Token) -> Self {
       Self{value, operator}
    }
}

// Binary def
#[derive(Debug)]
struct Binary<T: Expr> {
    left: T,
    operator: Token,
    right: T
}

impl<T> Expr for Binary<T> where T: Expr {}

impl<T: Expr> Binary <T> {
    pub fn new(left: T, operator: Token, right: T) -> Self {
       Self{left, operator, right}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_expr() {
        let lit = Literal::new("bla".to_string());
        let grouping = Grouping::new(lit);
        assert!(true)

    }
}
