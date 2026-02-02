// use std::os::unix::fs;

use std::process;

fn main() {
    let arglength = std::env::args().len();
    if (arglength > 2) {
        print!("To many arguments");
        process::exit(64);
    } else if (arglength == 2) {
        run_file(std::env::args().nth(1).expect("no file given"));
    } else {
    }
}

fn run_file(filepath: String) {
    println!("Command line argument: {:?}", filepath);
    let contents = std::fs::read_to_string(filepath).expect("File should have opened");
    println!("File contents:\n{contents}");
}

fn run_prompt() {
    loop {}
}
