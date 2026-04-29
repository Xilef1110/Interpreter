#[path = "./token.rs"]
mod token;

pub struct Scanner {
    source: String,
    tokens: Vec<token::Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Vec<token::Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let tok = token::Token::new_token(
            token::token_type::TokenType::EOF,
            "".to_string(),
            Option::None,
            self.line,
        );
        self.tokens.push(tok);
        return self.tokens.clone();
    }

    pub fn new_scanner(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn scan_token(&self) {
        println!("scanToken")
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }
}
