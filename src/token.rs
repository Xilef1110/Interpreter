mod token {
    enum TokenType {
        // Single-character tokens.
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        COMMA,
        DOT,
        MINUS,
        PLUS,
        SEMICOLON,
        SLASH,
        STAR,

        // One or two character tokens.
        BANG,
        BangEqual,
        EQUAL,
        EqualEqual,
        GREATER,
        GreaterEqual,
        LESS,
        LessEqual,

        // Literals.
        IDENTIFIER,
        STRING,
        NUMBER,

        // Keywords.
        AND,
        CLASS,
        ELSE,
        FALSE,
        FUN,
        FOR,
        IF,
        NIL,
        OR,
        PRINT,
        RETURN,
        SUPER,
        THIS,
        TRUE,
        VAR,
        WHILE,

        EOF,
    }
    pub struct Token<T> {
        ttype: TokenType,
        lexeme: String,
        literal: Option<T>,
        line: i32,
    }

    impl<T> Token<T> {
        pub fn token(ttype: TokenType, lexeme: String, literal: Option<T>, line: i32) -> Token<T> {
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
}
