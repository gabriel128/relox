use crate::errors::ErrorKind::LexError;
use crate::errors::ReloxError;
use crate::token::token::{Literal, Token};
use crate::token::token_type::TokenKind;
use crate::token::token_type::TokenType;

#[derive(Debug)]
pub struct Scanner {
    line: usize,
    start: usize,
    current_index: usize,
    source_chars: Vec<char>,
    source_length: usize,
    tokens: Vec<Token>,
    errors: Vec<ReloxError>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens = Vec::new();
        let source_chars: Vec<char> = source.chars().collect();
        let source_length = source_chars.len();
        let line = 1;
        let current_index = 0;
        let start = 0;
        Scanner {
            tokens,
            source_chars,
            source_length,
            line,
            current_index,
            start,
            errors: Vec::new(),
        }
    }

    /// Returns a tuple of Tokens and Errors. It uses vector of errors instead of interrupting
    /// to provide a better experience to (hypotehtical) users by showing all the lexical errors at once.
    pub fn scan_tokens(&mut self) -> (&Vec<Token>, &Vec<ReloxError>) {
        self.run_scan();
        (&self.tokens, &self.errors)
    }

    fn run_scan(&mut self) {
        while let Some(ref source_char) = self.source_chars.get(self.current_index) {
            self.start = self.current_index;

            // println!("Parsing {}, current_index {}", source_char, self.current_index);
            match TokenType::from_single_char(source_char) {
                Some((token_type, TokenKind::SingleChar)) => self.add_token(token_type),
                Some((token_type, TokenKind::OneOrTwoChar)) => self.handle_one_or_two(token_type),
                Some((token_type, TokenKind::SlashOrComment)) => {
                    self.handle_slash_or_comment(token_type)
                }
                Some((TokenType::String, _)) => self.handle_string(),
                Some((TokenType::Skip, _)) => {}
                Some((TokenType::NewLine, _)) => self.line += 1,
                Some((TokenType::Number, _)) => self.handle_number(),
                Some(token_type) => {
                    self.errors.push(ReloxError::new_compile_error(
                        self.line,
                        format!("Unexpected token {:?}", token_type),
                        None,
                        LexError,
                    ));
                }
                None => {
                    if source_char.is_ascii_alphabetic() {
                        self.handle_keyword_or_identifier();
                    } else {
                        self.errors.push(ReloxError::new_compile_error(
                            self.line,
                            format!("Unexpected Token {}", source_char),
                            None,
                            LexError,
                        ));
                    }
                }
            }

            self.advance();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line))
    }

    fn handle_slash_or_comment(&mut self, token_type: TokenType) {
        let current_char = self.current_char();
        let next_char: Option<&char> = self.source_chars.get(self.current_index + 1);

        if let (Some(next_char), Some(current_char)) = (next_char, current_char) {
            if TokenType::is_comment(*current_char, *next_char) {
                self.handle_comment();
            } else {
                self.add_token(token_type);
            }
        } else {
            self.add_token(token_type);
        }
    }

    fn handle_one_or_two(&mut self, token_type: TokenType) {
        let current_char = self.current_char();
        let next_char: Option<&char> = self.source_chars.get(self.current_index + 1);
        if let (Some(next_char), Some(current_char)) = (next_char, current_char) {
            let (token_type, lexeme, lexeme_length) =
                TokenType::from_two_chars(current_char, &next_char, token_type);

            self.add_token_with_lexeme(token_type, &lexeme);

            if lexeme_length == 2 {
                self.advance();
            }
        } else {
            self.add_token(token_type);
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.source_length
    }

    fn current_char(&self) -> Option<&char> {
        self.source_chars.get(self.current_index)
    }

    fn next_char(&self) -> Option<&char> {
        self.source_chars.get(self.current_index + 1)
    }

    fn handle_keyword_or_identifier(&mut self) {
        while let Some(ref current_char) = self.current_char() {
            if current_char.is_ascii_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        let lexeme = self.substring_source(self.start, self.current_index);

        if let Some(keyword_type) = TokenType::keyword(&lexeme) {
            self.add_token_with_lexeme(keyword_type, &lexeme);
        } else {
            self.add_token_with_lexeme(TokenType::Identifier, &lexeme);
        }
    }

    fn handle_comment(&mut self) {
        while let Some(current_char) = self.current_char() {
            if *current_char == '\n' {
                self.line += 1;
                break;
            } else {
                self.advance();
            }
        }
    }

    fn handle_string(&mut self) {
        self.advance();
        while let Some(current_char) = self.current_char() {
            if *current_char == '\n' {
                self.line += 1;
                self.advance();
            } else if *current_char != '"' {
                self.advance();
            } else {
                break;
            }
        }

        if self.is_at_end() {
            self.errors.push(ReloxError::new_compile_error(
                self.line,
                "String not terminated".to_string(),
                None,
                LexError,
            ));
            return;
        }

        let the_string = &self.substring_source(self.start + 1, self.current_index);

        self.tokens.push(Token::new(
            TokenType::String,
            the_string,
            Some(Literal::String(the_string.to_string())),
            self.line,
        ));
    }

    fn handle_number(&mut self) {
        let mut got_a_dot = false;

        while let Some(next_char) = self.next_char() {
            if next_char.is_digit(10) {
                self.advance();
            } else if *next_char == '.' && !got_a_dot {
                got_a_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        // if the last char is a dot it means that it's  a number like 11.
        // so we retreat so the dot gets parsed as its own
        if let Some('.') = self.current_char() {
            self.retreat();
        }

        let numstr = &self.substring_source(self.start, self.current_index + 1);
        let num: Result<f64, _> = numstr.parse();

        self.tokens.push(Token::new(
            TokenType::Number,
            numstr,
            num.ok().map(|the_num| Literal::Double(the_num)),
            self.line,
        ));
    }

    fn substring_source(&self, start: usize, end: usize) -> String {
        if end >= self.source_chars.len() {
            self.source_chars[start..].iter().collect::<String>()
        } else {
            self.source_chars[start..end].iter().collect::<String>()
        }
    }

    fn advance(&mut self) {
        self.current_index += 1;
    }

    fn retreat(&mut self) {
        self.current_index -= 1;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            &self.substring_source(self.start, self.current_index + 1),
            None,
            self.line,
        ));
    }

    fn add_token_with_lexeme(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
    }
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
