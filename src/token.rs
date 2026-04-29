#[path = "token/object_type.rs"]
pub mod object_type;
#[path = "token/token_type.rs"]
pub mod token_type;

use object_type::ObjectType;
use token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: ObjectType,
    line: i32,
}

impl Token {
    pub fn new_token(ttype: TokenType, lexeme: String, literal: ObjectType, line: i32) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        // let loc_type = self.ttype;
        // let lexeme = self.lexeme;
        // let literal = self.literal;
        // format!("{loc_type} {lexeme} {literal}")
        return "".to_string();
    }
}

// impl Clone for Token {
//     fn clone(&self) -> Self {

//     }
// }
