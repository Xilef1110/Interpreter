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
    Grouping,
    Literal {
        value: TokenType,
    },
    Logical,
    Set,
    Super,
    This,
    Unary,
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
            format!("{left} {op} {right}")
        }
        Expr::Literal { value } => TokenType::as_string(value),
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
        assert_eq!("Number: 3".to_string(), print_expr(lit));
    }
}
