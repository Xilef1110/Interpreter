struct Token<T> {
    ttype: TokenType,
    lexeme: String,
    literal: T,
    line: i32,
}

impl Token<T> {
    fn token<T>(ttype: TokenType, lexeme: String, literal: T, line: i32) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}
