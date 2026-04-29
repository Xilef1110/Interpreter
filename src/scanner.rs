include!("token.rs");
include!("token_type.rs");
mod scanner {
    use crate::token;
    struct Scanner<T> {
        source: String,
        tokens: Vec<token::Token<T>>,
        start: i32,
        current: i32,
        line: i32,
    }

    impl<T> Scanner<T> {
        pub fn scan_tokens(&self) -> Vec<token::Token<T>> {
            while !self.is_at_end() {
                self.start = self.current;
                self.scanToken();
            }
            let tok = token::token(EOF, "", Option::None, self.line);
            self.tokens.push(token(EOF, "", Option::None, self.line));
            return self.tokens;
        }

        pub fn new_scanner(source: String) -> Scanner<T> {
            Scanner {
                source,
                tokens: vec![],
                start: 0,
                current: 0,
                line: 0,
            }
        }

        fn scanToken(&self) {
            println!("scanToken")
        }

        fn is_at_end(&self) -> bool {
            self.current >= self.source.len() as i32
        }
    }
}
