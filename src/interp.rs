use crate::{Lox, Token, TokenType, environment::Environment, expr::Expr, stmt::Stmt};
use anyhow::{Result, anyhow};

// Core expression evaluation
pub fn interpret(statements: Vec<Stmt>, lox: &mut Lox) {
    let mut env = Environment::new_environment();
    for stmt in statements {
        match execute(&mut env, stmt) {
            Ok(ttype) => {} //TODO
            Err(err) => {
                lox.runtime_error(err.to_string());
            }
        }
    }
}
fn evaluate(env: &Environment, ex: Expr) -> Result<TokenType> {
    match ex {
        Expr::Literal { value } => Ok(lit_expr(value)),
        Expr::Grouping(inside) => group_expr(env, *inside),
        Expr::Unary { operator, right } => unary_expr(env, operator, *right),
        Expr::Binary {
            left,
            operator,
            right,
        } => binary_expr(env, operator, *left, *right),
        Expr::Variable(tok) => var_expr(env, tok),
        Expr::Error => panic!(), // TODO: handle this case
        _ => Ok(TokenType::NIL),
    }
}

fn execute(env: &mut Environment, stmt: Stmt) -> Result<TokenType> {
    match stmt {
        Stmt::Expr(expr) => expr_stmt(env, expr),
        Stmt::Print(value) => print_stmt(env, value),
        Stmt::Var { name, initializer } => var_stmt(env, name, initializer),
        _ => Ok(TokenType::NIL),
    }
}

fn expr_stmt(env: &mut Environment, expr: Expr) -> Result<TokenType> {
    evaluate(env, expr)?;
    return Ok(TokenType::NIL);
}

fn print_stmt(env: &mut Environment, expr: Expr) -> Result<TokenType> {
    let value: TokenType = evaluate(env, expr)?;
    print!("{}", value.stringify());
    return Ok(TokenType::NIL);
}

fn var_stmt(env: &mut Environment, name: Token, initializer: Expr) -> Result<TokenType> {
    let mut value = TokenType::NIL;
    if initializer != Expr::Null {
        value = evaluate(env, initializer)?
    }
    env.define(name.get_lexeme(), value);
    Ok(TokenType::NIL)
}

// Helpers for each expression type
fn lit_expr(lit: Token) -> TokenType {
    lit.get_type()
}

fn group_expr(env: &Environment, inside: Expr) -> Result<TokenType> {
    Ok(evaluate(env, inside)?)
}

fn unary_expr(env: &Environment, operator: Token, ex: Expr) -> Result<TokenType> {
    let right = evaluate(env, ex)?;
    let line: i32 = operator.get_line();
    match operator.get_type() {
        TokenType::MINUS => Ok(TokenType::NUMBER(-handle_number(right, line)?)),
        TokenType::BANG => negation(is_truthy(right), line),
        _ => Err(anyhow!("Expected Unary Operator: line {}", line)),
    }
}

fn binary_expr(env: &Environment, operator: Token, l: Expr, r: Expr) -> Result<TokenType> {
    let left: TokenType = evaluate(env, l)?;
    let right: TokenType = evaluate(env, r)?;
    let line: i32 = operator.get_line();
    match operator.get_type() {
        TokenType::GREATER => greater(handle_number(left, line)?, handle_number(right, line)?),
        TokenType::GreaterEqual => {
            greater_equal(handle_number(left, line)?, handle_number(right, line)?)
        }
        TokenType::LESS => less(handle_number(left, line)?, handle_number(right, line)?),
        TokenType::LessEqual => less_equal(handle_number(left, line)?, handle_number(right, line)?),
        TokenType::EqualEqual => is_equal(left, right),
        TokenType::BangEqual => negation(is_equal(left, right)?, line),
        TokenType::MINUS => Ok(TokenType::NUMBER(
            handle_number(left, line)? - handle_number(right, line)?,
        )),
        TokenType::SLASH => Ok(TokenType::NUMBER(
            handle_number(left, line)? / handle_number(right, line)?,
        )),
        TokenType::STAR => Ok(TokenType::NUMBER(
            handle_number(left, line)? * handle_number(right, line)?,
        )),
        TokenType::PLUS => {
            if let TokenType::STRING(string) = left {
                Ok(TokenType::STRING(format!(
                    "{}{}",
                    string,
                    handle_string(right, line)?
                )))
            } else {
                Ok(TokenType::NUMBER(
                    handle_number(left, line)? + handle_number(right, line)?,
                ))
            }
        }
        _ => Err(anyhow!("Incorrect binary operator: {}", line)),
    }
}

fn var_expr(env: &Environment, tok: Token) -> Result<TokenType> {
    env.get(tok)
}

// Other Helpers
fn is_truthy(ttype: TokenType) -> TokenType {
    match ttype {
        TokenType::NIL => TokenType::FALSE,
        TokenType::FALSE => TokenType::FALSE,
        _ => TokenType::TRUE,
    }
}
fn negation(ttype: TokenType, line: i32) -> Result<TokenType> {
    if let TokenType::TRUE = ttype {
        Ok(TokenType::FALSE)
    } else if let TokenType::FALSE = ttype {
        Ok(TokenType::TRUE)
    } else {
        Err(anyhow!("Expected Boolean: {}", line))
    }
}
fn greater(l: f64, r: f64) -> Result<TokenType> {
    if l > r {
        Ok(TokenType::TRUE)
    } else {
        Ok(TokenType::FALSE)
    }
}
fn greater_equal(l: f64, r: f64) -> Result<TokenType> {
    if l >= r {
        Ok(TokenType::TRUE)
    } else {
        Ok(TokenType::FALSE)
    }
}
fn less(l: f64, r: f64) -> Result<TokenType> {
    if l < r {
        Ok(TokenType::TRUE)
    } else {
        Ok(TokenType::FALSE)
    }
}
fn less_equal(l: f64, r: f64) -> Result<TokenType> {
    if l <= r {
        Ok(TokenType::TRUE)
    } else {
        Ok(TokenType::FALSE)
    }
}
fn is_equal(left: TokenType, right: TokenType) -> Result<TokenType> {
    if left == right {
        Ok(TokenType::TRUE)
    } else {
        Ok(TokenType::FALSE)
    }
}

fn handle_number(ttype: TokenType, line: i32) -> Result<f64> {
    if let TokenType::NUMBER(num) = ttype {
        Ok(num)
    } else {
        Err(anyhow!("Expected Number: {}", line))
    }
}
fn handle_string(ttype: TokenType, line: i32) -> Result<String> {
    if let TokenType::STRING(str) = ttype {
        Ok(str)
    } else {
        Err(anyhow!("Expected String: {} ", line))
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
    fn new_group_expr(ex: Expr) -> Expr {
        Expr::Grouping(Box::new(ex))
    }
    fn new_unary_expr(ttype: TokenType, ex: Expr) -> Expr {
        Expr::Unary {
            operator: tok(ttype),
            right: Box::new(ex),
        }
    }
    fn new_binary_expr(ttype: TokenType, left: Expr, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator: tok(ttype),
            right: Box::new(right),
        }
    }

    // ── evaluate ─────────────────────────────────────────
    // Since evaluate is just a match statement that unwraps expressions, before calling relevant helpers, testing both the helpers and evaluate is redundant
    #[test]
    fn test_large_num_expr() {
        let one = num_expr(1.0);
        let two = num_expr(2.0);
        let three = num_expr(3.0);
        let ntwo = new_unary_expr(TokenType::MINUS, two);
        let four = new_binary_expr(TokenType::PLUS, one, three);
        let group = new_group_expr(four);
        let neight = new_binary_expr(TokenType::STAR, group, ntwo);
        let result = match evaluate(neight) {
            Ok(ttype) => ttype,
            Err(_err) => {
                panic!();
            }
        };
        assert_eq!(result, TokenType::NUMBER(-8.0));
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
        assert_eq!(group_expr(num_expr(9.0)).unwrap(), TokenType::NUMBER(9.0));
    }
    #[test]
    fn group_expr_nested() {
        assert_eq!(
            group_expr(Expr::Grouping(Box::new(num_expr(2.0)))).unwrap(),
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
        assert_eq!(negation(TokenType::TRUE, 0).unwrap(), TokenType::FALSE);
    }
    #[test]
    fn negate_false_returns_true() {
        assert_eq!(negation(TokenType::FALSE, 0).unwrap(), TokenType::TRUE);
    }
    #[test]
    #[should_panic(expected = "Expected Boolean: 0")]
    fn negate_non_bool_panics() {
        negation(TokenType::NUMBER(1.0), 0).unwrap();
    }

    // ── unary_expr ───────────────────────────────────────
    #[test]
    fn unary_minus_negates_number() {
        let op = tok(TokenType::MINUS);
        assert_eq!(
            unary_expr(op, num_expr(5.0)).unwrap(),
            TokenType::NUMBER(-5.0)
        );
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
        assert_eq!(evaluate(outer).unwrap(), TokenType::NUMBER(3.0));
    }
    #[test]
    fn unary_bang_true() {
        let op = tok(TokenType::BANG);
        assert_eq!(
            unary_expr(op, new_lit_expr(TokenType::TRUE)).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn unary_bang_false() {
        let op = tok(TokenType::BANG);
        assert_eq!(
            unary_expr(op, new_lit_expr(TokenType::FALSE)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn unary_bang_nil() {
        let op = tok(TokenType::BANG);
        assert_eq!(
            unary_expr(op, new_lit_expr(TokenType::NIL)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn unary_bang_number() {
        let op = tok(TokenType::BANG);
        assert_eq!(unary_expr(op, num_expr(42.0)).unwrap(), TokenType::FALSE);
    }
    #[test]
    #[should_panic(expected = "unary operator")]
    fn unary_unknown_operator_panics() {
        // PLUS is not a valid unary operator
        unary_expr(tok(TokenType::PLUS), num_expr(1.0)).unwrap();
    }
    #[test]
    #[should_panic(expected = "Not a number")]
    fn unary_minus_on_non_number_panics() {
        unary_expr(tok(TokenType::MINUS), str_expr("x")).unwrap();
    }

    // ── binary_expr: arithmetic ──────────────────────────
    #[test]
    fn binary_plus_numbers() {
        let op = tok(TokenType::PLUS);
        assert_eq!(
            binary_expr(op, num_expr(3.0), num_expr(4.0)).unwrap(),
            TokenType::NUMBER(7.0)
        );
    }
    #[test]
    fn binary_plus_strings() {
        let op = tok(TokenType::PLUS);
        assert_eq!(
            binary_expr(op, str_expr("a"), str_expr("b")).unwrap(),
            TokenType::STRING("ab".to_owned())
        );
    }
    #[test]
    fn binary_minus() {
        let op = tok(TokenType::MINUS);
        assert_eq!(
            binary_expr(op, num_expr(10.0), num_expr(3.0)).unwrap(),
            TokenType::NUMBER(7.0)
        );
    }
    #[test]
    fn binary_star() {
        let op = tok(TokenType::STAR);
        assert_eq!(
            binary_expr(op, num_expr(6.0), num_expr(7.0)).unwrap(),
            TokenType::NUMBER(42.0)
        );
    }
    #[test]
    fn binary_slash() {
        let op = tok(TokenType::SLASH);
        assert_eq!(
            binary_expr(op, num_expr(10.0), num_expr(2.0)).unwrap(),
            TokenType::NUMBER(5.0)
        );
    }
    #[test]
    fn binary_slash_by_zero() {
        let op = tok(TokenType::SLASH);
        let result = binary_expr(op, num_expr(1.0), num_expr(0.0)).unwrap();
        assert!(matches!(result, TokenType::NUMBER(v) if v.is_infinite()));
    }
    #[test]
    #[should_panic]
    fn binary_plus_mixed_type_string_rhs() {
        let op = tok(TokenType::PLUS);
        binary_expr(op, str_expr("x"), num_expr(1.0)).unwrap();
    }

    // ── binary_expr: comparison ──────────────────────────
    #[test]
    fn binary_greater_true() {
        let op = tok(TokenType::GREATER);
        assert_eq!(
            binary_expr(op, num_expr(5.0), num_expr(3.0)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_greater_false() {
        let op = tok(TokenType::GREATER);
        assert_eq!(
            binary_expr(op, num_expr(2.0), num_expr(2.0)).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_greater_equal_true() {
        let op = tok(TokenType::GreaterEqual);
        assert_eq!(
            binary_expr(op, num_expr(2.0), num_expr(2.0)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_greater_equal_false() {
        let op = tok(TokenType::GreaterEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_less_true() {
        let op = tok(TokenType::LESS);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_less_false() {
        let op = tok(TokenType::LESS);
        assert_eq!(
            binary_expr(op, num_expr(3.0), num_expr(3.0)).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_less_equal_true() {
        let op = tok(TokenType::LessEqual);
        assert_eq!(
            binary_expr(op, num_expr(3.0), num_expr(3.0)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_less_equal_false() {
        let op = tok(TokenType::LessEqual);
        assert_eq!(
            binary_expr(op, num_expr(4.0), num_expr(3.0)).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_equal_equal_true() {
        let op = tok(TokenType::EqualEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(1.0)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_equal_equal_false() {
        let op = tok(TokenType::EqualEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_equal_equal_different_types() {
        let op = tok(TokenType::EqualEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), str_expr("1")).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn binary_bang_equal_true() {
        let op = tok(TokenType::BangEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(2.0)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn binary_bang_equal_false() {
        let op = tok(TokenType::BangEqual);
        assert_eq!(
            binary_expr(op, num_expr(1.0), num_expr(1.0)).unwrap(),
            TokenType::FALSE
        );
    }

    #[test]
    #[should_panic(expected = "Incorrect binary operator")]
    fn binary_unknown_operator_panics() {
        binary_expr(tok(TokenType::LeftParen), num_expr(1.0), num_expr(2.0)).unwrap();
    }
    #[test]
    #[should_panic(expected = "Not a number")]
    fn binary_minus_on_strings_panics() {
        binary_expr(tok(TokenType::MINUS), str_expr("a"), str_expr("b")).unwrap();
    }

    // ── is_equal ─────────────────────────────────────────
    #[test]
    fn is_equal_same_number() {
        assert_eq!(
            is_equal(TokenType::NUMBER(5.0), TokenType::NUMBER(5.0)).unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn is_equal_different_number() {
        assert_eq!(
            is_equal(TokenType::NUMBER(5.0), TokenType::NUMBER(6.0)).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn is_equal_same_string() {
        assert_eq!(
            is_equal(
                TokenType::STRING("X".to_owned()),
                TokenType::STRING("X".to_owned())
            )
            .unwrap(),
            TokenType::TRUE
        );
    }
    #[test]
    fn is_equal_different_string() {
        assert_eq!(
            is_equal(
                TokenType::STRING("a".to_owned()),
                TokenType::STRING("A".to_owned())
            )
            .unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn is_equal_different_kind() {
        assert_eq!(
            is_equal(TokenType::TRUE, TokenType::NUMBER(1.0)).unwrap(),
            TokenType::FALSE
        );
        assert_eq!(
            is_equal(TokenType::NIL, TokenType::FALSE).unwrap(),
            TokenType::FALSE
        );
    }
    #[test]
    fn is_equal_bool() {
        assert_eq!(
            is_equal(TokenType::TRUE, TokenType::TRUE).unwrap(),
            TokenType::TRUE
        );
        assert_eq!(
            is_equal(TokenType::FALSE, TokenType::FALSE).unwrap(),
            TokenType::TRUE
        );
        assert_eq!(
            is_equal(TokenType::TRUE, TokenType::FALSE).unwrap(),
            TokenType::FALSE
        );
    }
}
