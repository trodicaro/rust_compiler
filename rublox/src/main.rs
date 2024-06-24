mod tokenize;
mod parse;
mod interp;

fn main() {
    println!("Hello, Lox!");
    // Interpreter is going to involve some different steps
    tokenize::tokenize();
    parse::parse();
    interp::interpret();
}
