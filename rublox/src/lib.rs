pub mod tokenize;
pub mod interp;
pub mod parse;

// Commentary:  I'm pretty sure I don't want to write my entire project
// inside `main.rs` or inside `lib.rs`.   So currently trying to figure
// out how to "best" organize a multifile project.   Put everything in
// the top level?  Use a submodule?
//
// There's a tradeoff here.   Entire Lox project is not *THAT* big so
// I don't necessarily want to overengineer it.  But, I'd definitely
// like to figure out some basic source structure now before I get too
// far deep into actual implementation.
