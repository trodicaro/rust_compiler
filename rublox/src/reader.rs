// reader.rs
//
// Read Lox source code from a file (or wherever)

// What I'm trying to do here... Figure out how to link types
// defined here to types used by another module. But, I'm just stubbing...
use crate::{Filename, Source};
use std::fs::File;
use std::io::Read;

pub fn read_source(filename : &Filename) -> Source {
    println!("Reading source code");
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Can't read file");
    println!("source={contents:?}");
    contents
}
