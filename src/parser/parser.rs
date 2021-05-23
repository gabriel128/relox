use crate::grammar::expr::Literal;
use crate::grammar::expr::Expr;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
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
impl<'a> Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
       Self { tokens, current: 0 }
    }

    // expression → equality ;
    fn expression() -> impl Expr {
       Parser::equality()
    }

    // equality → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality() -> impl Expr {
        let expr = Parser::comparison();

        // while let Some(TokenType::Ban)

        expr
    }

    fn comparison() -> impl Expr {
        Literal::new("ex")
    }
}
