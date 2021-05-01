use crate::error_handler;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

struct Scanner {
    line: usize,
    start: usize,
    current_index: usize,
    source: String,
    source_chars: Vec<char>,
    source_length: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(source: String) -> Self {
        let tokens = Vec::new();
        let source_chars: Vec<char> = source.chars().collect();
        let source_length = source_chars.len();
        let line = 1;
        let current_index = 0;
        let start = 0;
        Scanner { source, tokens, source_chars, source_length, line, current_index, start }
    }

    fn scan_tokens(&mut self) {
        while let Some(source_char) = self.source_chars.get(self.current_index) {
            let next_char: Option<&char> = self.source_chars.get(self.current_index+1);

            self.start = self.current_index;

            // println!("Parsing {}, current_index {}", source_char, current_index);
            match TokenType::from_single_char(&source_char) {
                Some(token_type @ TokenType::SingleChar(_)) => self.add_token(token_type),
                Some(token_type @ TokenType::OneOrTwoChar(_)) => {
                    if let Some(next_char) = next_char {
                        let (token_type, lexeme, lexeme_length) =
                            TokenType::from_two_chars(&source_char, &next_char, token_type);

                        self.add_token_with_lexeme(token_type, &lexeme);

                        if lexeme_length == 2 {
                            self.advance();
                        }

                    } else {
                        self.add_token(token_type);
                    }
                }
                Some(token_type @ TokenType::SlashOrComment(_)) => {
                    if let Some(next_char) = next_char {
                        if TokenType::is_comment(*source_char, *next_char) {
                            eprintln!("Got here");
                            while self.current_index < self.source_length && (self.source_chars[self.current_index]) != '\n' {
                               self.advance();
                            }
                        }
                    } else {
                        self.add_token(token_type);
                    }
                }
                Some(_token_type) => unimplemented!(),
                None => error_handler::error(self.line, "Unexpected character"),
            }

            self.advance();
        }
        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line))
    }

    fn advance(&mut self) {
        self.current_index += 1;
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
                    token_type,
                    &self.source_chars[self.start..self.current_index+1]
                        .iter()
                        .collect::<String>(),
                    None,
                    self.line,
                ));

    }

    fn add_token_with_lexeme(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens.push(Token::new(
                    token_type,
                    lexeme,
                    None,
                    self.line,
                ));

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token_type::*;

    #[test]
    fn single_chars() {
        let mut scanner = Scanner::new("(".to_string());
        scanner.scan_tokens();
        let result = vec![
            Token::new(
                TokenType::SingleChar(SingleCharTokens::LeftParen),
                "(",
                None,
                1,
            ),
            Token::new(TokenType::Eof, "", None, 1),
        ];
        assert_eq!(scanner.tokens, result);

        let mut scanner = Scanner::new("()!/".to_string());
        scanner.scan_tokens();
        let result = vec![
            Token::new(
                TokenType::SingleChar(SingleCharTokens::LeftParen),
                "(",
                None,
                1,
            ),
            Token::new(
                TokenType::SingleChar(SingleCharTokens::RightParen),
                ")",
                None,
                1,
            ),
            Token::new(
                TokenType::OneOrTwoChar(OneOrTwoCharTokens::Bang),
                "!",
                None,
                1,
            ),
            Token::new(
                TokenType::SlashOrComment(SlashOrComment::Slash),
                "/",
                None,
                1,
            ),
            Token::new(TokenType::Eof, "", None, 1),
        ];
        assert_eq!(scanner.tokens, result);
    }

    #[test]
    fn multiple_char() {
        let mut scanner = Scanner::new("!<// blah blah blah".to_string());
        scanner.scan_tokens();
        let result = vec![
            Token::new(
                TokenType::OneOrTwoChar(OneOrTwoCharTokens::Bang),
                "!",
                None,
                1,
            ),
            Token::new(
                TokenType::OneOrTwoChar(OneOrTwoCharTokens::Less),
                "<",
                None,
                1,
            ),
            Token::new(TokenType::Eof, "", None, 1),
        ];
        assert_eq!(scanner.tokens, result);

        let mut scanner = Scanner::new("<= // blah \n !".to_string());
        scanner.scan_tokens();
        let result = vec![
            Token::new(
                TokenType::OneOrTwoChar(OneOrTwoCharTokens::LessEqual),
                "<=",
                None,
                1,
            ),
            Token::new(
                TokenType::OneOrTwoChar(OneOrTwoCharTokens::Bang),
                "!",
                None,
                1,
            ),
            Token::new(TokenType::Eof, "", None, 1),
        ];
        assert_eq!(scanner.tokens, result);
    }
}
