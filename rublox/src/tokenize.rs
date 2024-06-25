// tokenize.rs
//
// Tokenize Lox

use std::str::Chars;

use crate::{Source, Tokens, Token};
use crate::TokenType::*;

struct Scanner {
    source : String,       // Input text
    index : usize,         // Current scan position
}

impl Scanner {
    fn new(source : String) -> Scanner {
    Scanner { source, index: 0 }
    }
    fn peek(&self, n : usize) -> &str {
    if self.index + n >= self.source.len() {
        ""
    } else {
        &self.source[self.index..self.index+n]
    }
    }
    fn remaining(&self) -> Chars {
    self.source[self.index..].chars()
    }

    // Return the next token on the input.
    fn next_token(&mut self) -> Option<Token> {
    if let Some(tok) = self.match_any() {
        self.index += tok.lexeme.len();
        Some(tok)
    } else {
        None
    }
    }
    fn match_any(&self) -> Option<Token> {
    if let Some(tok) = self.match_identifier() {
        return Some(tok);
    }
    if let Some(tok) = self.match_two_character_symbol() {
        return Some(tok);
    }
    if let Some(tok) = self.match_one_character_symbol() {
        return Some(tok);
    }
    return None
    }
    // Match any single character symbol like "+", ".", etc.
    fn match_one_character_symbol(&self) -> Option<Token> {
    match self.peek(1) {
        "+" => Some(Token::new(PLUS, "+", 0)),
        "-" => Some(Token::new(MINUS, "-", 0)),
        "*" => Some(Token::new(STAR, "*", 0)),
        "(" => Some(Token::new(LPAREN, "(", 0)),
        ")" => Some(Token::new(RPAREN, ")", 0)),
        "{" => Some(Token::new(LBRACE, "{", 0)),
        "}" => Some(Token::new(RBRACE, "}", 0)),
        ";" => Some(Token::new(SEMICOLON, ";", 0)),
        "," => Some(Token::new(COMMA, ",", 0)),
        "." => Some(Token::new(DOT, ".", 0)),
        "=" => Some(Token::new(ASSIGN, "=", 0)),
        ">" => Some(Token::new(GT, ">", 0)),
        "<" => Some(Token::new(LT, "<", 0)),
        "!" => Some(Token::new(BANG, "!", 0)),
        "/" => Some(Token::new(SLASH, "/", 0)),
        _ => None
    }
    }
    // Match any two-character symbol like "<=", "==", "!=", etc.
    fn match_two_character_symbol(&self) -> Option<Token> {
    match self.peek(2) {
        "<=" => Some(Token::new(LE, "<=", 0)),
        ">=" => Some(Token::new(GE, ">=", 0)),
        "!=" => Some(Token::new(NE, "!=", 0)),
        "==" => Some(Token::new(EQ, "==", 0)),
        _ => None
    }
    }
    fn match_identifier(&self) -> Option<Token> {
    let mut chars = self.remaining();
    if let Some(ch) = chars.next() {
        if ch.is_alphabetic() || ch == '_' {
        // Iterate over remaining characters looking for valid identifier characters
        let mut lexeme = String::new();
        lexeme.push(ch);
        for ch in chars {
            if ch.is_alphanumeric() || ch == '_' {
            lexeme.push(ch);
            } else {
            break;
            }
        }
        Some(Token::new(IDENTIFIER, &lexeme, 0))
        } else {
        None
        }
    } else {
        None
    }
    }

    fn match_number(&self) -> Option<Token> {
    todo!();
    }
    fn match_comment(&self) -> Option<Token> {
    todo!();
    }
    fn match_whitespace(&self) -> Option<Token> {
    todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests here
}

#[test]
fn test_scanner() {
    let mut scan = Scanner::new(String::from("hello world"));
    assert_eq!(scan.peek(1), "h");
    assert_eq!(scan.peek(2), "he");
}

#[test]
fn test_match_one_character_symbol() {
    let scanner = Scanner::new(String::from("+*"));
    let t = scanner.match_one_character_symbol();
    assert_eq!(t, Some(Token::new(PLUS, "+", 0)));

    let scanner = Scanner::new(String::from("a"));
    let t = scanner.match_one_character_symbol();
    assert_eq!(t, None);
}

#[test]
fn test_match_two_character_symbol() {
    let scanner = Scanner::new(String::from("<= "));
    let t = scanner.match_two_character_symbol();
    assert_eq!(t, Some(Token::new(LE, "<=", 0)));
}

#[test]
fn test_next_token() {
    let mut scanner = Scanner::new(String::from("<<=abc "));
    let t = scanner.next_token();
    assert_eq!(t, Some(Token::new(LT, "<", 0)));
    let t = scanner.next_token();
    assert_eq!(t, Some(Token::new(LE, "<=", 0)));
    let t = scanner.next_token();
    assert_eq!(t, Some(Token::new(IDENTIFIER, "abc", 0)));
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

    // Need to know how to make a tokens list
    // let mut toks: Tokens = Vec::new();
    // toks.push(Token { toktype: PLUS, lexeme: String::from("+"), line:1 });
    // toks.push(Token { toktype: MINUS, lexeme: String::from("-"), line:1 });

    println!("tokens={tokens:?}");
    tokens
}

fn tokenize_number() {
}

fn tokenize_identifier() {
}

#[test]
fn test_smoke() {
    // A basic test just to make sure tests are running
    let t = Token::new(PLUS, "+", 1);
    assert_eq!(t.toktype, PLUS);
    assert_eq!(t.lexeme, "+");
    assert_eq!(t.line, 1);
    assert_eq!(t, Token::new(PLUS, "+", 1));
}