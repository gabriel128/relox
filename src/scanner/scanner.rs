use crate::token::token::Token;
use crate::token::token_type::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    fn new(source: String) -> Self {
        let tokens = Vec::new();
        Scanner { source, tokens }
    }

    fn scan_tokens(&mut self) {
        let line = 1;
        // let mut start = 0;
        // let mut current = 0;
        for source_char in self.source.chars() {
            // TODO: Fix unwrap
            let token_type = TokenType::from_char(source_char).unwrap();
            self.tokens.push(Token::new(token_type, source_char.to_string(), None, line))

        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, line))
    }


}
