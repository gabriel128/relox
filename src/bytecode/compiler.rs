use super::chunk::{Chunk, OpCode};
use crate::errors::ErrorKind::ParserError;
use crate::errors::{ErrorKind::Fatal, ReloxError};
use crate::token::token::Literal;
use crate::token::{token::Token, token_type::TokenType};
use crate::Result;

#[derive(Debug)]
enum Precedence {
    None,
    Assignment,  // =
    Or,          // or
    And,         // and
    Equality,    // == !=
    Comparison,  // < > <= >=
    Term,        // + -
    Factor,      // * /
    Unary,       // ! -
    Call,        // . ()
    Primary
}

impl Precedence {
    pub fn new(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Slash => Precedence::Factor,
            TokenType::Star => Precedence::Factor,
            TokenType::Minus => Precedence::Term,
            TokenType::Plus => Precedence::Term,
            _ => Precedence::None,
        }
    }

    pub fn to_number(&self) -> u8 {
        match self {
            Precedence::None => 1,
            Precedence::Assignment => 2,
            Precedence::Or => 3,
            Precedence::And => 4,
            Precedence::Equality => 5,
            Precedence::Comparison => 6,
            Precedence::Term => 7,
            Precedence::Factor => 8,
            Precedence::Unary => 9,
            Precedence::Call => 10,
            Precedence::Primary => 11,
        }
    }

}

#[derive(Debug)]
pub struct Compiler {
    chunk: Chunk,
    tokens: Vec<Token>,
    cursor: usize,
    had_error: bool,
    panic_mode: bool,
}

impl Compiler {
    pub fn run_with(tokens: Vec<Token>) -> Result<Chunk> {
        Self::new(tokens).compile()
    }
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            chunk: Chunk::new(),
            tokens,
            cursor: 0,
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(mut self) -> Result<Chunk> {
        if self.had_error {
            return Err(ReloxError::new_compile_error(
                0,
                "Error on compilation".to_string(),
                None,
                ParserError,
            ));
        }

        self.parse()?;

        self.emit_return()?;
        Ok(self.chunk)
    }

    pub fn parse(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::Eof, "Expect and expression")
    }

    fn advance(&mut self) -> Result<()> {
        loop {
            let token = self.current_token()?;

            if token.token_type == TokenType::ErrorToken {
                let token_cl = token.clone();
                self.handle_error(token_cl, "");
            } else {
                break;
            }
        }

        self.cursor += 1;
        Ok(())
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<()> {
        let token = self.current_token()?;

        if token.token_type == token_type {
            self.advance()?;
        } else {
            let token = token.clone();
            self.handle_error(token, message)
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_with_precendece(Precedence::Assignment.to_number())?;
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        self.emit_constant()
    }

    fn grouping(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression")
    }

    fn unary(&mut self) -> Result<()> {
        let original_cursor = self.cursor;

        self.parse_with_precendece(Precedence::Unary.to_number())?;

        let prev_token = self.prev_token_for(original_cursor)?;
        match prev_token.token_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => Ok(())
        }
    }

    fn binary(&mut self) -> Result<()> {
        let original_cursor = self.cursor;


        let higher_precedence = Precedence::new(self.current_token_type()?).to_number() + 1;
        self.parse_with_precendece(higher_precedence)?;

        let prev_token = self.prev_token_for(original_cursor)?;

        match prev_token.token_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Substract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            _ => Ok(())
        }
    }

    fn parse_with_precendece(&mut self, precedence: u8) -> Result<()> {
        self.advance()?;
        // dbg!(&self.tokens, self.current_token(), self.prev_token(), self.cursor);
        self.parse_prefix_for_type(self.prev_token_type()?)?;

        while precedence <= Precedence::new(self.current_token_type()?).to_number() {
            self.advance()?;
            self.parse_infix_for_type(self.prev_token_type()?)?
        }
        Ok(())
    }

    fn parse_prefix_for_type(&mut self, token_type: TokenType) -> Result<()> {
        match token_type {
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus => self.unary(),
            TokenType::Number => self.number(),
            unreq_token_type => Err(ReloxError::new_fatal_error(format!("Prefix unimplemented for {:?}", unreq_token_type)))
        }
    }

    fn parse_infix_for_type(&mut self, token_type: TokenType) -> Result<()> {
        match token_type {
            TokenType::Slash => self.binary(),
            TokenType::Minus => self.binary(),
            TokenType::Plus => self.binary(),
            TokenType::Star => self.binary(),
            unreq_token_type => Err(ReloxError::new_fatal_error(format!("Infix unimplemented for {:?}", unreq_token_type)))
        }
    }

    // == ByteCode Handling ==
    fn emit_constant(&mut self) -> Result<()> {
        let prev_token = self.prev_token()?;
        match prev_token.literal {
            Some(Literal::Double(value)) => {
                let token_line = prev_token.line as u16;
                self.chunk.add_constant(value, token_line)?;
                Ok(())
            }
            _ => Err(ReloxError::new_compile_error(
                prev_token.line,
                "Error on compilation".to_string(),
                None,
                Fatal,
            )),
        }
    }

    fn emit_byte(&mut self, bytecode: OpCode) -> Result<()> {
        let prev_token = self.prev_token()?;
        let token_line = prev_token.line as u16;
        self.chunk.write_bytecode(bytecode, token_line);
        Ok(())
    }

    fn emit_return(&mut self) -> Result<()> {
        self.emit_byte(OpCode::Return)?;
        Ok(())
    }

    // ==  Utility Functions ==
    fn handle_error(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            self.cursor += 1;
            return;
        }
        self.panic_mode = true;

        match token.token_type {
            TokenType::Eof => println!("[line {}] Error at end: {}", token.line, message),
            _ => println!(
                "[line {}] Error at {}: {}",
                token.line, token.lexeme, message
            ),
        }
        self.had_error = true;
    }

    fn current_token(&self) -> Result<&Token> {
        self.tokens
            .get(self.cursor)
            .ok_or(ReloxError::new_fatal_error(
                "Parser tried to fetch an unexistent token".to_string(),
            ))
    }

    fn current_token_type(&self) -> Result<TokenType> {
        Ok(self.current_token()?.token_type)
    }

    fn prev_token(&self) -> Result<&Token> {
        self.prev_token_for(self.cursor)
    }

    fn prev_token_type(&self) -> Result<TokenType> {
        Ok(self.prev_token()?.token_type)
    }

    fn prev_token_for(&self, cursor: usize) -> Result<&Token> {
        self.tokens
            .get(cursor - 1)
            .ok_or(ReloxError::new_fatal_error(
                "Parser tried to fetch an unexistent token".to_string(),
            ))
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::scanner::Scanner;
    use crate::bytecode::vm::Vm;

    use super::*;

    #[test]
    fn test_simple_addition() {
        let tokens = Scanner::run_with("1 + 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, 3.0);
    }

    #[test]
    fn test_simple_substraction() {
        let tokens = Scanner::run_with("3 - 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, 1.0);
    }

    #[test]
    fn test_addition_with_mult() {
        let tokens = Scanner::run_with("1 + 2 * 3".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, 7.0);
    }

    #[test]
    fn test_addition_with_mult2() {
        let tokens = Scanner::run_with("1 * 3 + 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, 5.0);
    }

    #[test]
    fn test_parens1() {
        let tokens = Scanner::run_with("(1 + 3) * 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, 8.0);
    }

    #[test]
    fn test_parens2() {
        let tokens = Scanner::run_with("(1 + (3 - 1)) * (2 + 2)".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, 12.0);
    }

    #[test]
    #[should_panic]
    fn test_syntax_error() {
        let tokens = Scanner::run_with("##$".to_string()).unwrap();
        Compiler::run_with(tokens).unwrap();
    }
}
