use crate::{Token, TokenType};
use anyhow::{Result, anyhow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Clone, PartialEq, Debug)]
pub struct Environment {
    map: RefCell<HashMap<String, TokenType>>,
    enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: RefCell::new(HashMap::new()),
            enclosing: Option::None,
        }
    }
    pub fn new_nested(enclosing: Rc<Environment>) -> Environment {
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

    fn get_with_string(&self, name: String) -> Result<TokenType> {
        match self.map.borrow().get(&name) {
            Some(ttype) => Ok(ttype.clone()),
            None => match &self.enclosing {
                Option::Some(env) => return env.get_with_string(name),
                Option::None => Err(anyhow!("Undefined variable",)),
            },
        }
    }

    pub fn get_at(&self, distance: i32, name: String) -> Result<TokenType> {
        // dbg!(distance);
        if distance > 0 {
            match &self.enclosing {
                Option::Some(env) => return env.get_at(distance - 1, name),
                Option::None => panic!(),
            }
        } else {
            match self.map.borrow().get(&name) {
                Option::Some(value) => Ok(value.clone()),
                Option::None => {
                    dbg!(name);
                    panic!("Incorrect distance calculation");
                }
            }
            // self.get_with_string(name)
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

    pub fn assign_at(&self, distance: i32, name: Token, value: TokenType) -> bool {
        if distance != 0 {
            match &self.enclosing {
                Option::Some(env) => return env.assign_at(distance - 1, name, value),
                Option::None => panic!(),
            }
        } else {
            self.assign(name.get_lexeme(), value)
        }
    }

    pub fn get_enclosing(&self) -> Option<Rc<Environment>> {
        self.enclosing.clone()
    }
}
