use super::chunk::{Chunk, OpCode};
use super::value::Value;
use crate::errors::ErrorKind::ParserError;
use crate::errors::{ErrorKind::Fatal, ReloxError};
use crate::token::Literal;
use crate::token::Token;
use crate::token::token_type::TokenType;
use crate::Result;

#[derive(Debug)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
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
            return ReloxError::new_compile_error(
                0,
                "Error on compilation".to_string(),
                None,
                ParserError,
            );
        }

        self.parse()?;

        self.emit_return()?;
        Ok(self.chunk)
    }

    pub fn parse(&mut self) -> Result<()> {
        self.expression()?;
        let res = self.consume(TokenType::Eof, "Expects an expression");

        if self.had_error {
            let current_token = self.current_token()?;
            ReloxError::new_compile_error(
                current_token.line,
                "Compilation Error".to_string(),
                None,
                ParserError,
            )
        } else {
            res
        }
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

    fn binary(&mut self, token_type: TokenType) -> Result<()> {
        let higher_precedence = Precedence::new(self.current_token_type()?).to_number() + 1;
        self.parse_with_precendece(higher_precedence)?;

        match token_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Substract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            _ => Ok(()),
        }
    }

    fn parse_with_precendece(&mut self, precedence: u8) -> Result<()> {
        self.advance()?;

        // dbg!(self.prev_token()?, self.current_token()?, self.cursor);

        self.parse_prefix_for_type(self.prev_token_type()?)?;

        while precedence <= Precedence::new(self.current_token_type()?).to_number() {
            self.advance()?;
            self.parse_infix_for_type(self.prev_token_type()?)?
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_with_precendece(Precedence::Assignment.to_number())
    }

    fn number(&mut self) -> Result<()> {
        self.emit_constant()
    }

    fn literal(&mut self, token_type: TokenType) -> Result<()> {
        match token_type {
            TokenType::Nil => self.emit_byte(OpCode::Nil),
            TokenType::True => self.emit_byte(OpCode::True),
            TokenType::False => self.emit_byte(OpCode::False),
            _ => Ok(()),
        }
    }

    fn grouping(&mut self) -> Result<()> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression")
    }

    fn unary(&mut self, token_type: TokenType) -> Result<()> {
        self.parse_with_precendece(Precedence::Unary.to_number())?;

        match token_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => Ok(()),
        }
    }

    fn parse_prefix_for_type(&mut self, token_type: TokenType) -> Result<()> {
        match token_type {
            TokenType::LeftParen => self.grouping(),
            TokenType::Minus => self.unary(token_type),
            TokenType::Number => self.number(),
            TokenType::Nil => self.literal(token_type),
            TokenType::True => self.literal(token_type),
            TokenType::False => self.literal(token_type),
            unreq_token_type => ReloxError::new_fatal_error(format!(
                "Prefix unimplemented for {:?}",
                unreq_token_type
            )),
        }
    }

    fn parse_infix_for_type(&mut self, token_type: TokenType) -> Result<()> {
        match token_type {
            TokenType::Slash => self.binary(token_type),
            TokenType::Minus => self.binary(token_type),
            TokenType::Plus => self.binary(token_type),
            TokenType::Star => self.binary(token_type),
            unreq_token_type => ReloxError::new_fatal_error(format!(
                "Infix unimplemented for {:?}",
                unreq_token_type
            )),
        }
    }

    // == ByteCode Handling ==
    fn emit_constant(&mut self) -> Result<()> {
        let prev_token = self.prev_token()?;
        match prev_token.literal {
            Some(Literal::Double(value)) => {
                let token_line = prev_token.line as u16;
                self.chunk.add_constant(Value::Number(value), token_line)?;
                Ok(())
            }
            _ => ReloxError::new_compile_error(
                prev_token.line,
                "Error on compilation".to_string(),
                None,
                Fatal,
            ),
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
        if let Some(token) = self.tokens.get(self.cursor) {
            Ok(token)
        } else {
            ReloxError::new_fatal_error("Parser tried to fetch an unexistent token".to_string())
        }
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
        if let Some(token) = self.tokens.get(cursor - 1) {
            Ok(token)
        } else {
            ReloxError::new_fatal_error("Parser tried to fetch an unexistent token".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::vm::Vm;
    use crate::scanner::Scanner;

    use super::*;

    #[test]
    fn test_simple_addition() {
        let tokens = Scanner::run_with("1 + 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        // dbg!(&chunk);
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Number(3.0));
    }

    #[test]
    fn test_simple_substraction() {
        let tokens = Scanner::run_with("3 - 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Number(1.0));
    }

    #[test]
    fn test_addition_with_mult() {
        let tokens = Scanner::run_with("1 + 2 * 3".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Number(7.0));
    }

    #[test]
    fn test_addition_with_mult2() {
        let tokens = Scanner::run_with("1 * 3 + 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Number(5.0));
    }

    #[test]
    fn test_parens1() {
        let tokens = Scanner::run_with("(1 + 3) * 2".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Number(8.0));
    }

    #[test]
    fn test_parens2() {
        let tokens = Scanner::run_with("(1 + (3 - 1)) * (2 + 2)".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Number(12.0));
    }

    #[test]
    fn test_booleans() {
        let tokens = Scanner::run_with("true".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Bool(true));
    }

    #[test]
    fn test_boolean_grouping() {
        let tokens = Scanner::run_with("(true)".to_string()).unwrap();
        let chunk = Compiler::run_with(tokens).unwrap();
        let val = Vm::run_with(chunk, false).unwrap();
        assert_eq!(val, Value::Bool(true));
    }

    #[test]
    #[should_panic]
    fn test_syntax_errors() {
        let tokens = Scanner::run_with("##$".to_string()).unwrap();
        Compiler::run_with(tokens).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_syntax_errors2() {
        let tokens = Scanner::run_with("((true)".to_string()).unwrap();
        Compiler::run_with(tokens).unwrap();
    }
}
