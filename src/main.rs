// use std::os::unix::fs;

use std::cell::Cell;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::process;

use crate::callable::{Callables, Clock};
use crate::environment::Environment;
use crate::parser::Parser;
use crate::scanner::TokenType;
use crate::scanner::token::Token;

mod callable;
mod environment;
pub mod expr;
mod interp;
mod parser;
pub mod scanner;
mod stmt;

pub struct Lox<'a> {
    had_error: Cell<bool>,
    had_runtime_error: Cell<bool>,
    env: Box<Environment<'a>>,
}

fn main() {
    let arglength = std::env::args().len();
    let mut interp = Lox::new_lox();
    if arglength > 2 {
        print!("To many arguments");
        process::exit(63);
    } else if arglength == 2 {
        interp.run_file(std::env::args().nth(1).expect("no file given"));
    } else {
        interp.run_prompt();
    }
}

impl<'a> Lox<'a> {
    pub fn new_lox() -> Lox<'a> {
        let lox = Lox {
            had_error: Cell::new(false),
            had_runtime_error: Cell::new(false),
            env: Box::new(Environment::new()),
        };
        lox.env.define(
            String::from("clock"),
            TokenType::LitFun(Box::new(Callables::Clock(Clock {}))),
        );
        lox
    }
    fn run_file(&mut self, filepath: String) {
        println!("Command line argument: {:?}", filepath);
        let contents = std::fs::read_to_string(filepath).expect("File should have opened");
        // println!("File contents:\n{contents}");
        self.run(contents);
        if self.had_error.get() {
            std::process::exit(65)
        }
        if self.had_runtime_error.get() {
            std::process::exit(75)
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
                    self.had_error.set(false);
                }
                _ => {}
            }
        }
    }

    fn run(&mut self, input: String) {
        // println!("{}", input);

        // Scan input
        let tokens;
        {
            let mut scanner = Box::new(scanner::Scanner::new_scanner(input, self));
            tokens = scanner.scan_tokens();
            // dbg!(tokens.clone());
        }
        let statements;
        {
            let mut parser = Parser::new_parser(tokens, self);
            statements = parser.parse();
        }
        // Stop if there was an error
        if self.had_error.get() {
            return;
        }

        // Interpret statements
        interp::interpret(statements, self);
    }

    pub fn scan_error(&self, line: i32, message: String) {
        self.report(line, "".to_string(), message);
    }

    pub fn parse_error(&self, tok: Token, message: String) {
        match tok.get_type() {
            TokenType::EOF => self.report(tok.get_line(), " at end".to_string(), message),
            _ => self.report(
                tok.get_line(),
                format! {" at '{}'", tok.get_lexeme()},
                message,
            ),
        }
    }

    pub fn runtime_error(&mut self, message: String) {
        self.runtime_report(message);
    }

    fn report(&self, line: i32, loc: String, message: String) {
        println!("[line {} ] Error {}: {}", line, loc, message);
        self.had_error.set(true);
    }

    fn runtime_report(&mut self, message: String) {
        println!("{}", message);
        self.had_runtime_error.set(true);
    }
}
