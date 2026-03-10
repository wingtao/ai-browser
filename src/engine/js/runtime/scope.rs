use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::value::Value;

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Debug, Default)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<EnvRef>,
}

impl Environment {
    pub fn new(parent: Option<EnvRef>) -> EnvRef {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            parent,
        }))
    }

    pub fn define(env: &EnvRef, name: impl Into<String>, value: Value) {
        env.borrow_mut().values.insert(name.into(), value);
    }

    pub fn get(env: &EnvRef, name: &str) -> Option<Value> {
        if let Some(v) = env.borrow().values.get(name) {
            return Some(v.clone());
        }
        env.borrow()
            .parent
            .as_ref()
            .and_then(|parent| Self::get(parent, name))
    }

    pub fn assign(env: &EnvRef, name: &str, value: Value) -> bool {
        if env.borrow().values.contains_key(name) {
            env.borrow_mut().values.insert(name.to_string(), value);
            return true;
        }
        if let Some(parent) = env.borrow().parent.as_ref() {
            return Self::assign(parent, name, value);
        }
        false
    }
}
