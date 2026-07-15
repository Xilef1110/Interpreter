use token_type::TokenType;
enum Expr {
    Assign,
    Binary {
        left: Expr,
        operator: TokenType,
        right: Expr,
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

pub fn printExpr(expr: Expr) -> String {
    match expr {
        Binary => {
            format!("{printExpr(left)} {value} {printExpr(right)}")
        }
        Literal => {
            format!({ value })
        }
        _ => {}
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testPrintLiteral() {
        let lit: Expr = Literal(TokenType::NUMBER(3));
        assert_eq!("3".to_string, printExpr(lit));
    }
}
