#[path = "./token.rs"]
mod token;
use crate::Lox;
use token::Token;
use token::token_type::TokenType;

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<token::Token>,
    start: i32,
    current: i32,
    line: i32,
    lox: &'a mut Lox,
}

impl<'a> Scanner<'a> {
    pub fn scan_tokens(&mut self) -> Vec<token::Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let tok = token::Token::new_token(TokenType::EOF, "".to_string(), self.line);
        self.tokens.push(tok);
        return self.tokens.clone();
    }

    pub fn new_scanner(source: String, lox: &'a mut Lox) -> Scanner<'a> {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            lox,
        }
    }

    fn scan_token(&mut self) {
        let c: &str = self.advance();
        let mut double = false;
        match c {
            "(" => self.add_token(TokenType::LeftParen),
            ")" => self.add_token(TokenType::RightParen),
            "{" => self.add_token(TokenType::LeftBrace),
            "}" => self.add_token(TokenType::RightBrace),
            "," => self.add_token(TokenType::COMMA),
            "." => self.add_token(TokenType::DOT),
            "-" => self.add_token(TokenType::MINUS),
            "+" => self.add_token(TokenType::PLUS),
            ";" => self.add_token(TokenType::SEMICOLON),
            "*" => self.add_token(TokenType::STAR),
            "!" => self.add_token(if self.match_next("=") {
                double = true;
                TokenType::BangEqual
            } else {
                TokenType::BANG
            }),
            "=" => self.add_token(if self.match_next("=") {
                double = true;
                TokenType::EqualEqual
            } else {
                TokenType::EQUAL
            }),
            "<" => self.add_token(if self.match_next("=") {
                double = true;
                TokenType::LessEqual
            } else {
                TokenType::LESS
            }),
            ">" => self.add_token(if self.match_next("=") {
                double = true;
                TokenType::GreaterEqual
            } else {
                TokenType::GREATER
            }),
            "/" => {
                if self.match_next("/") {
                    while self.peek() != "\n" && !self.is_at_end() {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            " " => (),
            "/r" => (),
            "/t" => (),
            "/n" => self.line += 1,
            "\"" => self.string(),
            _ => self
                .lox
                .error(self.line, "Unexpected character".to_string()), // To Do
        }
        if double {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn add_token(&mut self, ttype: TokenType) {
        // Copies the entire line
        let mut text: String = self.source.clone();
        // Cast start and current fields to usize
        let start: usize = self.start as usize;
        let current: usize = self.current as usize;
        // Take the desired substring of source
        text = text[start..current].to_string();
        let tok = Token::new_token(ttype, text, self.line);
        self.tokens.push(tok);
    }

    fn advance(&mut self) -> &str {
        let start: usize = self.current as usize;
        let end: usize = start + 1;
        self.current += 1;
        &self.source[start..end]
    }

    //
    fn match_next(&self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        let start: usize = self.current as usize;
        let end: usize = start + 1;
        if &self.source[start..end] != expected {
            false
        } else {
            true
        }
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {
            "\0"
        } else {
            let start: usize = self.current as usize;
            let end: usize = start + 1;
            &self.source[start..end]
        }
    }

    fn string(&mut self) {
        // ToDO
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            self.lox.error(self.line, "Unterminated String".to_string());
        }
        // closing "
        self.advance();

        // Trim surounding quotes
        let start: usize = self.start as usize + 1;
        let end: usize = self.current as usize - 1;
        let value: String = self.source[start..end].to_string();
        self.add_token(TokenType::STRING(value));
    }
}
