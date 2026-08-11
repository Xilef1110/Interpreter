use crate::expr::Expr;

pub enum Stmt {
    Block,
    Class,
    Expr(Expr),
    Func,
    If,
    Print(Expr),
    Return,
    Var,
    While,
}
