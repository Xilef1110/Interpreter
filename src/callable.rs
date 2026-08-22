use crate::Environment;
use crate::Token;
use crate::TokenType;
use crate::stmt::Stmt;
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

trait_enum::trait_enum! {
    #[derive(Clone, Debug, PartialEq)]
    pub enum Callables: Callable {
        LoxFunction,
    }
}
