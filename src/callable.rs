use crate::Environment;
use crate::Token;
use crate::TokenType;
use crate::stmt::Stmt;
use std::time::SystemTime;
use trait_enum;

pub trait Callable {
    fn arity(&self) -> i32;
    fn call_fun(
        &self,
        globals: &Box<Environment>,
        environment: &Box<Environment>,
        arguments: Vec<TokenType>,
    ) -> TokenType;
}

trait_enum::trait_enum! {
    #[derive(Clone, Debug, PartialEq)]
    pub enum Callables: Callable {
        LoxFunction,
        Clock,
    }
}

/* Lox Function
A type for user defined functions */

#[derive(Debug, Clone, PartialEq)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    // arguments: Vec<TokenType>,
}

impl LoxFunction {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> LoxFunction {
        LoxFunction { name, params, body }
    }

    pub fn to_strint(&self) -> String {
        format!("<fn {}>", self.name.get_lexeme())
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> i32 {
        return self.params.len() as i32;
    }

    fn call_fun(
        &self,
        globals: &Box<Environment>,
        env: &Box<Environment>,
        arguments: Vec<TokenType>,
    ) -> TokenType {
        TokenType::NIL
        // TODO
    }
}

/* Native Functions */

// Clock

#[derive(Debug, Clone, PartialEq)]
pub struct Clock {}

impl Callable for Clock {
    fn arity(&self) -> i32 {
        0
    }

    fn call_fun(
        &self,
        _globals: &Box<Environment>,
        _environment: &Box<Environment>,
        _arguments: Vec<TokenType>,
    ) -> TokenType {
        TokenType::NUMBER(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        )
    }
}
