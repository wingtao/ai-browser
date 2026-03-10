use std::rc::Rc;

use anyhow::anyhow;

use super::{
    ast::{AssignTarget, BinaryOp, Expr, Program, Stmt, UnaryOp},
    parser::parse_program,
    runtime::{
        gc::GcRuntime,
        object::Object,
        scope::{EnvRef, Environment},
        value::{FunctionValue, Value},
    },
};
use crate::engine::webapi::{console::format_console_args, dom_bindings::JsDocumentBinding};

#[derive(Debug)]
enum Flow {
    Normal(Value),
    Return(Value),
}

#[derive(Debug)]
pub struct Interpreter {
    global: EnvRef,
    gc: GcRuntime,
}

impl Default for Interpreter {
    fn default() -> Self {
        let global = Environment::new(None);
        let mut vm = Self {
            global,
            gc: GcRuntime::default(),
        };
        vm.define_native_function("print", |args| {
            println!("{}", format_console_args(&args));
            Ok(Value::Undefined)
        });
        vm
    }
}

impl Interpreter {
    pub fn define_native_function<F>(&mut self, name: &str, func: F)
    where
        F: Fn(Vec<Value>) -> Result<Value, String> + 'static,
    {
        Environment::define(
            &self.global,
            name.to_string(),
            Value::NativeFunction {
                name: name.to_string(),
                func: Rc::new(func),
            },
        );
    }

    pub fn install_dom_apis(&mut self, binding: JsDocumentBinding) {
        let read_binding = binding.clone();
        self.define_native_function("dom_get_text", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            Ok(read_binding
                .query_selector_text(&selector)
                .map(Value::String)
                .unwrap_or(Value::Undefined))
        });

        self.define_native_function("dom_set_text", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            let text = args.get(1).map(ToString::to_string).unwrap_or_default();
            let changed = binding.set_first_text_for_tag(&selector, &text);
            Ok(Value::Bool(changed))
        });
    }

    pub fn eval(&mut self, code: &str) -> anyhow::Result<Value> {
        let program = parse_program(code)?;
        self.eval_program(&program, Rc::clone(&self.global))
    }

    fn eval_program(&mut self, program: &Program, env: EnvRef) -> anyhow::Result<Value> {
        let mut last = Value::Undefined;
        for stmt in &program.body {
            match self.eval_stmt(stmt, Rc::clone(&env))? {
                Flow::Normal(v) => last = v,
                Flow::Return(v) => return Ok(v),
            }
        }
        Ok(last)
    }

    fn eval_stmt(&mut self, stmt: &Stmt, env: EnvRef) -> anyhow::Result<Flow> {
        match stmt {
            Stmt::VarDecl { name, init, .. } => {
                let val = if let Some(expr) = init {
                    self.eval_expr(expr, Rc::clone(&env))?
                } else {
                    Value::Undefined
                };
                Environment::define(&env, name.clone(), val.clone());
                Ok(Flow::Normal(val))
            }
            Stmt::FunctionDecl { name, params, body } => {
                let func = Value::Function(Rc::new(FunctionValue {
                    name: Some(name.clone()),
                    params: params.clone(),
                    body: body.clone(),
                    closure: Rc::clone(&env),
                }));
                Environment::define(&env, name.clone(), func.clone());
                Ok(Flow::Normal(func))
            }
            Stmt::Return(expr) => {
                let value = if let Some(expr) = expr {
                    self.eval_expr(expr, env)?
                } else {
                    Value::Undefined
                };
                Ok(Flow::Return(value))
            }
            Stmt::If {
                test,
                consequent,
                alternate,
            } => {
                if self.eval_expr(test, Rc::clone(&env))?.truthy() {
                    self.eval_stmt(consequent, env)
                } else if let Some(alt) = alternate {
                    self.eval_stmt(alt, env)
                } else {
                    Ok(Flow::Normal(Value::Undefined))
                }
            }
            Stmt::While { test, body } => {
                let mut last = Value::Undefined;
                while self.eval_expr(test, Rc::clone(&env))?.truthy() {
                    match self.eval_stmt(body, Rc::clone(&env))? {
                        Flow::Normal(v) => last = v,
                        Flow::Return(v) => return Ok(Flow::Return(v)),
                    }
                }
                Ok(Flow::Normal(last))
            }
            Stmt::Block(stmts) => {
                let block = Environment::new(Some(env));
                let mut last = Value::Undefined;
                for stmt in stmts {
                    match self.eval_stmt(stmt, Rc::clone(&block))? {
                        Flow::Normal(v) => last = v,
                        Flow::Return(v) => return Ok(Flow::Return(v)),
                    }
                }
                Ok(Flow::Normal(last))
            }
            Stmt::Expr(expr) => {
                let value = self.eval_expr(expr, env)?;
                Ok(Flow::Normal(value))
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr, env: EnvRef) -> anyhow::Result<Value> {
        match expr {
            Expr::Number(v) => Ok(Value::Number(*v)),
            Expr::String(v) => Ok(Value::String(v.clone())),
            Expr::Bool(v) => Ok(Value::Bool(*v)),
            Expr::Null => Ok(Value::Null),
            Expr::Undefined => Ok(Value::Undefined),
            Expr::Identifier(name) => {
                Environment::get(&env, name).ok_or_else(|| anyhow!("未定义变量: {name}"))
            }
            Expr::ObjectLiteral(props) => {
                let obj = Object::new();
                self.gc.track_alloc();
                for (k, v_expr) in props {
                    let value = self.eval_expr(v_expr, Rc::clone(&env))?;
                    Object::set(&obj, k.clone(), value);
                }
                Ok(Value::Object(obj))
            }
            Expr::Unary { op, expr } => {
                let value = self.eval_expr(expr, env)?;
                match op {
                    UnaryOp::Neg => Ok(Value::Number(-to_number(&value))),
                    UnaryOp::Not => Ok(Value::Bool(!value.truthy())),
                }
            }
            Expr::Binary { op, left, right } => {
                let l = self.eval_expr(left, Rc::clone(&env))?;
                let r = self.eval_expr(right, env)?;
                eval_binary(*op, l, r)
            }
            Expr::Assignment { target, value } => {
                let rhs = self.eval_expr(value, Rc::clone(&env))?;
                match target {
                    AssignTarget::Identifier(name) => {
                        if !Environment::assign(&env, name, rhs.clone()) {
                            return Err(anyhow!("赋值失败，变量不存在: {name}"));
                        }
                        Ok(rhs)
                    }
                    AssignTarget::Member { object, property } => {
                        let obj = self.eval_expr(object, env)?;
                        match obj {
                            Value::Object(obj_ref) => {
                                Object::set(&obj_ref, property.clone(), rhs.clone());
                                Ok(rhs)
                            }
                            _ => Err(anyhow!("成员赋值目标不是对象")),
                        }
                    }
                }
            }
            Expr::Member { object, property } => {
                let value = self.eval_expr(object, env)?;
                match value {
                    Value::Object(obj) => Ok(Object::get(&obj, property)),
                    _ => Err(anyhow!("成员访问目标不是对象")),
                }
            }
            Expr::Call { callee, args } => {
                let callee_value = self.eval_expr(callee, Rc::clone(&env))?;
                let mut arg_values = Vec::with_capacity(args.len());
                for arg in args {
                    arg_values.push(self.eval_expr(arg, Rc::clone(&env))?);
                }
                self.call(callee_value, arg_values)
            }
        }
    }

    fn call(&mut self, callee: Value, args: Vec<Value>) -> anyhow::Result<Value> {
        match callee {
            Value::Function(func) => {
                let frame = Environment::new(Some(Rc::clone(&func.closure)));
                for (idx, param) in func.params.iter().enumerate() {
                    let arg = args.get(idx).cloned().unwrap_or(Value::Undefined);
                    Environment::define(&frame, param.clone(), arg);
                }
                if let Some(name) = &func.name {
                    Environment::define(&frame, name.clone(), Value::Function(Rc::clone(&func)));
                }
                let mut ret = Value::Undefined;
                for stmt in &func.body {
                    match self.eval_stmt(stmt, Rc::clone(&frame))? {
                        Flow::Normal(v) => ret = v,
                        Flow::Return(v) => return Ok(v),
                    }
                }
                Ok(ret)
            }
            Value::NativeFunction { func, .. } => (func)(args).map_err(|e| anyhow!(e)),
            _ => Err(anyhow!("调用目标不可执行")),
        }
    }
}

fn to_number(v: &Value) -> f64 {
    match v {
        Value::Number(n) => *n,
        Value::Bool(true) => 1.0,
        Value::Bool(false) => 0.0,
        Value::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
        Value::Null => 0.0,
        Value::Undefined => f64::NAN,
        Value::Object(_) | Value::Function(_) | Value::NativeFunction { .. } => f64::NAN,
    }
}

fn eval_binary(op: BinaryOp, left: Value, right: Value) -> anyhow::Result<Value> {
    use BinaryOp::*;
    match op {
        Add => match (left, right) {
            (Value::String(a), b) => Ok(Value::String(format!("{a}{b}"))),
            (a, Value::String(b)) => Ok(Value::String(format!("{a}{b}"))),
            (a, b) => Ok(Value::Number(to_number(&a) + to_number(&b))),
        },
        Sub => Ok(Value::Number(to_number(&left) - to_number(&right))),
        Mul => Ok(Value::Number(to_number(&left) * to_number(&right))),
        Div => Ok(Value::Number(to_number(&left) / to_number(&right))),
        Eq => Ok(Value::Bool(left == right)),
        NotEq => Ok(Value::Bool(left != right)),
        Gt => Ok(Value::Bool(to_number(&left) > to_number(&right))),
        Gte => Ok(Value::Bool(to_number(&left) >= to_number(&right))),
        Lt => Ok(Value::Bool(to_number(&left) < to_number(&right))),
        Lte => Ok(Value::Bool(to_number(&left) <= to_number(&right))),
        And => Ok(if left.truthy() { right } else { left }),
        Or => Ok(if left.truthy() { left } else { right }),
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use crate::engine::{dom::parser::parse_html, webapi::dom_bindings::JsDocumentBinding};

    #[test]
    fn eval_basic_arithmetic() {
        let mut vm = Interpreter::default();
        let out = vm.eval("let a = 1 + 2 * 3; a;").unwrap();
        assert_eq!(out, Value::Number(7.0));
    }

    #[test]
    fn eval_closure() {
        let mut vm = Interpreter::default();
        let out = vm
            .eval(
                r#"
                function makeAdder(x) {
                  function inner(y) { return x + y; }
                  return inner;
                }
                let add2 = makeAdder(2);
                add2(5);
            "#,
            )
            .unwrap();
        assert_eq!(out, Value::Number(7.0));
    }

    #[test]
    fn eval_while_and_object_member() {
        let mut vm = Interpreter::default();
        let out = vm
            .eval(
                r#"
                let i = 0;
                let obj = { total: 0 };
                while (i < 3) {
                    obj.total = obj.total + i;
                    i = i + 1;
                }
                obj.total;
            "#,
            )
            .unwrap();
        assert_eq!(out, Value::Number(3.0));
    }

    #[test]
    fn eval_dom_bridge_functions() {
        let doc = parse_html("<html><body><h1>Hello</h1></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        let mut vm = Interpreter::default();
        vm.install_dom_apis(binding);
        let out = vm
            .eval(r#"dom_set_text("h1", "World"); dom_get_text("h1");"#)
            .unwrap();
        assert_eq!(out, Value::String("World".to_string()));
    }
}
