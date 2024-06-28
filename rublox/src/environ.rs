// environ.rs
//
// For the interpreter, we need to manage environments that hold variables.
// The variables are organized into different scopes, which get nested
// inside each other.

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::interp::LoxValue;
use crate::interp::LoxValue::*;

/*
Discussion about Lox variables and scope.

(1) All variables have to live someplace.  The value has to exist somewhere in
    memory and there has to be an association between a name and a value.
    A HashMap seems like a logical choice for this.

(2) Variables are mutable. They can be assigned a new value at any time.

(3) There are nested scopes.   Lox has both block scope and local scoping
    of variables in functions.   Each nested scope is its own environment.
    This environment is created at the start of a block and destroyed at the end.
    Environments need to refer to the parent environment.

(4) Lox has inner functions and closures.  A closure is a function paired with
    the environment that was active when the function was defined.  This means
    that environments might be kept alive for some indeterminate amount of time.
    As an example, if a function was defined inside a block, the environment
    associated with the block is attached to the function behind the scenes.
    This environment is used to create new environments when the function is
    called.  Because of this, it is difficult to pinpoint a single "owner"
    of an environment.  Some kind of garbage collection or management is needed.

General thoughts: Maybe some combination of Rc<> and RefCell<> would be useful here.
 */

pub struct Environment {
    values : RefCell<HashMap<String, LoxValue>>,
    parent : Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Rc<Environment> {
    Rc::new(Environment { values : RefCell::new(HashMap::<String, LoxValue>::new()), parent: None })
    }
    // Create a new environment, but with a parent scope
    pub fn new_scope(parent : &Rc<Environment>) -> Rc<Environment> {
    Rc::new(Environment { values : RefCell::new(HashMap::<String, LoxValue>::new()), parent: Some(parent.clone())})
    }

    // Define a variable for the first time.   This is the "var name;" feature
    pub fn define(&self, name: &str, value: LoxValue) {
    let mut vals = self.values.borrow_mut();
    vals.insert(name.to_string(), value);
    }
    // Look up the value of a variable.   Return if it exists.
    pub fn lookup(&self, name: &str) -> Option<LoxValue> {
    let vals = self.values.borrow();   // Runtime borrow (not compiler checked)
    if let Some(value) = vals.get(name) {
        Some(value.clone())
    } else if let Some(parent) = &self.parent {
        // We don't have the value, but maybe our parent does
        parent.lookup(name)
    } else {
        None
    }
    }
    // Set the value of an existing variable, deleting its old value.
    pub fn set(&self, name: &str, value: LoxValue) {
    let mut vals = self.values.borrow_mut();
    if vals.contains_key(name) {
        vals.insert(name.to_string(), value.clone());
    } else if let Some(parent) = &self.parent {
        parent.set(name, value);
    } else {
        panic!("setting non-existent variable");   // FIXME
    }
    }
}

#[test]
fn test_environment() {
    let env = Environment::new();
    env.define("x", LNumber(4.0));
    assert_eq!(env.lookup("x"), Some(LNumber(4.0)));
    env.set("x", LNumber(10.0));
    assert_eq!(env.lookup("x"), Some(LNumber(10.0)));
}