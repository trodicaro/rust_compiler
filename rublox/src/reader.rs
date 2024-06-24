// reader.rs
//
// Read Lox source code from a file (or wherever)


// What I'm trying to do here... Figure out how to link types
// defined here to types used by another module. But, I'm just stubbing...
use crate::{Filename, Source};

pub fn read_source(filename : &Filename) -> Source {
    println!("Reading source code");
}
