// use crate::scanner::TokenType;
use crate::scanner::token::Token;
#[derive(Debug, PartialEq)]
pub enum Expr {
    Assign,
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call,
    Get,
    Grouping(Box<Expr>),
    Literal {
        value: Token,
    },
    Logical,
    Set,
    Super,
    This,
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable,
    Error,
}

pub fn print_expr(expr_p: Box<Expr>) -> String {
    match *expr_p {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left: String = print_expr(left);
            let right: String = print_expr(right);
            let op: String = operator.to_string();
            // let op: String = TokenType::as_string(operator);
            format!("({left} {op} {right})")
        }
        Expr::Grouping(expr) => print_expr(expr),
        Expr::Literal { value } => value.to_string(),
        // Expr::Literal { value } => TokenType::as_string(value),
        Expr::Unary { operator, right } => {
            let right: String = print_expr(right);
            let op: String = operator.to_string();
            // let op: String = TokenType::as_string(operator);
            format!("{op} {right}")
        }
        _ => "Error".to_string(),
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenType;

    #[test]
    fn test_print_literal() {
        let lit: Box<Expr> = Box::new(Expr::Literal {
            value: Token::new_token(TokenType::NUMBER(3 as f64), "".to_string(), 0),
        });
        assert_eq!("[Number:3 lexeme: line:0]".to_string(), print_expr(lit));
    }
    #[test]
    fn test_print_binary() {
        let six: Box<Expr> = Box::new(Expr::Literal {
            value: Token::new_token(TokenType::NUMBER(6 as f64), "".to_string(), 0),
        });
        let seven: Box<Expr> = Box::new(Expr::Literal {
            value: Token::new_token(TokenType::NUMBER(7 as f64), "".to_string(), 0),
        });
        let operator: Token = Token::new_token(TokenType::PLUS, "".to_string(), 0);
        let bin: Box<Expr> = Box::new(Expr::Binary {
            left: six,
            operator: operator,
            right: seven,
        });
        assert_eq!(
            "([Number:6 lexeme: line:0] [PLUS lexeme: line:0] [Number:7 lexeme: line:0])"
                .to_string(),
            print_expr(bin)
        );
    }
    #[test]
    fn test_print_bin_multiple() {
        let five: Box<Expr> = Box::new(Expr::Literal {
            value: Token::new_token(TokenType::NUMBER(5 as f64), "".to_string(), 0),
        });
        let six: Box<Expr> = Box::new(Expr::Literal {
            value: Token::new_token(TokenType::NUMBER(6 as f64), "".to_string(), 0),
        });
        let seven: Box<Expr> = Box::new(Expr::Literal {
            value: Token::new_token(TokenType::NUMBER(7 as f64), "".to_string(), 0),
        });
        let operator: Token = Token::new_token(TokenType::PLUS, "".to_string(), 0);
        let bin: Box<Expr> = Box::new(Expr::Binary {
            left: six,
            operator: operator,
            right: seven,
        });
        let outbin: Box<Expr> = Box::new(Expr::Binary {
            left: bin,
            operator: Token::new_token(TokenType::STAR, "".to_string(), 0),
            right: five,
        });
        assert_eq!(
            "(([Number:6 lexeme: line:0] [PLUS lexeme: line:0] [Number:7 lexeme: line:0]) [STAR lexeme: line:0] [Number:5 lexeme: line:0])"
                .to_string(),
            print_expr(outbin)
        );
    }
}
