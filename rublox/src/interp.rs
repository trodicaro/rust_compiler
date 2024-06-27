// interp.rs
//
// Interpret Lox code

use crate::AST;
use crate::ast::Expression::*;
use crate::ast::Statement::*;
use crate::ast::{Expression, Statement, Statements};
use crate::ast::Op::*;
use crate::parse::parse_expression_string;

pub fn interpret(ast : &AST) {
    println!("========= Interpreting Lox");
    interpret_statements(ast);
}

#[derive(PartialEq, Debug)]
enum LoxValue {
    LNumber(f64),       // Runtime representation of Lox values.
    LString(String),
    LBoolean(bool),
    LNil
}

use LoxValue::*;

pub fn interpret_statements(statements : &Statements) -> () {
    for stmt in statements.iter() {
    interpret_statement(stmt);
    }
}

pub fn interpret_statement(stmt : &Statement) -> () {
    match stmt {
    SPrint(value) => {
        let lvalue = interpret_expression(value);
        println!("LOX: {lvalue:?}");
    },
    SExpr(value) => {
        interpret_expression(value);
    },
    }
}

// Tree-walk interpreter (simplest thing you can do, but not fastest)
pub fn interpret_expression(expr : &Expression) -> LoxValue {
    match expr {
    ENumber(value) => {
        LNumber(*value)       // In AST, value was already f64
    },
    EString(value) => {
        todo!()
    },
    EBoolean(value) => {
        LBoolean(*value)
    },
    ENil => {
        LNil
    },
    EBinary(op, left, right) => {
        let leftval = interpret_expression(left);   // Box<Expression>
        let rightval = interpret_expression(right);
        match (leftval, op, rightval) {
        // Numeric operations
        (LNumber(lv), OpPlus, LNumber(rv)) => { LNumber(lv+rv) },
        (LNumber(lv), OpMinus, LNumber(rv)) => { LNumber(lv-rv) },
        (LNumber(lv), OpMult, LNumber(rv)) => { LNumber(lv*rv) },
        (LNumber(lv), OpDiv, LNumber(rv)) => { LNumber(lv/rv) },
        (LNumber(lv), OpLt, LNumber(rv)) => { LBoolean(lv < rv) },
        (LNumber(lv), OpLe, LNumber(rv)) => { LBoolean(lv <= rv) },
        (LNumber(lv), OpGt, LNumber(rv)) => { LBoolean(lv > rv) },
        (LNumber(lv), OpGe, LNumber(rv)) => { LBoolean(lv >= rv) },
        (LNumber(lv), OpEq, LNumber(rv)) => { LBoolean(lv == rv) },
        (LNumber(lv), OpNe, LNumber(rv)) => { LBoolean(lv != rv) },
        // String operations
        (LString(lv), OpPlus, LString(rv)) => { LString(lv+&rv) },
        (LString(lv), OpEq, LString(rv)) => { LBoolean(lv == rv) },
        (LString(lv), OpNe, LString(rv)) => { LBoolean(lv != rv) },
        // Boolean operations
        (LBoolean(lv), OpEq, LBoolean(rv)) => { LBoolean(lv == rv) },
        (LBoolean(lv), OpNe, LBoolean(rv)) => { LBoolean(lv != rv) },

        _ => {
            // 34 + "hello"
            panic!("Unsupported operation")
        }
        }
    },
    EGroup(value) => {
        interpret_expression(value)
    },
    EUnary(op, value) => {
        let lvalue = interpret_expression(value);
        match (op, lvalue) {
        (OpMinus, LNumber(v)) => { LNumber(-v) },

        // What is "truthy"?
        (OpNot, LBoolean(v)) => { LBoolean(!v) },
        _ => {
            panic!("Unsupported operation")
        }
        }
    }
    }
}

#[test]
fn test_interpret() {
    let expr = parse_expression_string("2 + 3 * 4");
    assert_eq!(interpret_expression(&expr), LNumber(14.0));
    let expr = parse_expression_string("(2 + 3) * (4 + 5)");
    assert_eq!(interpret_expression(&expr), LNumber(45.0));
    let expr = parse_expression_string("(2 + 3) < (4 + 5)");
    assert_eq!(interpret_expression(&expr), LBoolean(true));

}
