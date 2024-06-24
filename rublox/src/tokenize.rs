// tokenize.rs
//
// Tokenize Lox

use crate::{Source, Tokens, Token};
use crate::TokenType::*;

pub fn tokenize(src: &Source) -> Tokens {
    println!("Tokenizing Lox");

    // Make sure I can create a token and actually see it in debugging!
    let t = Token { toktype: PLUS, lexeme: String::from("+"), line:1 };
    println!("tok = {t:?}");

    let mut toks: Vec<Token> = Vec::new();
    toks.push(Token { toktype: PLUS, lexeme: String::from("+"), line:1 });
    toks.push(Token { toktype: PLUS, lexeme: String::from("-"), line:1 });
    println!("toks = {toks:?}");

}

/*
pub fn tokenize_symbol(text: ...) -> Token {
    // +, *, -, (, ), {, }...
    ...
    // How do I match?
    // How do I report error?
    // How do I write a unit test?
	..
}
*/
