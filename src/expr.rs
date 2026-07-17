use crate::scanner::TokenType;
pub enum Expr {
    Assign,
    Binary {
        left: Box<Expr>,
        operator: TokenType,
        right: Box<Expr>,
    },
    Call,
    Get,
    Grouping(Box<Expr>),
    Literal {
        value: TokenType,
    },
    Logical,
    Set,
    Super,
    This,
    Unary {
        operator: TokenType,
        right: Box<Expr>,
    },
    Variable,
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
            let op: String = TokenType::as_string(operator);
            format!("({left} {op} {right})")
        }
        Expr::Grouping(expr) => print_expr(expr),
        Expr::Literal { value } => TokenType::as_string(value),
        Expr::Unary { operator, right } => {
            let right: String = print_expr(right);
            let op: String = TokenType::as_string(operator);
            format!("{op} {right}")
        }
        _ => "Error".to_string(),
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_literal() {
        let lit: Box<Expr> = Box::new(Expr::Literal {
            value: TokenType::NUMBER(3 as f64),
        });
        assert_eq!("Number:3".to_string(), print_expr(lit));
    }
    #[test]
    fn test_print_binary() {
        let six: Box<Expr> = Box::new(Expr::Literal {
            value: TokenType::NUMBER(6 as f64),
        });
        let seven: Box<Expr> = Box::new(Expr::Literal {
            value: TokenType::NUMBER(7 as f64),
        });
        let operator: TokenType = TokenType::PLUS;
        let bin: Box<Expr> = Box::new(Expr::Binary {
            left: six,
            operator: operator,
            right: seven,
        });
        assert_eq!("(Number:6 PLUS Number:7)".to_string(), print_expr(bin));
    }
    #[test]
    fn test_print_bin_multiple() {
        let six: Box<Expr> = Box::new(Expr::Literal {
            value: TokenType::NUMBER(6 as f64),
        });
        let seven: Box<Expr> = Box::new(Expr::Literal {
            value: TokenType::NUMBER(7 as f64),
        });
        let five: Box<Expr> = Box::new(Expr::Literal {
            value: TokenType::NUMBER(5 as f64),
        });
        let operator: TokenType = TokenType::PLUS;
        let bin: Box<Expr> = Box::new(Expr::Binary {
            left: six,
            operator: operator,
            right: seven,
        });
        let outbin: Box<Expr> = Box::new(Expr::Binary {
            left: bin,
            operator: TokenType::STAR,
            right: five,
        });
        assert_eq!(
            "((Number:6 PLUS Number:7) STAR Number:5)".to_string(),
            print_expr(outbin)
        );
    }
}
