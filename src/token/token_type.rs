#[derive(Clone, Debug, strum_macros::Display)]
pub enum TokenType {
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
    STRING(String),
    NUMBER(f64),

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
impl TokenType {
    pub fn as_string(tok: TokenType) -> String {
        match tok {
            TokenType::STRING(str) => {
                let nstr = str.clone();
                format!("String:{nstr}")
            }
            TokenType::NUMBER(num) => format!("Number:{num}"),
            _ => format!("{tok}"),
        }
    }
}
