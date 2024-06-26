// ast.rs
//
// Abstract Syntax Tree (AST) for Lox.

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
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    ENumber(f64),       // A number like 123 or 123.45
    EString(String),    // A string like "hello"
    EBoolean(bool),     // A boolean like true or false
    EBinary(Op, Box<Expression>, Box<Expression>),   // expr + expr
    EUnary(Op, Box<Expression>),                     // -expr
    EGroup(Box<Expression>),                         // ( expr )
}

use crate::ast::Expression::*;
use crate::ast::Op::*;

pub fn format_op(op : &Op) -> String {
    match op {
	OpPlus => String::from("+"),
	OpMinus => String::from("-"),
	OpMult => String::from("*"),
	OpDiv => String::from("/"),
	OpLt => String::from("<"),
	OpLe => String::from("<="),
	OpGt => String::from(">"),
	OpGe => String::from(">="),
	OpEq => String::from("=="),
	OpNe => String::from("!="),
    }
}
	
pub fn format_expression(expr : &Expression) -> String {
    match expr {
	ENumber(value) => { value.to_string() },
	EString(value) => { String::from(value) },
	EBoolean(value) => { if *value { String::from("true") } else
			     { String::from("false") } },
	EBinary(op, left, right) => {
	    format_expression(left) + &format_op(op) + &format_expression(right)
	},
	EGroup(value) => {
	    format!("({})", format_expression(value))
	},
	EUnary(op, value) => {
	    format_op(op) + &format_expression(value)
	}
    }
}


pub fn example() {
    // Example encoding of expressions

    // 2 + 3
    let expr1 = EBinary(OpPlus,
			Box::new(ENumber(2.0)),
			Box::new(ENumber(3.0)));
    println!("{expr1:?}");
    println!("{}", format_expression(&expr1));
    
    // 2 + (3 * 4)
    let expr2 = EBinary(OpPlus,
		       Box::new(ENumber(2.0)),
		       Box::new(EGroup(Box::new(EBinary(OpMult,
							Box::new(ENumber(3.0)),
							Box::new(ENumber(4.0)))))));
    println!("{expr2:?}");
    println!("{}", format_expression(&expr2));    
}