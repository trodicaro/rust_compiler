// interp.rs
//
// Interpret Lox code

use std::rc::Rc;

use crate::AST;
use crate::ast::Expression::*;
use crate::ast::Statement::*;
use crate::ast::{Expression, Statement, Statements};
use crate::ast::Op::*;
use crate::parse::parse_expression_string;
use crate::environ::Environment;

pub fn interpret(ast : &AST) {
    println!("========= Interpreting Lox");
    interpret_statements(ast, &Environment::new());
}

#[derive(PartialEq, Clone, Debug)]
pub enum LoxValue {
    LNumber(f64),       // Runtime representation of Lox values.
    LString(String),
    LBoolean(bool),
    LNil
}

use LoxValue::*;

pub fn interpret_statements(statements : &Statements, environ : &Rc<Environment>) -> () {
    for stmt in statements.iter() {
    interpret_statement(stmt, environ);
    }
}

pub fn interpret_statement(stmt : &Statement, environ : &Rc<Environment>) -> () {
    match stmt {
    SPrint(value) => {
        let lvalue = interpret_expression(value, environ);
        // Note: This will need to be refined later for the final language.
        // I've modified the print so it appears as something very obvious.
        println!("LOX: {lvalue:?}");
    },
    SExpr(value) => {
        interpret_expression(value, environ);
    },
    SVar(name, value) => {
        let lvalue = interpret_expression(value, environ);
        environ.define(name, lvalue);
    },
    SIf(test, consequence, alternative) => {
        let tvalue = interpret_expression(test, environ);
        if is_truthy(&tvalue) {
        interpret_statement(consequence, environ);
        } else {
        interpret_statement(alternative, environ);
        }
    },
    SWhile(test, body) => {
        while is_truthy(&interpret_expression(test, environ)) {
        interpret_statement(body, environ);
        }
    },
    SAssignment(location, body) => {
        match location {
        EName(name) => environ.set(name, interpret_expression(body, environ)),
        _ => panic!("Can't assign to that")
        }
    },
    SBlock(statements) => {
        interpret_statements(statements, &Environment::new_scope(environ))
    }
    }
}

fn is_truthy(lvalue : &LoxValue) -> bool {
    // See section 7.2.4
    match lvalue {
    LBoolean(false) | LNil => false,
    _ => true
    }
}
// Tree-walk interpreter (simplest thing you can do, but not fastest)
pub fn interpret_expression(expr : &Expression, environ : &Rc<Environment>) -> LoxValue {
    match expr {
    ENumber(value) => {
        LNumber(*value)       // In AST, value was already f64
    },
    EString(value) => {
        LString(value.clone())
    },
    EBoolean(value) => {
        LBoolean(*value)
    },
    ENil => {
        LNil
    },
    EName(name) => {
        if let Some(lvalue) = environ.lookup(name) {
        lvalue
        } else {
        panic!("Undefined variable name")
        }
    }
    EBinary(op, left, right) => {
        let leftval = interpret_expression(left, environ);
        let rightval = interpret_expression(right, environ);
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
        interpret_expression(value, environ)
    },
    EUnary(op, value) => {
        let lvalue = interpret_expression(value, environ);
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
    assert_eq!(interpret_expression(&expr, &Environment::new()), LNumber(14.0));
    let expr = parse_expression_string("(2 + 3) * (4 + 5)");
    assert_eq!(interpret_expression(&expr, &Environment::new()), LNumber(45.0));
    let expr = parse_expression_string("(2 + 3) < (4 + 5)");
    assert_eq!(interpret_expression(&expr, &Environment::new()), LBoolean(true));
}
