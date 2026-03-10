use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::value::Value;

pub type ObjectRef = Rc<RefCell<Object>>;

#[derive(Debug, Default)]
pub struct Object {
    pub properties: HashMap<String, Value>,
    pub prototype: Option<ObjectRef>,
}

impl Object {
    pub fn new() -> ObjectRef {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn get(obj: &ObjectRef, key: &str) -> Value {
        if let Some(v) = obj.borrow().properties.get(key) {
            return v.clone();
        }
        if let Some(proto) = &obj.borrow().prototype {
            return Self::get(proto, key);
        }
        Value::Undefined
    }

    pub fn set(obj: &ObjectRef, key: impl Into<String>, value: Value) {
        obj.borrow_mut().properties.insert(key.into(), value);
    }
}
