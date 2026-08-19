use crate::Token;
use crate::expr::Expr;

#[derive(PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Class,
    Expr(Expr),
    Func,
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Print(Expr),
    Return,
    Var {
        name: Token,
        initializer: Expr,
    },
    While,
    None,
}
