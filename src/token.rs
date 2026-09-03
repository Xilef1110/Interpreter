#[path = "token/token_type.rs"]
pub mod token_type;

use std::hash::{Hash, Hasher};
use token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: i32,
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lexeme.hash(state);
        self.line.hash(state);
    }
}

impl Eq for Token {}

impl Token {
    pub fn new_token(ttype: TokenType, lexeme: String, line: i32) -> Token {
        Token {
            ttype,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let loc_type = TokenType::as_string(self.ttype.clone());
        let lexeme = self.lexeme.clone();
        let literal = self.line;
        format!("[{loc_type} lexeme:{lexeme} line:{literal}]")
    }

    pub fn get_type(&self) -> TokenType {
        self.ttype.clone()
    }
    pub fn get_line(&self) -> i32 {
        self.line
    }
    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }
}

// impl Clone for Token {
//     fn clone(&self) -> Self {

//     }
// }
