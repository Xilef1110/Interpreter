use crate::{
    Lox,
    expr::Expr,
    scanner::{TokenType, token::Token},
    stmt::Stmt,
};
use anyhow::{Result, anyhow};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: i32,
    lox: &'a Lox<'a>,
}

/* Parser
    As part of parsing, the Token is converted to TokenType
*/
impl<'a> Parser<'a> {
    pub fn new_parser(tokens: Vec<Token>, lox: &'a mut Lox<'a>) -> Parser<'a> {
        Parser {
            tokens,
            current: 0,
            lox,
        }
    }

    /*
     Recursive Descent Methods
    */
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];
        while !self.is_at_end() {
            // panic!("parse");
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        let statement: Result<Stmt>;
        // panic!("declaration");
        if self.match_types(vec![TokenType::VAR]) {
            statement = self.var_declaration();
        } else {
            statement = self.statement()
        }
        // Catch Errors
        match statement {
            Ok(stmt) => stmt,
            Err(_err) => {
                self.synchronize();
                return Stmt::Error;
            }
        }
    }
    fn var_declaration(&mut self) -> Result<Stmt> {
        // Ok(Stmt::Block)
        let name: Token = self.tok_consume(TokenType::IDENTIFIER, "Expect variable name.")?;
        let mut initializer = Expr::Null;
        if self.match_types(vec![TokenType::EQUAL]) {
            initializer = self.expression()?;
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.advance().get_type() {
            TokenType::PRINT => self.print_statement(),
            TokenType::LeftBrace => Ok(Stmt::Block(self.block_statement()?)),
            _ => self.expr_statement(),
        }
        // if self.match_types(vec![TokenType::PRINT]) {
        //     return self.print_statement();
        // }
        // self.expr_statement()
    }

    fn block_statement(&mut self) -> Result<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expr_statement(&mut self) -> Result<Stmt> {
        let expr: Expr = match self.expression() {
            Ok(expr) => expr,
            Err(_error) => {
                self.synchronize();
                Expr::Error
            }
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after expr.")?;
        Ok(Stmt::Expr(expr))
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let value: Expr = match self.expression() {
            Ok(expr) => expr,
            Err(_error) => {
                self.synchronize();
                Expr::Error
            }
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.equality()?;
        if self.match_types(vec![TokenType::EQUAL]) {
            let equals: Token = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(tok) = expr {
                let name: Token = tok;
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }
            if let Err(_err) = self.error(equals, "Invalid assignment target.") {
                // Since the Parser is not in a confused state, there is no need to synchronize
            }
        }
        Ok(expr)
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
        let line = next.get_line();
        match next.get_type() {
            TokenType::LeftParen => {
                // TODO!!!
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                return Ok(Expr::Grouping(Box::new(expr)));
            }
            TokenType::FALSE | TokenType::TRUE | TokenType::NIL => {
                return Ok(Expr::Literal { value: next });
            }
            TokenType::NUMBER(_n) => Ok(Expr::Literal { value: next }),
            TokenType::STRING(_str) => Ok(Expr::Literal { value: next }),
            TokenType::IDENTIFIER => Ok(Expr::Variable(self.previous())),
            _ => {
                self.lox.parse_error(next, "Expect Expression".to_string());
                Err(anyhow! {"Expect Expression: line {}", line})
            }
        }
    }

    // Descent helpers
    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<TokenType> {
        if self.check(ttype) {
            return Ok(self.advance().get_type());
        }
        self.error(self.peek_tok(), message)
    }
    fn tok_consume(&mut self, ttype: TokenType, message: &str) -> Result<Token> {
        if self.check(ttype) {
            return Ok(self.advance());
        }
        match self.error(self.peek_tok(), message) {
            Ok(_) => panic!("parser.error should always return error"),
            Err(err) => Err(err),
        }
    }
    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
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
    fn peek_tok(&self) -> Token {
        let i: usize = self.current as usize;
        let tok = self.tokens[i].clone();
        tok
    }
    fn previous(&self) -> Token {
        let i: usize = (self.current - 1) as usize;
        let tok = self.tokens[i].clone();
        tok
    }

    // Error Handling

    fn error(&mut self, tok: Token, message: &str) -> Result<TokenType> {
        self.lox.parse_error(tok, message.to_string());
        Err(anyhow!("{}", message))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().get_type() == TokenType::SEMICOLON {
                return;
            }

            if let TokenType::CLASS
            | TokenType::FUN
            | TokenType::VAR
            | TokenType::FOR
            | TokenType::IF
            | TokenType::WHILE
            | TokenType::PRINT
            | TokenType::RETURN = self.peek()
            {
                return;
            }

            self.advance();
        }
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
        let mut t_lox = Lox::new_lox();
        let par = Parser::new_parser(vec![gen_token(0), gen_token(1)], &mut t_lox);
        assert!(par.check(TokenType::DOT));
        assert!(!par.check(TokenType::MINUS));
    }
}
