// ast.rs
//
// Abstract Syntax Tree (AST) for Lox.

use std::fmt;

// All of the valid operators
#[derive(PartialEq, Debug)]
pub enum Op {
    OpPlus,       // +
    OpMinus,      // -
    OpMult,       // *
    OpDiv,        // /
    OpLt,         // <
    OpLe,         // <=
    OpGt,         // >
    OpGe,         // >=
    OpEq,         // ==
    OpNe,         // !=
    OpNot,        // !
}

// This allows the Op enum to be converted into a nice string for printing, formatting, etc.
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    OpPlus => write!(f, "+"),
	    OpMinus => write!(f, "-"),
	    OpMult => write!(f, "*"),
	    OpDiv => write!(f, "/"),
	    OpLt => write!(f, "<"),
	    OpLe => write!(f, "<="),
	    OpGt => write!(f, ">"),
	    OpGe => write!(f, ">="),
	    OpEq => write!(f, "=="),
	    OpNe => write!(f, "!="),
	    OpNot => write!(f, "!"),
	}
    }
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    ENumber(f64),       // A number like 123 or 123.45
    EString(String),    // A string like "hello"
    EBoolean(bool),     // A boolean like true or false
    ENil,               // nil
    EBinary(Op, Box<Expression>, Box<Expression>),   // expr + expr
    EUnary(Op, Box<Expression>),                     // -expr
    EGroup(Box<Expression>),                         // ( expr )
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    SPrint(Expression),      // print expr ;
    SExpr(Expression),       // expr ;   (Statement expression)
}

use crate::ast::Expression::*;
use crate::ast::Op::*;
use crate::ast::Statement::*;

// Turn an expression into nicely formatted Lox code
pub fn format_expression(expr : &Expression) -> String {
    match expr {
	ENumber(value) => {
	    value.to_string()
	},
	EString(value) => {
	    String::from(value)
	},
	EBoolean(value) => {
	    if *value { String::from("true") } else { String::from("false") }
	},
	ENil => {
	    String::from("nil")
	},
	EBinary(op, left, right) => {
	    format!("{} {} {}", format_expression(left), op, format_expression(right))
	},
	EGroup(value) => {
	    format!("({})", format_expression(value))
	},
	EUnary(op, value) => {
	    format!("{}{}", op, format_expression(value))
	}
    }
}

pub fn format_statement(stmt : &Statement) -> String {
    match stmt {
	SPrint(value) => {
	    format!("print {};\n", format_expression(value))
	},
	SExpr(value) => {
	    format!("{};\n", format_expression(value))
	},
    }
}

#[test]
pub fn test_formatting() {
    // Example encoding of expressions

    // 2 + 3
    let expr1 = EBinary(OpPlus,
			Box::new(ENumber(2.0)),
			Box::new(ENumber(3.0)));
    let fmt1 = format_expression(&expr1);
    assert_eq!(fmt1, "2 + 3");
    
    // 2 + (3 * 4)
    let expr2 = EBinary(OpPlus,
		       Box::new(ENumber(2.0)),
		       Box::new(EGroup(Box::new(EBinary(OpMult,
							Box::new(ENumber(3.0)),
							Box::new(ENumber(4.0)))))));
    let fmt2 = format_expression(&expr2);
    assert_eq!(fmt2, "2 + (3 * 4)");

    // print 2;
    let stmt3 = SPrint(ENumber(2.0));
    let fmt3 = format_statement(&stmt3);
    assert_eq!(fmt3, "print 2;\n");
}