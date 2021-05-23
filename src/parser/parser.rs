use crate::grammar::expr::Expr;
use crate::token::token::Token;
use crate::token::token_type::TokenType;
use std::rc::Rc;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    cursor: usize,
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
        Self { tokens, cursor: 0 }
    }

    // expression → equality ;
    fn expression(&mut self) -> Expr {
       let expr = self.equality();
       *expr
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    pub fn equality(&mut self) -> Box<Expr<'a>> {
        let left_expr = self.comparison();
        let mut left_expr = left_expr;

        while let Some(ref token) = self.tokens.get(self.cursor) {
            match token.token_type {
                TokenType::Bang | TokenType::EqualEqual => {
                    self.cursor += 1;
                    left_expr = Box::new(Expr::Binary(left_expr, token, self.comparison()));
                }
                _ => break
            }
        }

       left_expr
    }

    pub fn comparison(&mut self) -> Box<Expr<'a>> {
        self.cursor += 1;
        Box::new(Expr::Literal("1".to_string()))
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
        let mut parser = Parser::new(tokens);
        let res = parser.expression();
        println!("Check {:?}", res);
        // assert_eq!(Expr::Literal, *res)
    }
}
