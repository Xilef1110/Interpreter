use crate::TokenType;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum ErrWrap {
    ReturnErr(TokenType),
    InterpErr(String),
}

impl ErrWrap {
    pub fn new_return(ttype: TokenType) -> ErrWrap {
        ErrWrap::ReturnErr(ttype)
    }

    pub fn new_interp(msg: String) -> ErrWrap {
        ErrWrap::InterpErr(msg)
    }
}

impl Display for ErrWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Test")
    }
}

impl Error for ErrWrap {}
