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
        ' ' | '\t' | '\r' => (),     // Whitespace ignored
        '\n' => { line += 1; },      // Newline ignored (but line number bumped up)
        '+' => tokens.push(Token::new(PLUS, "+", line)),
        '-' => tokens.push(Token::new(MINUS, "-", line)),
        '*' => tokens.push(Token::new(STAR, "*", line)),
        '(' => tokens.push(Token::new(LPAREN, "(", line)),
        ')' => tokens.push(Token::new(RPAREN, ")", line)),
        '{' => tokens.push(Token::new(LBRACE, "{", line)),
        '}' => tokens.push(Token::new(RBRACE, "}", line)),
        ';' => tokens.push(Token::new(SEMICOLON, ";", line)),
        ',' => tokens.push(Token::new(COMMA, ",", line)),
        '.' => tokens.push(Token::new(DOT, ".", line)),
        '=' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token::new(EQ, "==", line))
        } else {
            tokens.push(Token::new(ASSIGN, "=", line))
        }
        },
        '>' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token::new(GE, ">=", line))
        } else {
            tokens.push(Token::new(GT, ">", line))
        }
        },
        '<' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token::new(LE, "<=", line))
        } else {
            tokens.push(Token::new(LT, "<", line))
        }
        },
        '!' => {
        if let Some('=') = chars.peek() {
            chars.next();
            tokens.push(Token::new(NE, "!=", line))
        } else {
            tokens.push(Token::new(BANG, "!", line))
        }
        },

        // TODO : Must modify to / recognized comments (//)
        '/' => {
        if let Some('/') = chars.peek() {
            chars.next();
            while let Some(nextch) = chars.next() {
            if nextch == '\n' {
                line += 1;
                break
            }
            }
        } else {
            tokens.push(Token::new(SLASH, "/", line))
        }
        }

        // TODO: Numeric Literals
        // TODO: Strings
        // TODO: Identifiers and reserved words
        'a'..='z' | 'A'..='Z' | '_' => {
        let mut lexeme = String::new();
        lexeme.push(ch);
        while let Some(nextch) = chars.peek() {
            match *nextch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                lexeme.push(*nextch);
                chars.next();
            },
            _ => {
                break;
            }
            }
        }
        tokens.push(Token::new(IDENTIFIER, &lexeme, line));
        }
        _ => println!("Bad character {ch:?}"),
    }
    }
    /*
    let t = Token { toktype: PLUS, lexeme: String::from("+"), line:1 };
    println!("tok = {t:?}");

    // Need to know how to make a tokens list
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
