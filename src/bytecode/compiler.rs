use crate::errors::ErrorKind::ParserError;
use crate::errors::{ErrorKind::Fatal, ReloxError};
use crate::token::token::Literal;
use crate::token::{token::Token, token_type::TokenType};
use crate::Result;
use super::chunk::{Chunk, OpCode};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0, had_error: false, panic_mode: false }
    }

    pub fn parse(&mut self) -> Result<()> {
        self.advance()?;
        self.expression()?;
        self.consume(TokenType::Eof, "Expect and expression")
    }

    fn expression(&mut self) -> Result<()> {
        Ok(())
    }

    // TODO make it less procedural
    fn advance(&mut self) -> Result<()> {
        loop {
            let token = self.current_token()?;

            if token.token_type == TokenType::ErrorToken  {
                let token_cl = token.clone();
                self.handle_error(token_cl, "");
            } else {
                break;
            }
            self.cursor += 1;
        }
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

    fn handle_error(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return
        }
        self.panic_mode = true;

        match token.token_type {
            TokenType::Eof => println!("[line {}] Error at end: {}", token.line, message),
            _ => println!("[line {}] Error at {}: {}", token.line, token.lexeme, message),
        }
        self.had_error = true;
    }

    fn current_token(&self) -> Result<&Token> {
        self.tokens.get(self.cursor).ok_or(
            ReloxError::new_fatal_error("Parser tried to fetch an unexistent token".to_string())
        )
    }

    fn prev_token(&self) -> Result<&Token> {
        self.tokens.get(self.cursor - 1).ok_or(
            ReloxError::new_fatal_error("Parser tried to fetch an unexistent token".to_string())
        )
    }
}

#[derive(Debug)]
struct Compiler {
    parser: Parser,
    chunk: Chunk,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            chunk: Chunk::new(),
            parser: Parser::new(tokens)
        }
    }

    pub fn compile(&mut self) -> Result<()> {
        if self.parser.had_error {
            return Err(
                ReloxError::new_compile_error(0, "Error on compilation".to_string(), None, ParserError)
            )
        }
        self.parser.parse()?;

        self.emit_return()?;
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        self.emit_constant()
    }

    fn emit_constant(&mut self) -> Result<()> {
        let prev_token = self.parser.prev_token()?;
        match prev_token.literal {
            Some(Literal::Double(value)) => {
                self.chunk.add_constant(value, prev_token.line as u16)?;
                Ok(())
            },
            _ => {
                Err(
                    ReloxError::new_compile_error(prev_token.line, "Error on compilation".to_string(), None, Fatal)
                )
            }
        }
    }

    fn emit_byte(&mut self, bytecode: OpCode) -> Result<()> {
        let prev_token = self.parser.prev_token()?;
        self.chunk.write_bytecode(bytecode, prev_token.line as u16);
        Ok(())
    }

    // fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) -> Result<()> {
    //     self.emit_byte(byte1)?;
    //     self.emit_byte(byte2)?;
    //     Ok(())
    // }

    fn emit_return(&mut self) -> Result<()> {
        self.emit_byte(OpCode::Return)?;
        Ok(())
    }

}
