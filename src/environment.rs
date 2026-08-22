use crate::{Token, TokenType};
use anyhow::{Result, anyhow};
use std::cell::RefCell;
use std::collections::HashMap;
#[derive(Clone, PartialEq)]
pub struct Environment<'a> {
    map: RefCell<HashMap<String, TokenType>>,
    enclosing: Option<&'a Box<Environment<'a>>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            map: RefCell::new(HashMap::new()),
            enclosing: Option::None,
        }
    }
    pub fn new_nested(enclosing: &'a Box<Environment<'a>>) -> Environment<'a> {
        Environment {
            map: RefCell::new(HashMap::new()),
            enclosing: Option::Some(enclosing),
        }
    }
    pub fn define(&self, name: String, ttype: TokenType) {
        self.map.borrow_mut().insert(name, ttype);
    }

    pub fn get(&self, name: Token) -> Result<TokenType> {
        match self.map.borrow().get(&name.get_lexeme()) {
            Some(ttype) => Ok(ttype.clone()),
            None => match &self.enclosing {
                Option::Some(env) => return env.get(name),
                Option::None => Err(anyhow!(
                    "Undefined variable '{}': line {}",
                    name.get_lexeme(),
                    name.get_line()
                )),
            },
        }
    }

    pub fn assign(&self, name: String, value: TokenType) -> bool {
        if self.map.borrow().contains_key(&name) {
            self.map.borrow_mut().insert(name, value);
            return true;
        }
        match &self.enclosing {
            Option::Some(env) => env.assign(name, value),
            Option::None => false,
        }
    }

    pub fn get_enclosing(&self) -> Option<&'a Box<Environment<'a>>> {
        self.enclosing
    }
}
