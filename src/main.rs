fn main() {
    let python = std::env::args().nth(1).expect("no file given");
    println!("Command line argument: {:?}", python);
}
