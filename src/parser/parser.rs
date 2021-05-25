use crate::error_handler;
use crate::grammar::expr::ExprLiteral;
use crate::grammar::expr::Expr;
use crate::token::token::Token;
use crate::token::token::Literal as TokenLiteral;
use crate::token::token_type::TokenType;

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
                _ => break,
            }
        }

        left_expr
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    pub fn comparison(&mut self) -> Box<Expr<'a>> {
        let left_expr = self.term();
        let mut left_expr = left_expr;

        while let Some(ref token) = self.tokens.get(self.cursor) {
            match token.token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    self.cursor += 1;
                    left_expr = Box::new(Expr::Binary(left_expr, token, self.term()));
                }
                _ => break,
            }
        }

        left_expr
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    pub fn term(&mut self) -> Box<Expr<'a>> {
        let left_expr = self.factor();
        let mut left_expr = left_expr;

        while let Some(ref token) = self.tokens.get(self.cursor) {
            match token.token_type {
                TokenType::Minus | TokenType::Plus => {
                    self.cursor += 1;
                    left_expr = Box::new(Expr::Binary(left_expr, token, self.factor()));
                }
                _ => break,
            }
        }

        left_expr
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    pub fn factor(&mut self) -> Box<Expr<'a>> {
        let left_expr = self.unary();
        let mut left_expr = left_expr;

        while let Some(ref token) = self.tokens.get(self.cursor) {
            match token.token_type {
                TokenType::Minus | TokenType::Plus => {
                    self.cursor += 1;
                    left_expr = Box::new(Expr::Binary(left_expr, token, self.unary()));
                }
                _ => break,
            }
        }

        left_expr
    }

    // unary → ( "!" | "-" ) unary | primary ;
    pub fn unary(&mut self) -> Box<Expr<'a>> {
        if let Some(ref token) = self.tokens.get(self.cursor) {
            match token.token_type {
                TokenType::Bang | TokenType::Minus => {
                    self.cursor += 1;
                    Box::new(Expr::Unary(token, self.primary()))
                },
                _ => self.primary()
            }
        } else {
            self.primary()
        }

    }

    // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    pub fn primary(&mut self) -> Box<Expr<'a>> {
        if let Some(ref token) = self.tokens.get(self.cursor) {
            match (token.token_type, token.literal.as_ref()) {
                (TokenType::True, _) => {
                    self.cursor += 1;
                    let literal = Some(ExprLiteral::Bool(true));
                    Box::new(Expr::Literal(literal))
                }
                (TokenType::False, _) => {
                    self.cursor += 1;
                    let literal = Some(ExprLiteral::Bool(false));
                    Box::new(Expr::Literal(literal))
                }
                (TokenType::Nil, _) => {
                    self.cursor += 1;
                    Box::new(Expr::Literal(None))
                }
                (TokenType::Number, Some(TokenLiteral::Double(num_literal))) => {
                    self.cursor += 1;
                    let literal = Some(ExprLiteral::Number(*num_literal));
                    Box::new(Expr::Literal(literal))
                }
                (TokenType::String, Some(TokenLiteral::String(string_literal))) => {
                    self.cursor += 1;
                    let with_quotes = "\"".to_owned() + string_literal + "\"";
                    let literal = Some(ExprLiteral::String(with_quotes.to_string()));
                    Box::new(Expr::Literal(literal))
                }
                token => {
                    panic!("Parser Error: got this {:?}", token);
                    // self.cursor += 1;
                    // error_handler::error(0, "Unterminate string");
                    // Box::new(Expr::Literal(None))
                }
            }
        } else {
            panic!("Out of bounds")
         }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scanner;

    #[test]
    fn test_parsing_random0() {
        let mut scanner = Scanner::new("5 == 1 + 2".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.expression();
        assert_eq!("(5 == (1 + 2))", format!("{}", res, ));
    }

    #[test]
    fn test_parsing_random1() {
        let mut scanner = Scanner::new("5 <=    1 - 2".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.expression();
        assert_eq!("(5 <= (1 - 2))", format!("{}", res, ));
    }

    #[test]
    fn test_parsing_random2() {
        let mut scanner = Scanner::new("false - 2 + 3 + 4 == 2 == true <= 10".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.expression();
        assert_eq!("(((((false - 2) + 3) + 4) == 2) == (true <= 10))", format!("{}", res, ));
    }

    #[test]
    fn test_string_equality() {
        let mut scanner = Scanner::new("\"epppa\" == \"epppa\"".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.expression();
        assert_eq!("(\"epppa\" == \"epppa\")", format!("{}", res, ));
    }
}
