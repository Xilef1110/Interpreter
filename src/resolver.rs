use crate::expr::Expr;
use crate::interp::Interp;
use crate::scanner::TokenType;
use crate::stmt::Stmt;
use crate::{Lox, Token};
use std::collections::HashMap;

pub struct Resolver<'a> {
    interp: &'a Interp,
    lox: &'a Lox,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interp: &'a Interp, lox: &'a Lox) -> Resolver<'a> {
        Resolver {
            interp,
            lox,
            scopes: vec![],
        }
    }

    pub fn resolve_statements(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.resolve_stmt(stmt)
        }
    }

    // Handle Statements
    fn resolve_stmt(&mut self, statement: Stmt) {
        match statement {
            Stmt::Block(statements) => self.block_stmt(statements),
            Stmt::Var { name, initializer } => self.var_stmt(name, initializer),
            Stmt::Func { name, params, body } => self.fun_stmt(name, params, body),
            Stmt::Expr(value) => self.expression_stmt(value),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => self.if_stmt(condition, then_branch, else_branch),
            Stmt::Print(value) => self.print_stmt(value),
            Stmt::Return(_name, value) => self.return_stmt(value),
            Stmt::While(condition, body) => self.while_stmt(condition, body),
            _ => panic!("unimplemented statement in resolver"),
        };
        panic!();
    }

    fn block_stmt(&mut self, statements: Vec<Stmt>) {
        self.begin_scope();
        self.resolve_statements(statements);
        self.end_scope();
    }

    fn var_stmt(&mut self, name: Token, initializer: Expr) {
        self.declare(name.get_lexeme());
        if initializer != Expr::Null {
            self.resolve_expr(initializer);
        }
        self.define(name.get_lexeme());
    }

    fn fun_stmt(&mut self, name: Token, params: Vec<Token>, body: Vec<Stmt>) {
        self.declare(name.get_lexeme());
        self.define(name.get_lexeme());

        self.resolve_fun(params, body);
    }

    fn expression_stmt(&mut self, expr: Expr) {
        self.resolve_expr(expr);
    }

    fn if_stmt(&mut self, condition: Expr, thenBranch: Box<Stmt>, elseBranch: Box<Stmt>) {
        self.resolve_expr(condition);
        self.resolve_stmt(*thenBranch);
        if *elseBranch != Stmt::None {
            self.resolve_stmt(*elseBranch);
        }
    }

    fn print_stmt(&mut self, expression: Expr) {
        self.resolve_expr(expression);
    }

    fn return_stmt(&mut self, value: Expr) {
        if value != Expr::Null {
            self.resolve_expr(value);
        }
    }

    fn while_stmt(&mut self, condition: Expr, body: Box<Stmt>) {
        self.resolve_expr(condition);
        self.resolve_stmt(*body);
    }

    // Handle Expressions
    fn resolve_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Variable(name) => self.var_expr(name),
            Expr::Assign { name, value } => self.assign_expr(name, *value),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.binary_expr(left, right),
            Expr::Call(callee, paren, arguments) => self.call_expr(callee, arguments),
            Expr::Grouping(expr) => self.group_expr(expr),
            Expr::Literal { value } => (),
            Expr::Logical {
                left,
                operator,
                right,
            } => self.logic_expr(left, right),
            Expr::Unary { operator, right } => self.unary_expr(right),
            _ => panic!("Resolved unimplemented expression"),
        };
    }

    fn var_expr(&mut self, name: Token) {
        if !self.scopes.is_empty() {
            if let Option::None = self.peek().get(&name.get_lexeme()) {
                self.lox.parse_error(
                    name.clone(),
                    String::from("Can't read local variable in its own initializer."),
                );
            }
        }
        self.resolve_local(Expr::Variable(name.clone()), name);
    }

    fn assign_expr(&mut self, name: Token, value: Expr) {
        self.resolve_expr(value.clone());
        self.resolve_local(
            Expr::Assign {
                name: name.clone(),
                value: Box::new(value),
            },
            name,
        );
    }

    fn binary_expr(&mut self, left: Box<Expr>, right: Box<Expr>) {
        self.resolve_expr(*left);
        self.resolve_expr(*right);
    }

    fn call_expr(&mut self, callee: Box<Expr>, arguments: Vec<Expr>) {
        self.resolve_expr(*callee);
        for arg in arguments {
            self.resolve_expr(arg);
        }
    }

    fn group_expr(&mut self, expr: Box<Expr>) {
        self.resolve_expr(*expr);
    }

    fn logic_expr(&mut self, left: Box<Expr>, right: Box<Expr>) {
        self.resolve_expr(*left);
        self.resolve_expr(*right);
    }

    fn unary_expr(&mut self, right: Box<Expr>) {
        self.resolve_expr(*right);
    }

    // Helpers
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String) {
        if self.scopes.is_empty() {
            return;
        }

        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name, false);
        self.scopes.push(scope);
    }

    fn define(&mut self, name: String) {
        if self.scopes.is_empty() {
            return;
        }

        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name, true);
        self.scopes.push(scope);
    }

    fn peek(&mut self) -> &HashMap<String, bool> {
        &self.scopes[self.scopes.len()]
    }

    fn resolve_local(&mut self, expr: Expr, name: Token) {
        let mut i = self.scopes.len();
        while i > 0 {
            if self
                .scopes
                .get(i - 1)
                .unwrap()
                .contains_key(&name.get_lexeme())
            {
                self.interp.resolve(expr, self.scopes.len() - 1 - i);
                return;
            }
            i -= 1;
        }
    }

    fn resolve_fun(&mut self, params: Vec<Token>, body: Vec<Stmt>) {
        self.begin_scope();
        for param in params {
            self.declare(param.get_lexeme());
            self.define(param.get_lexeme());
        }
        self.resolve_statements(body);
        self.end_scope();
    }
}
