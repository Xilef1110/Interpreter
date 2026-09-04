use crate::interp::{ErrWrap, Interp, Result};
use crate::stmt::Stmt;
use crate::{Environment, Token, TokenType, interp};
use std::rc::Rc;
use std::time::SystemTime;
use trait_enum;

pub trait Callable {
    fn arity(&self) -> i32;
    fn call_fun(&self, interp: &mut Interp, arguments: Vec<TokenType>) -> Result<TokenType>;
    fn to_string(&self) -> String;
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
    closure: Rc<Environment>,
    // arguments: Vec<TokenType>,
}

impl LoxFunction {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<Environment>,
    ) -> LoxFunction {
        LoxFunction {
            name,
            params,
            body,
            closure,
        }
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> i32 {
        return self.params.len() as i32;
    }

    fn call_fun(&self, interp: &mut Interp, arguments: Vec<TokenType>) -> Result<TokenType> {
        let environment = Environment::new_nested(self.closure.clone());
        for i in 0..self.params.len() {
            environment.define(self.params[i].get_lexeme(), arguments[i].clone());
        }

        // If interpreting the function returns an error, we have to handle the case where we use error handling to unwind the stack
        if let Err(err) = interp.block_execute(self.body.clone(), Rc::new(environment)) {
            match err {
                ErrWrap::InterpErr(e) => return Err(ErrWrap::InterpErr(e)),
                ErrWrap::ReturnErr(returned) => {
                    return Ok(returned);
                    // if let TokenType::Returned(value) = returned {
                    //     dbg!(value.clone());
                    //     return Ok(*value);
                    // }
                }
            }
        }
        Ok(TokenType::NIL)
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.name.get_lexeme())
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

    fn call_fun(&self, _interp: &mut Interp, _arguments: Vec<TokenType>) -> Result<TokenType> {
        Ok(TokenType::NUMBER(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        ))
    }

    fn to_string(&self) -> String {
        String::from("<native fn>")
    }
}
