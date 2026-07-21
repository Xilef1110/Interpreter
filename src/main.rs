// use std::os::unix::fs;

use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::process;

// use crate::expr;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::scanner::TokenType;
use crate::scanner::token::Token;

pub mod expr;
mod parser;
pub mod scanner;

pub struct Lox {
    had_error: bool,
}

fn main() {
    let arglength = std::env::args().len();
    let mut interp = Lox { had_error: false };
    if arglength > 2 {
        print!("To many arguments");
        process::exit(63);
    } else if arglength == 2 {
        interp.run_file(std::env::args().nth(0).expect("no file given"));
    } else {
        interp.run_prompt();
    }
}

impl Lox {
    fn run_file(&mut self, filepath: String) {
        println!("Command line argument: {:?}", filepath);
        let contents = std::fs::read_to_string(filepath).expect("File should have opened");
        // println!("File contents:\n{contents}");
        self.run(contents);
        if self.had_error {
            std::process::exit(65)
        }
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            stdout().flush().unwrap();
            match stdin().lines().next() {
                Some(Ok(input)) => {
                    if input.trim() == "q" {
                        break;
                    }
                    if input.trim().is_empty() {
                        continue;
                    }
                    self.run(input);
                    self.had_error = false;
                }
                _ => {}
            }
        }
    }

    fn run(&mut self, input: String) {
        println!("{}", input);
        let mut scanner: Scanner = scanner::Scanner::new_scanner(input, self);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new_parser(tokens, self);
        let expression = parser.parse();
        if self.had_error {
            return;
        }
        print! {"{}", expr::print_expr(expression)}
        // for tok in tokens.into_iter() {
        //     println!("{}", tok.to_string());
        // }
    }

    pub fn scan_error(&mut self, line: i32, message: String) {
        self.report(line, "".to_string(), message);
    }

    pub fn parse_error(&mut self, tok: Token, message: String) {
        match tok.get_type() {
            TokenType::EOF => self.report(tok.get_line(), " at end".to_string(), message),
            _ => self.report(
                tok.get_line(),
                format! {" at '{}'", tok.get_lexeme()},
                message,
            ),
        }
    }

    fn report(&mut self, line: i32, loc: String, message: String) {
        println!("[line {} ] Error {}: {}", line, loc, message);
        self.had_error = true;
    }
}
