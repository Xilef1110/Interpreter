#[path = "token/token_type.rs"]
pub mod token_type;

use token_type::TokenType;

#[derive(Clone, PartialEq, Debug)]
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
