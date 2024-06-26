// parse.rs
//
// Parse Lox code

use crate::{Tokens, AST};
use crate::ast::Expression;

pub fn parse(_tokens : &Tokens) -> AST {
    println!("Parsing Lox");
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
//
// PROBLEM: This code doesn't work because it trips up in an infinite recursion.
// Have to fix the grammar and reorganize the code.

struct Parser {
    tokens : Tokens,    // From scanner
    current : usize     // Current position of parse (work left-to-right)
}

impl Parser {
    fn new(tokens: Tokens) -> Parser {
    Parser { tokens, current: 0 }
    }

    // Require the next token to exactly match an expected type or an error
    fn expect(&mut self, tty: TokenType) -> Result<Token, String> {
    if self.tokens[self.current].toktype == tty {
        self.current += 1;
        Ok(self.tokens[self.current-1])
    } else {
        Err("Syntax Error")
    }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
    todo!();
    }
    fn parse_literal(&mut self) -> Result<Expression, String> {
    // if match(NUMBER) return ENumber(...)
    // if match(STRING) return EString(...)
    // if match(TRUE) return EBoolean(true)
    // if match(FALSE) return EBoolean(false)
    // if match(NIL) return ENil
    todo!();
    }
    fn parse_grouping(&mut self) -> Result<Expression, String> {
    // "( expression )"
    // expect(LPAREN)    // Hard requirement
    // value = parse_expression()
    // expect(RPAREN)
    // return EGrouping(value)   // Create AST object
    todo!();
    }
    fn parse_unary(&mut self) -> Result<Expression, String> {
    // op = parse_operator()
    // value = parse_expression()
    // return EUnary(op, value)
    todo!();
    }
    fn parse_binary(&mut self) -> Result<Expression, String> {
    // left = parse_expression()
    // op = parse_operator()
    // right = parse_expression()
    // return EBinary(op, left, right)
    todo!();
    }
    fn parse_operator(&mut self) -> Result<Expression, String> {
    // if match(PLUS) return OpPlus
    // if match(MINUS) return OpMinus
    // if match(STAR) return OpMult
    // if match(SLASH) return OpDiv
    // if match(LT) return OpLt
    // if match(LE) return OpLe
    // if match(GT) return OpGt
    // if match(GE) return OpGe
    // if match(EQ) return OpEq
    // if match(NE) return OpNe
    todo!();
    }
}
