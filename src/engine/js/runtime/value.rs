use std::{fmt::Display, rc::Rc};

use crate::engine::js::ast::Stmt;

use super::{object::ObjectRef, scope::EnvRef};

pub type NativeFn = fn(Vec<Value>) -> Result<Value, String>;

#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: EnvRef,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    Object(ObjectRef),
    Function(Rc<FunctionValue>),
    NativeFunction { name: &'static str, func: NativeFn },
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            Value::Null | Value::Undefined => false,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Object(_) | Value::Function(_) | Value::NativeFunction { .. } => true,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (Null, Null) => true,
            (Undefined, Undefined) => true,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Null => write!(f, "null"),
            Value::Undefined => write!(f, "undefined"),
            Value::Object(_) => write!(f, "[object Object]"),
            Value::Function(func) => {
                write!(
                    f,
                    "[Function {}]",
                    func.name.as_deref().unwrap_or("anonymous")
                )
            }
            Value::NativeFunction { name, .. } => write!(f, "[NativeFunction {name}]"),
        }
    }
}
