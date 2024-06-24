// lib.rs

pub mod reader;
pub mod tokenize;
pub mod interp;
pub mod parse;

// Type definitions here?
pub type Filename = String;       // () = "Unit" (kind of like "None" in Python)
pub type Source = String;
pub type Tokens = ();
pub type AST = ();

// Commentary:  I'm pretty sure I don't want to write my entire project
// inside `main.rs` or inside `lib.rs`.   So currently trying to figure
// out how to "best" organize a multifile project.   Put everything in
// the top level?  Use a submodule?
//
// There's a tradeoff here.   Entire Lox project is not *THAT* big so
// I don't necessarily want to overengineer it.  But, I'd definitely
// like to figure out some basic source structure now before I get too
// far deep into actual implementation.

#[derive(Debug)]
pub enum TokenType {
    // Symbols
    LPAREN, RPAREN, LBRACE, RBRACE,   // ( ) { }
    COMMA, DOT, MINUS, PLUS,          // , . - +
    SEMICOLON, SLASH, STAR,           // ; / *
    BANG, ASSIGN, NE, EQ,             // ! = != ==
    GT, GE, LT, LE,                   // > >= < <=
    
    // literals
    IDENTIFIER,
    STRING(String),
    NUMBER(f64),

    // Keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    EOF
}

#[derive(Debug)]
pub struct Token {
    toktype : TokenType,
    lexeme  : String,
    // Discussion: CI (Crafting Interpreters) describes an extra field
    // here called "literal" that is set to Object (meaning any Java object).
    // I am not aware of anything comparable to that in Rust. So, I am
    // leaving it off completely.   A token has a type, a value, and a line.
    // However: see https://github.com/dabeaz-course/rust_2024_06/discussions/5
    line : i32,
}



