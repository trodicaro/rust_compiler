use rublox::reader::*;
use rublox::tokenize::*;
use rublox::parse::*;
use rublox::interp::*;
use rublox::Filename;

fn main() {
    println!("Hello, Lox!");
    // Interpreter is going to involve some different steps.  Right now,
    // this is a tremendous amount of "wishful thinking" on my part.
    // But, at a very high level, this is how an interpreter is going to
    // be put together and how the flow of data will work.
    let filename = get_filename_from_args();
    let src = read_source(&filename);
    let tokens = tokenize(&src);
    let ast = parse(tokens);
    interpret(&ast);
}

// use std::env;

// Read the input filename from the command line arguments
fn get_filename_from_args() -> Filename {
    let filename = std::env::args().nth(1).expect("Missing filename");
    println!("Getting filename from command line");
    println!("filename={filename}");
    return filename;
}
