use crate::scanner::token::Token;

struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new_parser(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }
}
