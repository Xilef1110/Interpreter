struct Token<T> {
    ttype: TokenType,
    lexeme: String,
    literal: T,
    line: i32,
}

impl Token<T> {
    pub fn token<T>(ttype: TokenType, lexeme: String, literal: T, line: i32) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn toString(&self) -> String {
        format!("{self.type} {self.lexeme} {self.literal}")
    }
}
