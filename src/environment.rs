use crate::{Token, TokenType};
use anyhow::{Result, anyhow};
use std::cell::RefCell;
use std::collections::HashMap;
#[derive(Clone)]
pub struct Environment {
    map: RefCell<HashMap<String, TokenType>>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_top_environment() -> Environment {
        Environment {
            map: RefCell::new(HashMap::new()),
            enclosing: Option::None,
        }
    }
    pub fn new_environment(enclosing: Box<Environment>) -> Environment {
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
}
