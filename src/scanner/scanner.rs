use crate::error_handler;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

struct Scanner {
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
        Scanner { source, tokens, source_chars, source_length }
    }

    fn scan_tokens(&mut self) {
        let line = 1;
        let mut start = 0;
        let mut current_index = 0;
        // let source_chars = &self.source_chars;

        while current_index < self.source_length {
            let source_char = &self.source_chars[current_index];

            start = current_index;
            current_index += 1;

            println!("Parsing {}", source_char);

            match TokenType::from_single_char(&source_char) {
                Some(token_type @ TokenType::SingleChar(_)) => self.add_token(token_type, start, current_index, line),
                Some(token_type @ TokenType::OneOrTwoChar(_)) => {
                    if (current_index + 1) < self.source_length {
                        let next_char = &self.source_chars[current_index + 1];
                        let (token_type, lexeme, lexeme_length) =
                            TokenType::from_two_chars(&source_char, &next_char, token_type);

                        self.add_token_with_lexeme(token_type, &lexeme, line);

                        if lexeme_length == 2 {
                            current_index += 1;
                        }

                    } else {
                        self.add_token(token_type, start, current_index, line);
                    }
                }
                Some(_token_type) => unimplemented!(),
                None => error_handler::error(line, "Unexpected character"),
            }
        }
        self.tokens.push(Token::new(TokenType::Eof, "", None, line))
    }

    fn add_token(&mut self, token_type: TokenType, start: usize, current_index: usize, line: usize) {
        self.tokens.push(Token::new(
                    token_type,
                    &self.source_chars[start..current_index]
                        .iter()
                        .collect::<String>(),
                    None,
                    line,
                ));

    }

    fn add_token_with_lexeme(&mut self, token_type: TokenType, lexeme: &str, line: usize) {
        self.tokens.push(Token::new(
                    token_type,
                    lexeme,
                    None,
                    line,
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

        let mut scanner = Scanner::new("()!".to_string());
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
            Token::new(TokenType::Eof, "", None, 1),
        ];
        assert_eq!(scanner.tokens, result);
    }

    #[test]
    fn multiple_char() {
        let mut scanner = Scanner::new("!<".to_string());
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
    }
}
