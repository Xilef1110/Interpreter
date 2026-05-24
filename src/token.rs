#[path = "token/object_type.rs"]
pub mod object_type;
#[path = "token/token_type.rs"]
pub mod token_type;

use token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new_token(ttype: TokenType, lexeme: String, line: i32) -> Token {
        Token {
            ttype,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let loc_type = self.ttype.clone();
        let lexeme = self.lexeme.clone();
        let literal = self.line;
        format!("{loc_type} {lexeme} {literal}")
    }

    pub fn get_type(&self) -> TokenType {
        self.ttype.clone()
    }
}

// impl Clone for Token {
//     fn clone(&self) -> Self {

//     }
// }
