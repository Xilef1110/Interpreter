use crate::Environment;
use crate::TokenType;

pub trait Callable {
    fn arity(&self) -> i32;
    fn call(&self) -> TokenType;
}

struct LoxFunction<'a> {
    ttype: TokenType,
    arguments: Vec<TokenType>,
    env: &'a Box<Environment<'a>>,
}

impl<'a> Callable for LoxFunction<'a> {
    fn arity(&self) -> i32 {
        return self.arguments.len() as i32;
    }

    fn call(&self) -> TokenType {
        TokenType::NIL
        // TODO
    }
}
