use crate::{Token, TokenType};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

pub struct Environment {
    map: HashMap<String, TokenType>,
}

impl Environment {
    pub fn define(&mut self, name: String, ttype: TokenType) {
        self.map.insert(name, ttype);
    }

    pub fn get(&self, name: Token) -> Result<TokenType> {
        match self.map.get(&name.get_lexeme()) {
            Some(ttype) => Ok(ttype.clone()),
            None => Err(anyhow!(
                "Undefined variable '{}': line {}",
                name.get_lexeme(),
                name.get_line()
            )),
        }
    }
}
