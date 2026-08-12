use crate::{Token, TokenType};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

pub struct Environment {
    map: HashMap<String, TokenType>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_top_environment() -> Environment {
        Environment {
            map: HashMap::new(),
            enclosing: Option::None,
        }
    }
    pub fn new_environment(enclosing: Environment) -> Environment {
        Environment {
            map: HashMap::new(),
            enclosing: Option::Some(Box::new(enclosing)),
        }
    }
    pub fn define(&mut self, name: String, ttype: TokenType) {
        self.map.insert(name, ttype);
    }

    pub fn get(&self, name: Token) -> Result<TokenType> {
        match self.map.get(&name.get_lexeme()) {
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

    pub fn assign(&mut self, name: String, value: TokenType) -> bool {
        if self.map.contains_key(&name) {
            self.map.insert(name, value);
            return true;
        }
        match &mut self.enclosing {
            Option::Some(env) => env.assign(name, value),
            Option::None => false,
        }
    }
}
