// interp.rs
//
// Interpret Lox code

use crate::AST;
use crate::ast::Expression::*;
use crate::ast::Expression;
use crate::ast::Op::*;
use crate::parse::parse_expression_string;

pub fn interpret(_ast : &AST) {
    println!("Interpreting Lox");
}

#[derive(PartialEq, Debug)]
enum LoxValue {
    LNumber(f64),       // Runtime representation of Lox values.
    LString(String),
    LBoolean(bool),
    LNil
}

use LoxValue::*;

// Tree-walk interpreter (not fastest)
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
        (LNumber(lv), OpPlus, LNumber(rv)) => { LNumber(lv+rv) },
        (LString(lv), OpPlus, LString(rv)) => { LString(lv+&rv) },
        (LNumber(lv), OpMinus, LNumber(rv)) => { LNumber(lv-rv) },
        (LNumber(lv), OpMult, LNumber(rv)) => { LNumber(lv*rv) },
        (LNumber(lv), OpDiv, LNumber(rv)) => { LNumber(lv/rv) },
        (LNumber(lv), OpLt, LNumber(rv)) => { LBoolean(lv < rv) },
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
