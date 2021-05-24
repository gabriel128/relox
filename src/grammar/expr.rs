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
    // Binary(&'a Expr<'a>, &'a Token, &'a Expr<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_expr() {
        // let lit = Expr::Literal("bla".to_string());
        // let grouping = Expr::Grouping(Rc::new(lit));
        // println!("Grouping is {:?}", grouping);
        // assert!(true)

    }
}
