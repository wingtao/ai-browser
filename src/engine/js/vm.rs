use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::{anyhow, Context};

use super::{
    ast::{AssignTarget, BinaryOp, Expr, ForInit, Program, Stmt, UnaryOp},
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
    Break,
    Continue,
}

#[derive(Debug)]
pub struct Interpreter {
    global: EnvRef,
    gc: GcRuntime,
    dom_binding: Option<JsDocumentBinding>,
    event_listeners: HashMap<String, Vec<Value>>,
    next_timer_id: u64,
    pending_timers: Vec<(u64, Value)>,
    cancelled_timers: HashSet<u64>,
}

impl Default for Interpreter {
    fn default() -> Self {
        let global = Environment::new(None);
        let mut vm = Self {
            global,
            gc: GcRuntime::default(),
            dom_binding: None,
            event_listeners: HashMap::new(),
            next_timer_id: 0,
            pending_timers: Vec::new(),
            cancelled_timers: HashSet::new(),
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
        self.dom_binding = Some(binding.clone());
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

        let qs_binding = binding.clone();
        self.define_native_function("querySelector", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            Ok(qs_binding
                .query_selector_text(&selector)
                .map(Value::String)
                .unwrap_or(Value::Undefined))
        });

        let read_all_binding = binding.clone();
        self.define_native_function("dom_get_all_text", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            let all = read_all_binding.query_selector_all_text(&selector);
            Ok(Value::String(all.join("\n")))
        });

        let qsa_binding = binding.clone();
        self.define_native_function("querySelectorAll", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            let all = qsa_binding.query_selector_all_text(&selector);
            Ok(Value::String(all.join("\n")))
        });

        let text_binding = binding.clone();
        self.define_native_function("textContent", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            Ok(text_binding
                .query_selector_text(&selector)
                .map(Value::String)
                .unwrap_or(Value::Undefined))
        });

        let set_text_binding = binding.clone();
        self.define_native_function("setTextContent", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            let text = args.get(1).map(ToString::to_string).unwrap_or_default();
            Ok(Value::Bool(
                set_text_binding.set_first_text_for_tag(&selector, &text),
            ))
        });

        let html_binding = binding.clone();
        self.define_native_function("innerHTML", move |args| {
            let selector = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            Ok(html_binding
                .inner_html_for_first_tag(&selector)
                .map(Value::String)
                .unwrap_or(Value::Undefined))
        });

        self.define_native_function("createElement", move |args| {
            let tag = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "div".to_string());
            let text = args.get(1).map(ToString::to_string).unwrap_or_default();
            let obj = Object::new();
            Object::set(&obj, "__tag", Value::String(tag));
            Object::set(&obj, "__text", Value::String(text));
            Ok(Value::Object(obj))
        });

        let append_binding = binding.clone();
        self.define_native_function("appendChild", move |args| {
            let parent = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            let Some(Value::Object(element)) = args.get(1).cloned() else {
                return Ok(Value::Bool(false));
            };
            let tag = match Object::get(&element, "__tag") {
                Value::String(s) if !s.trim().is_empty() => s,
                _ => "div".to_string(),
            };
            let text = match Object::get(&element, "__text") {
                Value::String(s) => s,
                _ => String::new(),
            };
            Ok(Value::Bool(
                append_binding.append_child_text_to_first_tag(&parent, &tag, &text),
            ))
        });

        let remove_binding = binding.clone();
        self.define_native_function("removeChild", move |args| {
            let parent = args
                .first()
                .map(ToString::to_string)
                .unwrap_or_else(|| "body".to_string());
            let child = args
                .get(1)
                .map(ToString::to_string)
                .unwrap_or_else(|| "div".to_string());
            Ok(Value::Bool(
                remove_binding.remove_first_child_tag_from_first_tag(&parent, &child),
            ))
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
                Flow::Break => return Err(anyhow!("break 只能在循环内部使用")),
                Flow::Continue => return Err(anyhow!("continue 只能在循环内部使用")),
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
                    prototype: Object::new(),
                }));
                self.gc.track_alloc();
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
                        Flow::Break => break,
                        Flow::Continue => continue,
                    }
                }
                Ok(Flow::Normal(last))
            }
            Stmt::For {
                init,
                test,
                update,
                body,
            } => {
                let loop_env = Environment::new(Some(env));
                if let Some(init) = init {
                    self.eval_for_init(init, Rc::clone(&loop_env))?;
                }

                let mut last = Value::Undefined;
                loop {
                    if let Some(test) = test {
                        if !self.eval_expr(test, Rc::clone(&loop_env))?.truthy() {
                            break;
                        }
                    }
                    match self.eval_stmt(body, Rc::clone(&loop_env))? {
                        Flow::Normal(v) => last = v,
                        Flow::Return(v) => return Ok(Flow::Return(v)),
                        Flow::Break => break,
                        Flow::Continue => {
                            if let Some(update_expr) = update {
                                self.eval_expr(update_expr, Rc::clone(&loop_env))?;
                            }
                            continue;
                        }
                    }
                    if let Some(update_expr) = update {
                        self.eval_expr(update_expr, Rc::clone(&loop_env))?;
                    }
                }
                Ok(Flow::Normal(last))
            }
            Stmt::Try {
                try_block,
                catch_param,
                catch_block,
                finally_block,
            } => {
                let mut result = self.eval_stmt(try_block, Rc::clone(&env));

                if let Err(err) = result {
                    if let Some(catch_block) = catch_block {
                        let catch_env = Environment::new(Some(Rc::clone(&env)));
                        if let Some(param) = catch_param {
                            Environment::define(
                                &catch_env,
                                param.clone(),
                                Value::String(err.to_string()),
                            );
                        }
                        result = self.eval_stmt(catch_block, catch_env);
                    } else {
                        return Err(err);
                    }
                }

                if let Some(finally_block) = finally_block {
                    match self.eval_stmt(finally_block, Environment::new(Some(env)))? {
                        Flow::Normal(_) => {}
                        Flow::Return(v) => return Ok(Flow::Return(v)),
                        Flow::Break => return Ok(Flow::Break),
                        Flow::Continue => return Ok(Flow::Continue),
                    }
                }

                result
            }
            Stmt::Throw(expr) => {
                let value = self.eval_expr(expr, env)?;
                Err(anyhow!("throw: {value}"))
            }
            Stmt::Break => Ok(Flow::Break),
            Stmt::Continue => Ok(Flow::Continue),
            Stmt::Block(stmts) => {
                let block = Environment::new(Some(env));
                let mut last = Value::Undefined;
                for stmt in stmts {
                    match self.eval_stmt(stmt, Rc::clone(&block))? {
                        Flow::Normal(v) => last = v,
                        Flow::Return(v) => return Ok(Flow::Return(v)),
                        Flow::Break => return Ok(Flow::Break),
                        Flow::Continue => return Ok(Flow::Continue),
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
            Expr::This => {
                Environment::get(&env, "this").ok_or_else(|| anyhow!("this 只能在函数上下文中使用"))
            }
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
                            Value::Function(_) => Err(anyhow!("暂不支持重写函数属性")),
                            _ => Err(anyhow!("成员赋值目标不是对象")),
                        }
                    }
                }
            }
            Expr::Member { object, property } => {
                let value = self.eval_expr(object, env)?;
                match value {
                    Value::Object(obj) => Ok(Object::get(&obj, property)),
                    Value::Function(func) if property == "prototype" => {
                        Ok(Value::Object(Rc::clone(&func.prototype)))
                    }
                    _ => Err(anyhow!("成员访问目标不是对象")),
                }
            }
            Expr::Call { callee, args } => {
                if let Expr::Identifier(name) = callee.as_ref() {
                    if let Some(result) = self.try_eval_builtin_call(name, args, Rc::clone(&env))? {
                        return Ok(result);
                    }
                }
                let callee_value = self.eval_expr(callee, Rc::clone(&env))?;
                let mut arg_values = Vec::with_capacity(args.len());
                for arg in args {
                    arg_values.push(self.eval_expr(arg, Rc::clone(&env))?);
                }
                self.call(callee_value, arg_values)
            }
            Expr::New { callee, args } => {
                let callee_value = self.eval_expr(callee, Rc::clone(&env))?;
                let mut arg_values = Vec::with_capacity(args.len());
                for arg in args {
                    arg_values.push(self.eval_expr(arg, Rc::clone(&env))?);
                }
                self.construct(callee_value, arg_values)
            }
        }
    }

    fn try_eval_builtin_call(
        &mut self,
        name: &str,
        args: &[Expr],
        env: EnvRef,
    ) -> anyhow::Result<Option<Value>> {
        match name {
            "setTimeout" => {
                if args.is_empty() {
                    return Err(anyhow!("setTimeout 至少需要回调参数"));
                }
                let callback = self.eval_expr(&args[0], Rc::clone(&env))?;
                if !matches!(callback, Value::Function(_) | Value::NativeFunction { .. }) {
                    return Err(anyhow!("setTimeout 第一个参数必须是函数"));
                }
                self.next_timer_id += 1;
                let id = self.next_timer_id;
                self.pending_timers.push((id, callback));
                Ok(Some(Value::Number(id as f64)))
            }
            "clearTimeout" => {
                let timer_id = if let Some(expr) = args.first() {
                    match self.eval_expr(expr, Rc::clone(&env))? {
                        Value::Number(n) => n as u64,
                        other => {
                            return Err(anyhow!("clearTimeout 参数必须是 number, got {}", other))
                        }
                    }
                } else {
                    0
                };
                self.cancelled_timers.insert(timer_id);
                Ok(Some(Value::Undefined))
            }
            "runTasks" => {
                self.run_pending_tasks()?;
                Ok(Some(Value::Undefined))
            }
            "addEventListener" => {
                let event = self.eval_expr(
                    args.first()
                        .context("addEventListener(event, callback) 缺少 event 参数")?,
                    Rc::clone(&env),
                )?;
                let callback = self.eval_expr(
                    args.get(1)
                        .context("addEventListener(event, callback) 缺少 callback 参数")?,
                    Rc::clone(&env),
                )?;
                let event = value_to_plain_string(&event);
                if !matches!(callback, Value::Function(_) | Value::NativeFunction { .. }) {
                    return Err(anyhow!("addEventListener callback 必须是函数"));
                }
                self.event_listeners
                    .entry(format!("global::{event}"))
                    .or_default()
                    .push(callback);
                Ok(Some(Value::Undefined))
            }
            "dispatchEvent" => {
                let event = self.eval_expr(
                    args.first()
                        .context("dispatchEvent(event) 缺少 event 参数")?,
                    Rc::clone(&env),
                )?;
                let event = value_to_plain_string(&event);
                self.dispatch_event_keys(vec![format!("global::{event}")])?;
                Ok(Some(Value::Undefined))
            }
            "dom_add_event_listener" => {
                let target = self.eval_expr(
                    args.first()
                        .context("dom_add_event_listener(target,event,callback) 缺少 target")?,
                    Rc::clone(&env),
                )?;
                let event = self.eval_expr(
                    args.get(1)
                        .context("dom_add_event_listener(target,event,callback) 缺少 event")?,
                    Rc::clone(&env),
                )?;
                let callback = self.eval_expr(
                    args.get(2)
                        .context("dom_add_event_listener(target,event,callback) 缺少 callback")?,
                    Rc::clone(&env),
                )?;
                if !matches!(callback, Value::Function(_) | Value::NativeFunction { .. }) {
                    return Err(anyhow!("dom_add_event_listener callback 必须是函数"));
                }
                let target = value_to_plain_string(&target).to_lowercase();
                let event = value_to_plain_string(&event);
                self.event_listeners
                    .entry(format!("dom::{event}::{target}"))
                    .or_default()
                    .push(callback);
                Ok(Some(Value::Undefined))
            }
            "dom_dispatch_event" => {
                let target = self.eval_expr(
                    args.first()
                        .context("dom_dispatch_event(target,event) 缺少 target")?,
                    Rc::clone(&env),
                )?;
                let event = self.eval_expr(
                    args.get(1)
                        .context("dom_dispatch_event(target,event) 缺少 event")?,
                    Rc::clone(&env),
                )?;
                let target = value_to_plain_string(&target).to_lowercase();
                let event = value_to_plain_string(&event);
                let chain = if let Some(binding) = &self.dom_binding {
                    binding.ancestor_chain_for_tag(&target)
                } else {
                    vec![target, "document".to_string()]
                };
                let keys = chain
                    .into_iter()
                    .map(|node| format!("dom::{event}::{node}"))
                    .collect::<Vec<_>>();
                self.dispatch_event_keys(keys)?;
                Ok(Some(Value::Undefined))
            }
            _ => Ok(None),
        }
    }

    fn dispatch_event_keys(&mut self, keys: Vec<String>) -> anyhow::Result<()> {
        for key in keys {
            let listeners = self.event_listeners.get(&key).cloned().unwrap_or_default();
            for callback in listeners {
                let _ = self.call(callback, vec![])?;
            }
        }
        Ok(())
    }

    fn run_pending_tasks(&mut self) -> anyhow::Result<()> {
        let pending = std::mem::take(&mut self.pending_timers);
        for (id, callback) in pending {
            if self.cancelled_timers.contains(&id) {
                continue;
            }
            let _ = self.call(callback, vec![])?;
        }
        Ok(())
    }

    fn eval_for_init(&mut self, init: &ForInit, env: EnvRef) -> anyhow::Result<()> {
        match init {
            ForInit::VarDecl { name, init, .. } => {
                let val = if let Some(expr) = init {
                    self.eval_expr(expr, Rc::clone(&env))?
                } else {
                    Value::Undefined
                };
                Environment::define(&env, name.clone(), val);
            }
            ForInit::Expr(expr) => {
                self.eval_expr(expr, env)?;
            }
        }
        Ok(())
    }

    fn call(&mut self, callee: Value, args: Vec<Value>) -> anyhow::Result<Value> {
        match callee {
            Value::Function(func) => self.execute_function(&func, args, Value::Undefined),
            Value::NativeFunction { func, .. } => (func)(args).map_err(|e| anyhow!(e)),
            _ => Err(anyhow!("调用目标不可执行")),
        }
    }

    fn construct(&mut self, callee: Value, args: Vec<Value>) -> anyhow::Result<Value> {
        match callee {
            Value::Function(func) => {
                let instance = Object::new();
                self.gc.track_alloc();
                instance.borrow_mut().prototype = Some(Rc::clone(&func.prototype));
                let ret =
                    self.execute_function(&func, args, Value::Object(Rc::clone(&instance)))?;
                match ret {
                    Value::Object(_) => Ok(ret),
                    _ => Ok(Value::Object(instance)),
                }
            }
            _ => Err(anyhow!("new 目标不可构造")),
        }
    }

    fn execute_function(
        &mut self,
        func: &Rc<FunctionValue>,
        args: Vec<Value>,
        this_value: Value,
    ) -> anyhow::Result<Value> {
        let frame = Environment::new(Some(Rc::clone(&func.closure)));
        Environment::define(&frame, "this", this_value);
        for (idx, param) in func.params.iter().enumerate() {
            let arg = args.get(idx).cloned().unwrap_or(Value::Undefined);
            Environment::define(&frame, param.clone(), arg);
        }
        if let Some(name) = &func.name {
            Environment::define(&frame, name.clone(), Value::Function(Rc::clone(func)));
        }
        let mut ret = Value::Undefined;
        for stmt in &func.body {
            match self.eval_stmt(stmt, Rc::clone(&frame))? {
                Flow::Normal(v) => ret = v,
                Flow::Return(v) => return Ok(v),
                Flow::Break => return Err(anyhow!("break 不能跨函数边界")),
                Flow::Continue => return Err(anyhow!("continue 不能跨函数边界")),
            }
        }
        Ok(ret)
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

fn value_to_plain_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
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
    fn eval_for_with_break_continue() {
        let mut vm = Interpreter::default();
        let out = vm
            .eval(
                r#"
                let s = 0;
                for (let i = 0; i < 6; i = i + 1) {
                    if (i == 2) { continue; }
                    if (i == 5) { break; }
                    s = s + i;
                }
                s;
            "#,
            )
            .unwrap();
        assert_eq!(out, Value::Number(8.0));
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

    #[test]
    fn eval_dom_get_all_text() {
        let doc = parse_html("<html><body><p>A</p><p>B</p></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        let mut vm = Interpreter::default();
        vm.install_dom_apis(binding);
        let out = vm.eval(r#"dom_get_all_text("p");"#).unwrap();
        assert_eq!(out, Value::String("A\nB".to_string()));
    }

    #[test]
    fn eval_new_and_this() {
        let mut vm = Interpreter::default();
        let out = vm
            .eval(
                r#"
                function User(name) {
                    this.name = name;
                }
                let u = new User("neo");
                u.name;
            "#,
            )
            .unwrap();
        assert_eq!(out, Value::String("neo".to_string()));
    }

    #[test]
    fn eval_function_prototype_read() {
        let mut vm = Interpreter::default();
        let out = vm
            .eval(
                r#"
                function A() {}
                let p = A.prototype;
                p;
            "#,
            )
            .unwrap();
        assert!(matches!(out, Value::Object(_)));
    }

    #[test]
    fn eval_try_catch_finally() {
        let mut vm = Interpreter::default();
        let out = vm
            .eval(
                r#"
                let x = 0;
                try { throw "bad"; }
                catch (e) { x = 2; }
                finally { x = x + 1; }
                x;
            "#,
            )
            .unwrap();
        assert_eq!(out, Value::Number(3.0));
    }

    #[test]
    fn eval_uncaught_throw_should_error() {
        let mut vm = Interpreter::default();
        let err = vm.eval(r#"throw "oops";"#).unwrap_err();
        assert!(err.to_string().contains("throw: oops"));
    }

    #[test]
    fn eval_set_timeout_and_run_tasks() {
        let mut vm = Interpreter::default();
        let out = vm
            .eval(
                r#"
                let x = 0;
                function tick() { x = 7; }
                setTimeout(tick, 0);
                runTasks();
                x;
            "#,
            )
            .unwrap();
        assert_eq!(out, Value::Number(7.0));
    }

    #[test]
    fn eval_dom_event_bubbling() {
        let doc = parse_html("<html><body><div><button>Go</button></div></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        let mut vm = Interpreter::default();
        vm.install_dom_apis(binding);
        let out = vm
            .eval(
                r#"
                let c = 0;
                function onBtn(){ c = c + 1; }
                function onBody(){ c = c + 10; }
                dom_add_event_listener("button", "click", onBtn);
                dom_add_event_listener("body", "click", onBody);
                dom_dispatch_event("button", "click");
                c;
            "#,
            )
            .unwrap();
        assert_eq!(out, Value::Number(11.0));
    }

    #[test]
    fn eval_query_selector_and_dom_mutation_builtins() {
        let doc = parse_html("<html><body><div><p>A</p></div></body></html>").unwrap();
        let binding = JsDocumentBinding::new(Rc::new(RefCell::new(doc)));
        let mut vm = Interpreter::default();
        vm.install_dom_apis(binding);
        let out = vm
            .eval(
                r#"
                let text = querySelector("p");
                let el = createElement("span", "B");
                appendChild("div", el);
                let html = innerHTML("div");
                removeChild("div", "p");
                html;
            "#,
            )
            .unwrap();
        assert!(out.to_string().contains("<p>A</p>"));
        assert!(out.to_string().contains("<span>B</span>"));
    }
}
