use rublox::tokenize::*;
use rublox::parse::*;
use rublox::interp::*;

fn main() {
    println!("Hello, Lox!");
    // Interpreter is going to involve some different steps
    tokenize();
    parse();
    interpret();
}
