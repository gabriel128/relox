use crate::grammar::expr::Expr;
use crate::token::token::Token;
use crate::token::token_type::TokenType;
use std::rc::Rc;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
//
// Recursive descent implementation
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        // println!("The tokens are {:?}", tokens);
        Self { tokens, current: 0 }
    }

    // expression → equality ;
    // fn expression(&mut self) -> &Expr {
    //    self.equality()
    // }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    // fn equality(&mut self, current: usize) -> Rc<Expr> {
    pub fn equality(&self, current: usize) -> (Box<Expr<'a>>, usize) {
        let (left_expr, new_current) = self.comparison(current);

        // println!("Current {:?}", current);
        let mut left_expr = left_expr;
        let mut current = new_current;
        while let Some(ref token) = self.tokens.get(current) {
            match token.token_type {
                TokenType::Bang | TokenType::EqualEqual => {
                    current += 1;
                    let (right_expr, new_current) = self.comparison(current);
                    left_expr = Box::new(Expr::Binary(left_expr, token, right_expr));
                    current = new_current;
                }
                _ => break
            }
        }

       (left_expr, current)
    }

    pub fn comparison(&self, current: usize) -> (Box<Expr<'a>>, usize) {
        (Box::new(Expr::Literal("1".to_string())), current+1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scanner;

    #[test]
    fn test_parsing_equality() {
        let mut scanner = Scanner::new("1 == 2 == 3".to_string());
        let tokens = scanner.scan_tokens();
        let parser = Parser::new(tokens);
        let (res, _) = parser.equality(0);
        println!("Check {:?}", *res);
        // assert_eq!(Expr::Literal, *res)
    }
}
