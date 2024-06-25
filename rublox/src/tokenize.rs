// tokenize.rs
//
// Tokenize Lox

use crate::{Source, Tokens, Token};
use crate::TokenType::*;

struct Scanner {
    source : String,
    tokens : Tokens
}

impl Scanner {

}


pub fn tokenize(src: &Source) -> Tokens {
    println!("Tokenizing Lox");

    let mut tokens: Tokens = Vec::new();
    let mut line = 1;
    //let mut chars = src.chars();
    let mut chars = src.chars().peekable();

    while let Some(ch) = chars.next() {
    println!("ch={ch:?}");
    match ch {
        '+' => tokens.push(Token { toktype: PLUS, lexeme: String::from("+"), line: line }),
        '-' => tokens.push(Token { toktype: MINUS, lexeme: String::from("-"), line: line }),
        '*' => tokens.push(Token { toktype: STAR, lexeme: String::from("*"), line: line }),
        '/' => tokens.push(Token { toktype: SLASH, lexeme: String::from("/"), line: line }),
        '(' => tokens.push(Token { toktype: LPAREN, lexeme: String::from("("), line: line }),
        ')' => tokens.push(Token { toktype: RPAREN, lexeme: String::from(")"), line: line }),
        '{' => tokens.push(Token { toktype: LBRACE, lexeme: String::from("{"), line: line }),
        '}' => tokens.push(Token { toktype: RBRACE, lexeme: String::from("}"), line: line }),
        ';' => tokens.push(Token { toktype: SEMICOLON, lexeme: String::from(";"), line: line }),
        ',' => tokens.push(Token { toktype: COMMA, lexeme: String::from(","), line: line }),
        '.' => tokens.push(Token { toktype: DOT, lexeme: String::from("."), line: line }),
        '=' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token { toktype: EQ, lexeme: String::from("=="), line: line })
        } else {
            tokens.push(Token { toktype: ASSIGN, lexeme: String::from("="), line:line })
        }
        },
        '>' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token { toktype: GE, lexeme: String::from(">="), line: line })
        } else {
            tokens.push(Token { toktype: GT, lexeme: String::from(">"), line:line })
        }
        },
        '<' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token { toktype: LE, lexeme: String::from("<="), line: line })
        } else {
            tokens.push(Token { toktype: LT, lexeme: String::from("<"), line:line })
        }
        },
        '!' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token { toktype: NE, lexeme: String::from("!="), line: line })
        } else {
            tokens.push(Token { toktype: BANG, lexeme: String::from("!"), line:line })
        }
        },
        _ => println!("Bad character {ch:?}"),
    }
    }
    /*
    // Make sure I can create a token and actually see it in debugging!
    let t = Token { toktype: PLUS, lexeme: String::from("+"), line:1 };
    println!("tok = {t:?}");

    let mut toks: Tokens = Vec::new();
    toks.push(Token { toktype: PLUS, lexeme: String::from("+"), line:1 });
    toks.push(Token { toktype: MINUS, lexeme: String::from("-"), line:1 });

    */
    println!("tokens={tokens:?}");
    tokens
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
