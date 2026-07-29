use crate::{Token, TokenType, expr::Expr};

// Core expression evaluation
fn evaluate(ex: Expr) -> TokenType {
    match ex {
        Expr::Literal { value } => lit_expr(value),
        Expr::Grouping(inside) => group_expr(*inside),
        _ => TokenType::NIL,
    }
}

// Helpers for each expression type
fn lit_expr(lit: Token) -> TokenType {
    lit.get_type()
}

fn group_expr(inside: Expr) -> TokenType {
    evaluate(inside)
}

fn unary_expr(operator: Token, ex: Expr) -> TokenType {
    let right = evaluate(ex);
    match operator.get_type() {
        TokenType::MINUS => TokenType::NUMBER(handle_number(right)),
        TokenType::BANG => negation(is_truthy(right)),
        // TODO add error handling, BANG operator
        _ => panic!("unary operator"),
    }
}

fn binary_expr(operator: Token, l: Expr, r: Expr) -> TokenType {
    let left: TokenType = evaluate(l);
    let right: TokenType = evaluate(r);
    match operator.get_type() {
        TokenType::GREATER => greater(handle_number(left), handle_number(right)),
        TokenType::GreaterEqual => greater_equal(handle_number(left), handle_number(right)),
        TokenType::LESS => less(handle_number(left), handle_number(right)),
        TokenType::LessEqual => less_equal(handle_number(left), handle_number(right)),
        TokenType::EqualEqual => is_equal(left, right),
        TokenType::BangEqual => negation(is_equal(left, right)),
        TokenType::MINUS => TokenType::NUMBER(handle_number(left) - handle_number(right)),
        TokenType::SLASH => TokenType::NUMBER(handle_number(left) / handle_number(right)),
        TokenType::STAR => TokenType::NUMBER(handle_number(left) * handle_number(right)),
        TokenType::PLUS => {
            if let TokenType::STRING(string) = left {
                TokenType::STRING(format!("{}{}", string, handle_string(right)))
            } else {
                TokenType::NUMBER(handle_number(left) + handle_number(right))
            }
        }
        _ => panic!("Incorrect binary operator"),
    }
}

// Other Helpers
fn is_truthy(ttype: TokenType) -> TokenType {
    match ttype {
        TokenType::NIL => TokenType::FALSE,
        TokenType::FALSE => TokenType::FALSE,
        _ => TokenType::TRUE,
    }
}
fn negation(ttype: TokenType) -> TokenType {
    if let TokenType::TRUE = ttype {
        TokenType::FALSE
    } else if let TokenType::FALSE = ttype {
        TokenType::TRUE
    } else {
        panic!("Not Boolean");
    }
}
fn greater(l: f64, r: f64) -> TokenType {
    if l > r {
        TokenType::TRUE
    } else {
        TokenType::FALSE
    }
}
fn greater_equal(l: f64, r: f64) -> TokenType {
    if l >= r {
        TokenType::TRUE
    } else {
        TokenType::FALSE
    }
}
fn less(l: f64, r: f64) -> TokenType {
    if l < r {
        TokenType::TRUE
    } else {
        TokenType::FALSE
    }
}
fn less_equal(l: f64, r: f64) -> TokenType {
    if l <= r {
        TokenType::TRUE
    } else {
        TokenType::FALSE
    }
}
fn is_equal(left: TokenType, right: TokenType) -> TokenType {
    if left == right {
        TokenType::TRUE
    } else {
        TokenType::FALSE
    }
}

fn handle_number(ttype: TokenType) -> f64 {
    if let TokenType::NUMBER(num) = ttype {
        num
    } else {
        panic!("not a number");
        // TODO add proper error handling
    }
}
fn handle_string(ttype: TokenType) -> String {
    if let TokenType::STRING(str) = ttype {
        str
    } else {
        panic!("not a number");
        // TODO add proper error handling
    }
}
