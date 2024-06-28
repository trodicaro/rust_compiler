// environ.rs

use std::collections::HashMap;
use crate::interp::LoxValue;
use crate::interp::LoxValue::*;

pub struct Environment {
    values : HashMap<String, LoxValue>,
    // parent : Option<&'a Environment>,    // NOT a copy of parent.
}

impl Environment {
    pub fn new() -> Environment {
	Environment { values : HashMap::<String, LoxValue>::new() }
    }
    // Define a variable for the first time.   This is the "var name;" feature
    pub fn define(&mut self, name: &str, value: &LoxValue) {
	self.values.insert(name.to_string(), value.clone());
    }
    // Look up the value of a variable.   Return if it exists.
    // return clones of everything.
    pub fn lookup(&self, name: &str) -> Option<LoxValue> {
	if let Some(value) = self.values.get(name) {
	    Some(value.clone())
	} else {
	    None
	}
    }
    // Set the value of an existing variable, deleting its old value.
    pub fn set(&mut self, name: &str, value: &LoxValue) {
	if self.values.contains_key(name) {
	    self.values.insert(name.to_string(), value.clone());
	} else {
	    panic!("setting non-existent variable");   // FIXME
	}
    }
}

#[test]
fn test_environment() {
    let mut env = Environment::new();
    env.define("x", &LNumber(4.0));
    assert_eq!(env.lookup("x"), Some(LNumber(4.0)));
    env.set("x", &LNumber(10.0));
    assert_eq!(env.lookup("x"), Some(LNumber(10.0)));
}