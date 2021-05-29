use crate::error_handler;
use crate::grammar::expr::Expr;
use crate::grammar::expr::ExprLiteral;
use crate::token::token::Literal as TokenLiteral;
use crate::token::token::Token;
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

    pub fn parse(&mut self) -> Result<Expr<'a>, String> {
        let expr = self.expression()?;
        Ok(*expr)
    }

    fn one_or_many<F>(&mut self, token_types: Vec<TokenType>, mut f: F,) -> Result<Box<Expr<'a>>, String>
    where
        F: FnMut(&mut Self) -> Result<Box<Expr<'a>>, String>,
    {
        let left_expr = f(self)?;
        let mut left_expr = left_expr;

        while let Some(ref token) = self.tokens.get(self.cursor) {
            if token_types.contains(&token.token_type) {
                self.cursor += 1;
                left_expr = Box::new(Expr::Binary(left_expr, token, f(self)?));
            } else {
                break;
            }
        }

        Ok(left_expr)
    }

    // expression → equality ;
    fn expression(&mut self) -> Result<Box<Expr<'a>>, String> {
        self.equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Box<Expr<'a>>, String> {
        let token_types = vec![TokenType::Bang, TokenType::EqualEqual];
        self.one_or_many(token_types, |the_self| the_self.comparison())
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Box<Expr<'a>>, String> {
        let token_types = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        self.one_or_many(token_types, |the_self| the_self.term())
    }

    // term → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Box<Expr<'a>>, String> {
        let token_types = vec![TokenType::Minus, TokenType::Plus];
        self.one_or_many(token_types, |the_self| the_self.factor())
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Box<Expr<'a>>, String> {
        let token_types = vec![TokenType::Star, TokenType::Slash];
        self.one_or_many(token_types, |the_self| the_self.unary())
    }

    // unary → ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Box<Expr<'a>>, String> {
        if let Some(ref token) = self.tokens.get(self.cursor) {
            match token.token_type {
                TokenType::Bang | TokenType::Minus => {
                    self.cursor += 1;
                    Ok(Box::new(Expr::Unary(token, self.primary()?)))
                }
                _ => self.primary(),
            }
        } else {
            self.primary()
        }
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Box<Expr<'a>>, String> {
        if let Some(ref token) = self.tokens.get(self.cursor) {
            match (token.token_type, token.literal.as_ref()) {
                (TokenType::True, _) => {
                    self.cursor += 1;
                    let literal = ExprLiteral::Bool(true);
                    Ok(Box::new(Expr::Literal(literal)))
                }
                (TokenType::False, _) => {
                    self.cursor += 1;
                    let literal = ExprLiteral::Bool(false);
                    Ok(Box::new(Expr::Literal(literal)))
                }
                (TokenType::Nil, _) => {
                    self.cursor += 1;
                    let literal = ExprLiteral::Nil;
                    Ok(Box::new(Expr::Literal(literal)))
                }
                (TokenType::Number, Some(TokenLiteral::Double(num_literal))) => {
                    self.cursor += 1;
                    let literal = ExprLiteral::Number(*num_literal);
                    Ok(Box::new(Expr::Literal(literal)))
                }
                (TokenType::String, Some(TokenLiteral::String(string_literal))) => {
                    self.cursor += 1;
                    let literal = ExprLiteral::String(string_literal.to_string());
                    Ok(Box::new(Expr::Literal(literal)))
                }
                (TokenType::LeftParen, _) => {
                    self.cursor += 1;
                    let expr = self.expression()?;
                    self.consume(
                        TokenType::RightParen,
                        "There should be a ')' after expression, duh.",
                    )?;
                    Ok(Box::new(Expr::Grouping(expr)))
                }
                _token => Err(format!("Parser Error: got this {:?}", token)),
            }
        } else {
            panic!("Out of bounds")
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<(), String> {
        if let Some(current_token) = self.tokens.get(self.cursor) {
            if current_token.token_type == token_type {
                self.cursor += 1;
                Ok(())
            } else if current_token.token_type == TokenType::Eof {
                error_handler::report(current_token.line, " at the end", message);
                Err(format!("Parser Error"))
            } else {
                let where_at = format!(" at '{}'", current_token.lexeme);
                error_handler::report(current_token.line, &where_at, message);
                Err(format!("Parser Error"))
            }
        } else {
            panic!("Parser Error: Fatal Error");
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
        let res = parser.parse();
        assert_eq!("(5 == (1 + 2))", format!("{}", res.unwrap(),));
    }

    #[test]
    fn test_parsing_random1() {
        let mut scanner = Scanner::new("5 <=    1 - 2".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse();
        assert_eq!("(5 <= (1 - 2))", format!("{}", res.unwrap(),));
    }

    #[test]
    fn test_parsing_random2() {
        let mut scanner = Scanner::new("false - 2 + 3 + 4 == 2 == true <= 10".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse();
        assert_eq!(
            "(((((false - 2) + 3) + 4) == 2) == (true <= 10))",
            format!("{}", res.unwrap(),)
        );
    }

    #[test]
    fn test_string_equality() {
        let mut scanner = Scanner::new("\"epppa\" == \"epppa\"".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse();
        assert_eq!("(\"epppa\" == \"epppa\")", format!("{}", res.unwrap(),));
    }

    #[test]
    fn test_grouping_equality() {
        let mut scanner = Scanner::new("1 == (1 + 2)".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse();
        assert_eq!("(1 == (grouping (1 + 2)))", format!("{}", res.unwrap(),));
    }

    #[test]
    fn test_grouping2() {
        let mut scanner = Scanner::new("true == (false == true".to_string());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let res = parser.parse().expect_err("should've been an error");
        assert_eq!("Parser Error", res);
    }
}
