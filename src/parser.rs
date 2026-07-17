use crate::{
    expr::Expr,
    scanner::{TokenType, token::Token},
};

struct Parser {
    tokens: Vec<Token>,
    current: i32,
}
/* Parser
    As part of parsing, the Token is converted to TokenType
*/

impl Parser {
    pub fn new_parser(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    /*
     Recursive Descent Functions
    */

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let mut expr = self.comparison();
        while (self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual])) {
            let operator: TokenType = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        expr
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
    fn peek(&self) -> TokenType {
        TokenType::DOT
    }
    fn previous(&self) -> TokenType {
        TokenType::DOT
    }
}
