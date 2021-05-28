use crate::token::token::Token;

pub trait Expr {}

// Grouping def
#[derive(Debug)]
pub struct Grouping<'a, T: Expr> {
    expression: &'a T
}

impl<T: Expr> Expr for Grouping<'_, T> {}

impl<'a, T: Expr> Grouping<'a, T> {
    pub fn new(expression: &'a T) -> Self {
       Self{expression: expression}
    }
}

// Literal def
#[derive(Debug)]
pub struct Literal<'a> {
    value: &'a str
}

impl Expr for Literal<'_> {}

impl<'a> Literal<'a> {
    pub fn new(value: &'a str) -> Self {
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
pub struct Binary<T: Expr> {
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
        let lit = Literal::new("bla");
        let grouping = Grouping::new(&lit);
        assert!(true)

    }
}
