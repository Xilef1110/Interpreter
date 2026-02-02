// use std::os::unix::fs;

fn main() {
    let filepath = std::env::args().nth(1).expect("no file given");
    println!("Command line argument: {:?}", filepath);
    let contents = std::fs::read_to_string(filepath).expect("File should have opened");
    println!("File contents:\n{contents}");
}
