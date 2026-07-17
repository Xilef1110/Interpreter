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
     Recursive Descent Methods
    */

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
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
    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types {
            if (self.check(ty)) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, ttype: TokenType) -> bool {
        if (self.is_at_end()) {
            return false;
        }
        ttype == self.peek()
    }
    fn advance(&mut self) -> TokenType {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }
    fn is_at_end(&self) -> bool {
        self.peek() == TokenType::EOF
    }
    fn peek(&self) -> TokenType {
        let i: usize = self.current as usize;
        let tok = self.tokens[i].clone();
        tok.get_type()
    }
    fn previous(&self) -> TokenType {
        let i: usize = (self.current - 1) as usize;
        let tok = self.tokens[i].clone();
        tok.get_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_token(n: i32) -> Token {
        match n {
            0 => Token::new_token(TokenType::DOT, "".to_string(), 0),
            1 => Token::new_token(TokenType::PLUS, "".to_string(), 0),
            _ => Token::new_token(TokenType::LeftParen, "".to_string(), 0),
        }
    }

    #[test]
    fn test_check() {
        let par = Parser::new_parser(vec![gen_token(0), gen_token(1)]);
        assert!(par.check(TokenType::DOT));
        assert!(!par.check(TokenType::MINUS));
    }
}
