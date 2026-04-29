pub mod token_type;

#[derive(Clone)]
pub struct Token {
    ttype: token_type::TokenType,
    lexeme: String,
    literal: Option<String>,
    line: i32,
}

impl Token {
    pub fn new_token(
        ttype: token_type::TokenType,
        lexeme: String,
        literal: Option<String>,
        line: i32,
    ) -> Token {
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
