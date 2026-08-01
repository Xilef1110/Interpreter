#[path = "./keywords.rs"]
mod keywords;
#[path = "./token.rs"]
pub mod token;

use crate::Lox;
use token::Token;
pub use token::token_type::TokenType;

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

    pub fn scan_token(&mut self) {
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
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    self.lox
                        .scan_error(self.line, "Unexpected character".to_string())
                }
            }
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
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            self.lox
                .scan_error(self.line, "Unterminated String".to_string());
        }
        // closing "
        self.advance();

        // Trim surounding quotes
        let start: usize = self.start as usize + 1;
        let end: usize = self.current as usize - 1;
        let value: String = self.source[start..end].to_string();
        self.add_token(TokenType::STRING(value));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == "." && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let start: usize = self.start as usize;
        let end: usize = self.current as usize;
        let num: f64 = self.source[start..end].parse().unwrap();
        self.add_token(TokenType::NUMBER(num));
    }

    fn peek_next(&self) -> &str {
        if self.current + 1 >= self.source.len() as i32 {
            return "\\0";
        }
        let start: usize = self.current as usize + 1;
        let end: usize = start + 1;
        &self.source[start..end]
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let start: usize = self.start as usize;
        let end: usize = self.current as usize;
        let text = &self.source[start..end];
        self.add_token(keywords::get_keyword(text));
    }
}

fn is_digit(c: &str) -> bool {
    if c.is_empty() {
        return false;
    }
    c.chars().all(char::is_numeric)
}

fn is_alpha(c: &str) -> bool {
    if c.is_empty() {
        return false;
    }
    c.chars().all(char::is_alphabetic)
}

fn is_alphanumeric(c: &str) -> bool {
    if c.is_empty() {
        return false;
    }
    c.chars().all(char::is_alphanumeric)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_digit() {
        assert!(is_digit("1"));
        assert!(is_digit("10"));
        assert!(is_digit("0123456789"));
        assert!(!is_digit("a"));
        assert!(!is_digit("1b"));
        assert!(!is_digit("1z34"));
        assert!(!is_digit(""));
    }
    #[test]
    fn test_is_alpha() {
        assert!(is_alpha("l"));
        assert!(is_alpha("rr"));
        assert!(is_alpha("abcdelmnopq"));
        assert!(!is_alpha("1"));
        assert!(!is_alpha("1b"));
        assert!(!is_alpha("az3m"));
        assert!(!is_alpha(""));
    }
    #[test]
    fn test_is_alphanumeric() {
        assert!(is_alphanumeric("l"));
        assert!(is_alphanumeric("rr"));
        assert!(is_alphanumeric("1"));
        assert!(is_alphanumeric("1"));
        assert!(is_alphanumeric("1b"));
        assert!(is_alphanumeric("az3m"));
        assert!(!is_alphanumeric(""));
    }

    #[test]
    fn test_constructor() {
        let mut t_lox = Lox::new_lox();
        let scan: Scanner = Scanner::new_scanner("test".to_string(), &mut t_lox);
        assert_eq!("test".to_string(), scan.source);
        assert_eq!(0, scan.start);
        assert_eq!(0, scan.current);
        assert_eq!(false, scan.lox.had_error);
        assert_eq!(0, scan.tokens.len());
    }

    #[test]
    fn test_is_at_end() {
        let mut t_lox = Lox::new_lox();
        let mut scan: Scanner = Scanner::new_scanner("test".to_string(), &mut t_lox);
        assert!(!scan.is_at_end());
        scan.current += 3;
        assert!(!scan.is_at_end());
        scan.current += 1;
        assert!(scan.is_at_end());
    }

    #[test]
    fn test_add_token() {
        let mut t_lox = Lox::new_lox();
        let mut scan: Scanner = Scanner::new_scanner("test".to_string(), &mut t_lox);
        scan.add_token(TokenType::AND);
        scan.add_token(TokenType::BANG);
        assert_eq!(2, scan.tokens.len());
        assert!(match scan.tokens[0].get_type() {
            TokenType::AND => true,
            _ => false,
        });
        assert!(match scan.tokens[1].get_type() {
            TokenType::BANG => true,
            _ => false,
        });
    }

    #[test]
    fn test_advance_and_check_functions() {
        let mut t_lox = Lox::new_lox();
        let mut scan: Scanner = Scanner::new_scanner("test".to_string(), &mut t_lox);
        assert!(scan.match_next("t"));
        assert_eq!("t", scan.peek());
        assert_eq!("e", scan.peek_next());
        assert_eq!("t", scan.advance());
        assert_eq!("e", scan.peek());
        assert_eq!("s", scan.peek_next());
        assert!(scan.match_next("e"));
        assert_eq!("e", scan.advance());
        assert_eq!(scan.current, 2);
        assert_eq!("s", scan.peek());
        scan.advance();
        assert_eq!("\\0", scan.peek_next());
        assert_eq!(3, scan.current);
        assert_eq!("t", scan.advance());
        assert!(!scan.match_next("a"));
        assert_eq!("\0", scan.peek());
    }

    #[test]
    fn test_string() {
        let mut t_lox = Lox::new_lox();
        let mut scan: Scanner = Scanner::new_scanner("\"test\"".to_string(), &mut t_lox);
        scan.current += 1;
        scan.string();
        let _value = "test".to_string();
        assert!(match scan.tokens[0].get_type() {
            TokenType::STRING(_value) => true,
            _ => false,
        });
        assert_eq!(0, scan.start);
        assert_eq!(6, scan.current);
    }

    #[test]
    fn test_number() {
        let mut t_lox = Lox::new_lox();
        let mut scan: Scanner = Scanner::new_scanner("1234".to_string(), &mut t_lox);
        scan.current += 1;
        scan.number();
        let _value: f64 = 1234 as f64;
        assert!(match scan.tokens[0].get_type() {
            TokenType::NUMBER(_value) => true,
            _ => false,
        });
        assert_eq!(0, scan.start);
        assert_eq!(4, scan.current);
    }

    #[test]
    fn test_identifier() {
        let mut t_lox = Lox::new_lox();
        let mut scan: Scanner = Scanner::new_scanner("1234".to_string(), &mut t_lox);
        scan.identifier();
        assert!(match scan.tokens[0].get_type() {
            TokenType::IDENTIFIER => true,
            _ => false,
        });
        assert_eq!(0, scan.start);
        assert_eq!(4, scan.current);
        let mut scan: Scanner = Scanner::new_scanner("and".to_string(), &mut t_lox);
        scan.identifier();
        assert!(match scan.tokens[0].get_type() {
            TokenType::AND => true,
            _ => false,
        });
        let mut scan: Scanner = Scanner::new_scanner("return".to_string(), &mut t_lox);
        scan.identifier();
        assert!(match scan.tokens[0].get_type() {
            TokenType::RETURN => true,
            _ => false,
        });
    }
}
