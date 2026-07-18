use crate::{
    expr::Expr,
    scanner::{
        TokenType::{self, FALSE},
        token::Token,
    },
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

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_types(vec![
            TokenType::GREATER,
            TokenType::GreaterEqual,
            TokenType::LESS,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_types(vec![TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_types(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_types(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let next = self.advance();
        match next {
            TokenType::LeftParen => {
                // ToDo!!!
                panic!("Did not implement Parens");
            }
            _ => return Expr::Literal { value: next },
        }
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
    fn consume(self, ttype: TokenType, message: String) -> TokenType {
        TokenType::RightParen
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
