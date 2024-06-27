// parse.rs
//
// Parse Lox code

use crate::{Tokens, TokenType, Token, AST};
use crate::ast::{Expression,Statement};
use crate::ast::Expression::*;
use crate::ast::Statement::*;
use crate::ast::Op::*;
use crate::TokenType::*;
use crate::tokenize::tokenize;

pub fn parse(_tokens : &Tokens) -> AST {
    println!("Parsing Lox");
}

pub fn parse_expression_string(src : &str) -> Expression {
    let s = String::from(src);
    let tokens = tokenize(&s);
    let mut parser = Parser::new(tokens);
    parser.parse_expression().expect("failed")
}

pub fn parse_statement_string(src : &str) -> Statement {
    let s = String::from(src);
    let tokens = tokenize(&s);
    let mut parser = Parser::new(tokens);
    parser.parse_statement().expect("failed")
}

// Discussion:  The Lox grammar for expressions is as follows. Tokens are ALLCAPS.
//
//  expression -> literal
//             |  unary
//             |  binary
//             |  grouping ;
//
// literal -> NUMBER | STRING | TRUE | FALSE | NIL ;
// grouping -> LPAREN expression RPAREN ;
// unary -> ( MINUS | BANG) expression ;
// binary -> expression operator expression ;
// operator -> PLUS | MINUS | STAR | SLASH | EQ | NE | LT | LE | GT | GE
//
// One way to approach a parser is to write a separate function for each left-hand-side.
// Each function needs to return something from the AST and possibly an error (if parse
// error).
//
// Strategy for parsing:  You try to work left-to-right over input tokens, matching
// them in order.

struct Parser {
    tokens : Tokens,    // From scanner
    current : usize     // Current position of parse (work left-to-right)
}

impl Parser {
    fn new(tokens: Tokens) -> Parser {
    Parser { tokens, current: 0 }
    }

    fn previous(&self) -> &Token {
    &self.tokens[self.current-1]
    }

    // Check next token *without* consuming it
    fn check(&self, tty: TokenType) -> bool {
    self.current < self.tokens.len() && self.tokens[self.current].toktype == tty
    }

    // If next token matches return true and advance.
    // This is called "match" in Crafting Interpreters
    fn accept(&mut self, tty: TokenType) -> bool {
    if self.check(tty) {
        self.current += 1;
        true
    } else {
        false
    }
    }

    // Require the next token to exactly match an expected type or a syntax error
    fn consume(&mut self, tty: TokenType, message: &str) -> Result<&Token, String> {
    if self.tokens[self.current].toktype == tty {
        self.current += 1;
        Ok(&self.tokens[self.current-1])
    } else {
        Err(String::from(message))
    }
    }

    // Expression Parsing
    fn parse_expression(&mut self) -> Result<Expression, String> {
    self.parse_equality()
    }
    fn parse_equality(&mut self) -> Result<Expression, String> {
    let mut expr = self.parse_comparison()?;
    while self.accept(EQ) || self.accept(NE) {
        let op = match self.previous().toktype {
        EQ => OpEq,
        NE => OpNe,
        _ => panic!("Should not be here")
        };
        expr = EBinary(op, Box::new(expr), Box::new(self.parse_comparison()?));
    }
    Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
    let mut expr = self.parse_term()?;
    while self.accept(LT) || self.accept(LE) || self.accept(GT) || self.accept(GE) {
        let op = match self.previous().toktype {
        LT => OpLt,
        LE => OpLe,
        GT => OpGt,
        GE => OpGe,
        _ => panic!("Should not be here")
        };
        expr = EBinary(op, Box::new(expr), Box::new(self.parse_term()?));
    }
    Ok(expr)
    }
    fn parse_term(&mut self) -> Result<Expression, String> {
    let mut expr = self.parse_factor()?;
    while self.accept(PLUS) || self.accept(MINUS) {
        let op = match self.previous().toktype {
        PLUS => OpPlus,
        MINUS => OpMinus,
        _ => panic!("Should not be here")
        };
        expr = EBinary(op, Box::new(expr), Box::new(self.parse_factor()?));
    }
    Ok(expr)
    }
    fn parse_factor(&mut self) -> Result<Expression, String> {
    let mut expr = self.parse_unary()?;
    while self.accept(SLASH) || self.accept(STAR) {
        let op = match self.previous().toktype {
        SLASH => OpDiv,
        STAR => OpMult,
        _ => panic!("Should not be here")
        };
        expr = EBinary(op, Box::new(expr), Box::new(self.parse_unary()?));
    }
    Ok(expr)
    }
    fn parse_unary(&mut self) -> Result<Expression, String> {
    if self.accept(MINUS) || self.accept(BANG) {
        let op = match self.previous().toktype {
        MINUS => OpMinus,
        BANG => OpNot,
        _ => panic!("Should not be here")
        };
        let right = self.parse_unary()?;
        Ok(EUnary(op, Box::new(right)))
    } else {
        self.parse_primary()
    }
    }
    fn parse_primary(&mut self) -> Result<Expression, String> {
    if self.accept(FALSE) {
        Ok(EBoolean(false))
    } else if self.accept(TRUE) {
        Ok(EBoolean(true))
    } else if self.accept(NIL) {
        Ok(ENil)
    } else if self.accept(NUMBER) {
        Ok(ENumber(self.previous().lexeme.parse().expect("")))
    } else if self.accept(STRING) {
        todo!();
    } else if self.accept(LPAREN) {
        let expr = self.parse_expression()?;
        self.consume(RPAREN, "Expect ')' after expression.")?;
        Ok(EGroup(Box::new(expr)))
    } else {
        Err(String::from("Expected a primary"))
    }
    }
    // Statement parsing
    fn parse_statement(&mut self) -> Result<Statement, String> {
    if self.check(PRINT) {
        self.parse_print()
    } else {
        self.parse_statement_expr()
    }
    }
    fn parse_print(&mut self) -> Result<Statement, String> {
    self.consume(PRINT, "Expected 'print'")?;
    let value = self.parse_expression()?;
    self.consume(SEMICOLON, "Expect ';' after expression.")?;
    Ok(SPrint(value))
    }
    fn parse_statement_expr(&mut self) -> Result<Statement, String> {
    let value = self.parse_expression()?;
    self.consume(SEMICOLON, "Expect ';' after expression.")?;
    Ok(SExpr(value))
    }
}

#[test]
fn test_primaries() {
    assert_eq!(parse_expression_string("1"), ENumber(1.0));
    assert_eq!(parse_expression_string("1.25"), ENumber(1.25));
    assert_eq!(parse_expression_string("true"), EBoolean(true));
    assert_eq!(parse_expression_string("false"), EBoolean(false));
    assert_eq!(parse_expression_string("nil"), ENil);
    // assert_eq!(parse_expression_string("\"hello\""), EString(String::from("hello")));
}

#[test]
fn test_unary() {
    assert_eq!(parse_expression_string("-1"), EUnary(OpMinus, Box::new(ENumber(1.0))));
    assert_eq!(parse_expression_string("!true"), EUnary(OpNot, Box::new(EBoolean(true))));
}

#[test]
fn test_factor() {
    assert_eq!(parse_expression_string("3*4"),
           EBinary(OpMult,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
    assert_eq!(parse_expression_string("3/4"),
           EBinary(OpDiv,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
}

#[test]
fn test_term() {
    assert_eq!(parse_expression_string("3+4"),
           EBinary(OpPlus,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
    assert_eq!(parse_expression_string("3-4"),
           EBinary(OpMinus,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
}
#[test]
fn test_comparison() {
    assert_eq!(parse_expression_string("3<4"),
           EBinary(OpLt,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
    assert_eq!(parse_expression_string("3<=4"),
           EBinary(OpLe,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
    assert_eq!(parse_expression_string("3>4"),
           EBinary(OpGt,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
    assert_eq!(parse_expression_string("3>=4"),
           EBinary(OpGe,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
}

#[test]
fn test_equality() {
    assert_eq!(parse_expression_string("3==4"),
           EBinary(OpEq,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
    assert_eq!(parse_expression_string("3!=4"),
           EBinary(OpNe,
               Box::new(ENumber(3.0)),
               Box::new(ENumber(4.0))));
}

#[test]
fn test_statement() {
    assert_eq!(parse_statement_string("print 3;"),
           SPrint(ENumber(3.0)));
    assert_eq!(parse_statement_string("3;"),
           SExpr(ENumber(3.0)));
}
