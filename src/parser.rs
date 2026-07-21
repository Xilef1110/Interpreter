use crate::{
    Lox,
    expr::Expr,
    scanner::{
        TokenType::{self, FALSE},
        token::Token,
    },
};
use anyhow::{Error, Result, anyhow};

struct Parser<'a> {
    tokens: Vec<Token>,
    current: i32,
    lox: &'a mut Lox,
}

/* Parser
    As part of parsing, the Token is converted to TokenType
*/
impl<'a> Parser<'a> {
    pub fn new_parser(tokens: Vec<Token>, lox: &'a mut Lox) -> Parser<'a> {
        Parser {
            tokens,
            current: 0,
            lox,
        }
    }

    /*
     Recursive Descent Methods
    */
    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self.match_types(vec![
            TokenType::GREATER,
            TokenType::GreaterEqual,
            TokenType::LESS,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.match_types(vec![TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.match_types(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_types(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        let next = self.advance();
        match next.get_type() {
            TokenType::LeftParen => {
                // ToDo!!!
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                return Ok(Expr::Grouping(Box::new(expr)));
            }
            _ => return Ok(Expr::Literal { value: next }),
        }
    }

    // Descent helpers
    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<TokenType> {
        if self.check(ttype) {
            return Ok(self.advance().get_type());
        }
        Err(anyhow!("{}", message))
    }
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
    fn advance(&mut self) -> Token {
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
    fn previous(&self) -> Token {
        let i: usize = (self.current - 1) as usize;
        let tok = self.tokens[i].clone();
        tok
    }
}

fn report_error(tok: Token, message: &str) {
    // ToDo!!!
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
        let mut t_lox = Lox { had_error: false };
        let par = Parser::new_parser(vec![gen_token(0), gen_token(1)], &mut t_lox);
        assert!(par.check(TokenType::DOT));
        assert!(!par.check(TokenType::MINUS));
    }
}
