use crate::{
    expr::Expr,
    scanner::{TokenType, token::Token},
};

struct Parser {
    tokens: Vec<Token>,
    current: i32,
}

impl Parser {
    pub fn new_parser(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    // Recursive Descent Functions
    fn expression(&self) -> Expr {
        Expr::Assign
    }

    fn comparison(&self) -> Expr {
        Expr::Assign
    }

    fn term(&self) -> Expr {
        Expr::Assign
    }

    fn factor(&self) -> Expr {
        Expr::Assign
    }

    fn unary(&self) -> Expr {
        Expr::Assign
    }

    fn primary(&self) -> Expr {
        Expr::Assign
    }

    // Descent helpers
    fn match_types(&self, types: Vec<TokenType>) -> bool {
        false
    }
    fn check(&self, ttype: TokenType) -> bool {
        false
    }
    fn advance(&self) -> Token {
        Token::new_token(TokenType::DOT, "".to_string(), 0)
    }
    fn is_at_end(&self) -> bool {
        false
    }
    fn peek(&self) -> Token {
        Token::new_token(TokenType::DOT, "".to_string(), 0)
    }
    fn previous(&self) -> Token {
        Token::new_token(TokenType::DOT, "".to_string(), 0)
    }
}
