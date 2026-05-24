use std::collections::HashMap;

use crate::scanner::token::token_type::TokenType;
pub fn gen_keywords() -> Box<HashMap<String, TokenType>> {
    let mut keywords = HashMap::new();
    keywords.insert("and".to_string(), TokenType::AND);
    keywords.insert("class".to_string(), TokenType::CLASS);
    keywords.insert("else".to_string(), TokenType::ELSE);
    keywords.insert("false".to_string(), TokenType::FALSE);
    keywords.insert("for".to_string(), TokenType::FOR);
    keywords.insert("fun".to_string(), TokenType::FUN);
    keywords.insert("if".to_string(), TokenType::AND);
    keywords.insert("nil".to_string(), TokenType::NIL);
    keywords.insert("or".to_string(), TokenType::OR);
    keywords.insert("print".to_string(), TokenType::PRINT);
    keywords.insert("return".to_string(), TokenType::RETURN);
    keywords.insert("super".to_string(), TokenType::SUPER);
    keywords.insert("this".to_string(), TokenType::THIS);
    keywords.insert("true".to_string(), TokenType::TRUE);
    keywords.insert("var".to_string(), TokenType::VAR);
    keywords.insert("while".to_string(), TokenType::WHILE);
    Box::new(keywords)
}

pub fn get_keyword(name: &str) -> TokenType {
    match name {
        "and" => TokenType::AND,
        "class" => TokenType::CLASS,
        "else" => TokenType::ELSE,
        "false" => TokenType::FALSE,
        "for" => TokenType::FOR,
        "fun" => TokenType::FUN,
        "if" => TokenType::IF,
        "nil" => TokenType::NIL,
        "or" => TokenType::OR,
        "print" => TokenType::PRINT,
        "return" => TokenType::RETURN,
        "super" => TokenType::SUPER,
        "this" => TokenType::THIS,
        "true" => TokenType::TRUE,
        "var" => TokenType::VAR,
        "while" => TokenType::WHILE,
        _ => TokenType::IDENTIFIER,
    }
}
