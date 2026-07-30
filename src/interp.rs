use crate::{Token, TokenType, expr::Expr};

// Core expression evaluation
fn evaluate(ex: Expr) -> TokenType {
    match ex {
        Expr::Literal { value } => lit_expr(value),
        Expr::Grouping(inside) => group_expr(*inside),
        Expr::Unary { operator, right } => unary_expr(operator, *right),
        Expr::Binary {
            left,
            operator,
            right,
        } => binary_expr(operator, *left, *right),
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
        TokenType::MINUS => TokenType::NUMBER(-handle_number(right)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenType;
    // use crate::scanner::token::Token;

    // ── Helpers ──────────────────────────────────────────
    fn num_tok(n: f64) -> Token {
        Token::new_token(TokenType::NUMBER(n), format!("{n}"), 0)
    }
    fn str_tok(s: &str) -> Token {
        Token::new_token(TokenType::STRING(s.to_owned()), s.to_owned(), 0)
    }
    fn tok(ttype: TokenType) -> Token {
        Token::new_token(ttype, "".to_owned(), 0)
    }
    fn new_lit_expr(ttype: TokenType) -> Expr {
        Expr::Literal { value: tok(ttype) }
    }
    fn num_expr(n: f64) -> Expr {
        Expr::Literal { value: num_tok(n) }
    }
    fn str_expr(s: &str) -> Expr {
        Expr::Literal { value: str_tok(s) }
    }

    // ── evaluate ─────────────────────────────────────────
    #[test]
    fn evaluate_literal_number() {
        assert_eq!(evaluate(num_expr(42.0)), TokenType::NUMBER(42.0));
    }
    #[test]
    fn evaluate_literal_string() {
        assert_eq!(evaluate(str_expr("hi")), TokenType::STRING("hi".to_owned()));
    }
    #[test]
    fn evaluate_literal_true() {
        assert_eq!(evaluate(new_lit_expr(TokenType::TRUE)), TokenType::TRUE);
    }
    #[test]
    fn evaluate_literal_false() {
        assert_eq!(evaluate(new_lit_expr(TokenType::FALSE)), TokenType::FALSE);
    }
    #[test]
    fn evaluate_literal_nil() {
        assert_eq!(evaluate(new_lit_expr(TokenType::NIL)), TokenType::NIL);
    }
    #[test]
    fn evaluate_grouping() {
        let inner = Expr::Grouping(Box::new(num_expr(7.0)));
        assert_eq!(evaluate(inner), TokenType::NUMBER(7.0));
    }
    #[test]
    fn evaluate_nested_grouping() {
        let inner = Expr::Grouping(Box::new(Expr::Grouping(Box::new(num_expr(3.0)))));
        assert_eq!(evaluate(inner), TokenType::NUMBER(3.0));
    }

    // ── lit_expr ─────────────────────────────────────────
    #[test]
    fn lit_expr_number() {
        assert_eq!(lit_expr(num_tok(1.5)), TokenType::NUMBER(1.5));
    }
    #[test]
    fn lit_expr_string() {
        assert_eq!(lit_expr(str_tok("a")), TokenType::STRING("a".to_string()));
    }
    #[test]
    fn lit_expr_keyword() {
        assert_eq!(lit_expr(tok(TokenType::TRUE)), TokenType::TRUE);
        assert_eq!(lit_expr(tok(TokenType::NIL)), TokenType::NIL);
    }

    // ── group_expr ───────────────────────────────────────
    #[test]
    fn group_expr_passthrough() {
        assert_eq!(group_expr(num_expr(9.0)), TokenType::NUMBER(9.0));
    }
    #[test]
    fn group_expr_nested() {
        assert_eq!(
            group_expr(Expr::Grouping(Box::new(num_expr(2.0)))),
            TokenType::NUMBER(2.0)
        );
    }

    // ── is_truthy ────────────────────────────────────────
    #[test]
    fn truthy_nil_is_false() {
        assert_eq!(is_truthy(TokenType::NIL), TokenType::FALSE);
    }
    #[test]
    fn truthy_false_is_false() {
        assert_eq!(is_truthy(TokenType::FALSE), TokenType::FALSE);
    }
    #[test]
    fn truthy_true_is_true() {
        assert_eq!(is_truthy(TokenType::TRUE), TokenType::TRUE);
    }
    #[test]
    fn truthy_number_is_true() {
        assert_eq!(is_truthy(TokenType::NUMBER(0.0)), TokenType::TRUE);
        assert_eq!(is_truthy(TokenType::NUMBER(-1.0)), TokenType::TRUE);
    }
    #[test]
    fn truthy_string_is_true() {
        assert_eq!(is_truthy(TokenType::STRING("".to_owned())), TokenType::TRUE);
    }

    // ── negation ─────────────────────────────────────────
    #[test]
    fn negate_true_returns_false() {
        assert_eq!(negation(TokenType::TRUE), TokenType::FALSE);
    }
    #[test]
    fn negate_false_returns_true() {
        assert_eq!(negation(TokenType::FALSE), TokenType::TRUE);
    }
    #[test]
    #[should_panic(expected = "Not Boolean")]
    fn negate_non_bool_panics() {
        negation(TokenType::NUMBER(1.0));
    }

    // ── unary_expr ───────────────────────────────────────
    #[test]
    fn unary_minus_negates_number() {
        let op = tok(TokenType::MINUS);
        assert_eq!(unary_expr(op, num_expr(5.0)), TokenType::NUMBER(-5.0));
    }
    #[test]
    fn unary_minus_twice() {
        let op = tok(TokenType::MINUS);
        // -(-3) = 3
        let inner = Expr::Unary {
            operator: tok(TokenType::MINUS),
            right: Box::new(num_expr(3.0)),
        };
        let outer = Expr::Unary {
            operator: op,
            right: Box::new(inner),
        };
        assert_eq!(evaluate(outer), TokenType::NUMBER(3.0));
    }
    #[test]
    fn unary_bang_true() {
        let op = tok(TokenType::BANG);
        assert_eq!(
            unary_expr(op, new_lit_expr(TokenType::TRUE)),
            TokenType::FALSE
        );
    }
    #[test]
    fn unary_bang_false() {
        let op = tok(TokenType::BANG);
        assert_eq!(
            unary_expr(op, new_lit_expr(TokenType::FALSE)),
            TokenType::TRUE
        );
    }
    #[test]
    fn unary_bang_nil() {
        let op = tok(TokenType::BANG);
        assert_eq!(
            unary_expr(op, new_lit_expr(TokenType::NIL)),
            TokenType::TRUE
        );
    }
    #[test]
    fn unary_bang_number() {
        let op = tok(TokenType::BANG);
        assert_eq!(unary_expr(op, num_expr(42.0)), TokenType::FALSE);
    }
    #[test]
    #[should_panic(expected = "unary operator")]
    fn unary_unknown_operator_panics() {
        // PLUS is not a valid unary operator
        unary_expr(tok(TokenType::PLUS), num_expr(1.0));
    }
    #[test]
    #[should_panic(expected = "not a number")]
    fn unary_minus_on_non_number_panics() {
        unary_expr(tok(TokenType::MINUS), str_expr("x"));
    }

    // ── binary_expr: arithmetic ──────────────────────────
    #[test]
    fn binary_plus_numbers() {
        let op = tok(TokenType::PLUS);
        assert_eq!(
            binary_expr(op, num_expr(3.0), num_expr(4.0)),
            TokenType::NUMBER(7.0)
        );
    }
    #[test]
    fn binary_plus_strings() {
        let op = tok(TokenType::PLUS);
        assert_eq!(
            binary_expr(op, str_expr("a"), str_expr("b")),
            TokenType::STRING("ab".to_owned())
        );
    }
    #[test]
    fn binary_minus() {
        let op = tok(TokenType::MINUS);
        assert_eq!(
            binary_expr(op, num_expr(10.0), num_expr(3.0)),
            TokenType::NUMBER(7.0)
        );
    }
    #[test]
    fn binary_star() {
        let op = tok(TokenType::STAR);
        assert_eq!(
            binary_expr(op, num_expr(6.0), num_expr(7.0)),
            TokenType::NUMBER(42.0)
        );
    }
    #[test]
    fn binary_slash() {
        let op = tok(TokenType::SLASH);
        assert_eq!(
            binary_expr(op, num_expr(10.0), num_expr(2.0)),
            TokenType::NUMBER(5.0)
        );
    }
    #[test]
    fn binary_slash_by_zero() {
        let op = tok(TokenType::SLASH);
        let result = binary_expr(op, num_expr(1.0), num_expr(0.0));
        assert!(matches!(result, TokenType::NUMBER(v) if v.is_infinite()));
    }
    #[test]
    #[should_panic]
    // TODO: Fix when error handling implemented
    fn binary_plus_mixed_type_string_rhs() {
        let op = tok(TokenType::PLUS);
        assert_eq!(
            binary_expr(op, str_expr("x"), num_expr(1.0)),
            TokenType::STRING("x1".to_owned())
        );
    }

    // ── binary_expr: comparison ──────────────────────────
    #[test]
    fn binary_greater_true() {
        let op = tok(TokenType::GREATER);
        assert_eq!(
            binary_expr(op, num_expr(5.0), num_expr(3.0)),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_greater_false() {
        let op = tok(TokenType::GREATER);
        assert_eq!(
            binary_expr(op, num_expr(2.0), num_expr(2.0)),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_greater_equal_true() {
        let op = tok(TokenType::GreaterEqual);
        assert_eq!(
            binary_expr(op, num_expr(2.0), num_expr(2.0)),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_greater_equal_false() {
        let op = tok(TokenType::GreaterEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_less_true() {
        let op = tok(TokenType::LESS);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_less_false() {
        let op = tok(TokenType::LESS);
        assert_eq!(
            binary_expr(op, num_expr(3.0), num_expr(3.0)),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_less_equal_true() {
        let op = tok(TokenType::LessEqual);
        assert_eq!(
            binary_expr(op, num_expr(3.0), num_expr(3.0)),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_less_equal_false() {
        let op = tok(TokenType::LessEqual);
        assert_eq!(
            binary_expr(op, num_expr(4.0), num_expr(3.0)),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_equal_equal_true() {
        let op = tok(TokenType::EqualEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(1.0)),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_equal_equal_false() {
        let op = tok(TokenType::EqualEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_equal_equal_different_types() {
        let op = tok(TokenType::EqualEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), str_expr("1")),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_bang_equal_true() {
        let op = tok(TokenType::BangEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_bang_equal_false() {
        let op = tok(TokenType::BangEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(1.0)),
            TokenType::FALSE
        );
    }

    // TODO: Fix when error handling is implemented
    #[test]
    #[should_panic(expected = "Incorrect binary operator")]
    fn binary_unknown_operator_panics() {
        binary_expr(tok(TokenType::LeftParen), num_expr(1.0), num_expr(2.0));
    }
    #[test]
    #[should_panic(expected = "not a number")]
    fn binary_minus_on_strings_panics() {
        binary_expr(tok(TokenType::MINUS), str_expr("a"), str_expr("b"));
    }

    // ── is_equal ─────────────────────────────────────────
    #[test]
    fn is_equal_same_number() {
        assert_eq!(
            is_equal(TokenType::NUMBER(5.0), TokenType::NUMBER(5.0)),
            TokenType::TRUE
        );
    }
    #[test]
    fn is_equal_different_number() {
        assert_eq!(
            is_equal(TokenType::NUMBER(5.0), TokenType::NUMBER(6.0)),
            TokenType::FALSE
        );
    }
    #[test]
    fn is_equal_same_string() {
        assert_eq!(
            is_equal(
                TokenType::STRING("x".to_owned()),
                TokenType::STRING("x".to_owned())
            ),
            TokenType::TRUE
        );
    }
    #[test]
    fn is_equal_different_string() {
        assert_eq!(
            is_equal(
                TokenType::STRING("a".to_owned()),
                TokenType::STRING("b".to_owned())
            ),
            TokenType::FALSE
        );
    }
    #[test]
    fn is_equal_different_kind() {
        assert_eq!(
            is_equal(TokenType::TRUE, TokenType::NUMBER(1.0)),
            TokenType::FALSE
        );
        assert_eq!(is_equal(TokenType::NIL, TokenType::FALSE), TokenType::FALSE);
    }
    #[test]
    fn is_equal_bool() {
        assert_eq!(is_equal(TokenType::TRUE, TokenType::TRUE), TokenType::TRUE);
        assert_eq!(
            is_equal(TokenType::FALSE, TokenType::FALSE),
            TokenType::TRUE
        );
        assert_eq!(
            is_equal(TokenType::TRUE, TokenType::FALSE),
            TokenType::FALSE
        );
    }

    // ── handle_number ────────────────────────────────────
    #[test]
    fn handle_number_valid() {
        assert!((handle_number(TokenType::NUMBER(3.14)) - 3.14).abs() < 1e-10);
    }
    #[test]
    fn handle_number_negative() {
        assert!((handle_number(TokenType::NUMBER(-2.5)) - (-2.5)).abs() < 1e-10);
    }

    // TODO: Fix when error handling implemented
    #[test]
    #[should_panic]
    fn handle_number_on_string_panics() {
        handle_number(TokenType::STRING("oops".to_string()));
    }
    #[test]
    #[should_panic]
    fn handle_number_on_nil_panics() {
        handle_number(TokenType::NIL);
    }
}
