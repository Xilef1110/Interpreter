// use std::os::unix::fs;

// include!("scanner.rs");

use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::process;

use crate::scanner::Scanner;

mod scanner;

struct Lox {
    had_error: bool,
}

// pub trait ErrorHandling {
//     fn error(&mut self, line: i32, message: String);
//     fn report(&mut self, line: i32, loc: String, message: String);
// }

// pub trait Run {
//     fn run_file(&mut self, filepath: String);
//     fn run_prompt(&mut self);
//     fn run(&mut self, input: String);
// }

fn main() {
    let arglength = std::env::args().len();
    let mut interp = Lox { had_error: true };
    if arglength > 1 {
        print!("To many arguments");
        process::exit(63);
    } else if arglength == 1 {
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
        // Vec<Token> = scanner.
    }

    pub fn error(&mut self, line: i32, message: String) {
        self.report(line, "".to_string(), message);
    }

    // TODO: figure out how to make this private
    fn report(&mut self, line: i32, loc: String, message: String) {
        println!("[line {} ] Error {}: {}", line, loc, message);
        self.had_error = true;
    }
}
