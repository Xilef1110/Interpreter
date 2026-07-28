use crate::{Token, TokenType, expr::Expr};

// Core expression evaluation
fn evaluate(ex: Expr) -> TokenType {
    match ex {
        Expr::Literal { value } => lit_expr(value),
        _ => TokenType::NIL,
    }
}

// Helpers for each expression type
fn lit_expr(lit: Token) -> TokenType {
    TokenType::NIL
}

// fn groupExpr()
