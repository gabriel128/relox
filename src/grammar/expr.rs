use crate::token::token::Token;

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(String),
    Grouping(Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    // Binary(&'a Expr<'a>, &'a Token, &'a Expr<'a>),
    // Unary(&'a Token, &'a Expr<'a>)
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
