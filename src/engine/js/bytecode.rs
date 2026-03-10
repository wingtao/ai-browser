use std::collections::HashMap;

use anyhow::anyhow;

use super::{
    ast::{BinaryOp, Expr, Program, Stmt},
    parser::parse_program,
    runtime::value::Value,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Push(Value),
    Load(String),
    Store(String),
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    Not,
    Neg,
    Pop,
    Return,
}

#[derive(Debug, Default)]
pub struct BytecodeCompiler {
    code: Vec<Instruction>,
}

impl BytecodeCompiler {
    pub fn compile_program(program: &Program) -> anyhow::Result<Vec<Instruction>> {
        let mut c = Self { code: vec![] };
        for stmt in &program.body {
            c.compile_stmt(stmt)?;
        }
        c.code.push(Instruction::Return);
        Ok(c.code)
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<()> {
        match stmt {
            Stmt::VarDecl { name, init, .. } => {
                if let Some(expr) = init {
                    self.compile_expr(expr)?;
                } else {
                    self.code.push(Instruction::Push(Value::Undefined));
                }
                self.code.push(Instruction::Store(name.clone()));
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.compile_stmt(stmt)?;
                }
            }
            Stmt::If {
                test,
                consequent,
                alternate,
            } => {
                self.compile_expr(test)?;
                if !matches!(consequent.as_ref(), Stmt::Block(_)) {
                    return Err(anyhow!("bytecode 编译器当前仅支持 if 的 block 语句体"));
                }
                if let Some(alt) = alternate {
                    if !matches!(alt.as_ref(), Stmt::Block(_)) {
                        return Err(anyhow!("bytecode 编译器当前仅支持 else 的 block 语句体"));
                    }
                }
                // 当前 bytecode 不实现跳转，直接拒绝并提示。
                return Err(anyhow!("bytecode 编译器暂不支持 if 控制流跳转"));
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.compile_expr(expr)?;
                } else {
                    self.code.push(Instruction::Push(Value::Undefined));
                }
                self.code.push(Instruction::Return);
            }
            _ => {
                return Err(anyhow!("bytecode 编译器暂未支持该语句类型: {:?}", stmt));
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> anyhow::Result<()> {
        match expr {
            Expr::Number(v) => self.code.push(Instruction::Push(Value::Number(*v))),
            Expr::String(v) => self.code.push(Instruction::Push(Value::String(v.clone()))),
            Expr::Bool(v) => self.code.push(Instruction::Push(Value::Bool(*v))),
            Expr::Null => self.code.push(Instruction::Push(Value::Null)),
            Expr::Undefined => self.code.push(Instruction::Push(Value::Undefined)),
            Expr::Identifier(name) => self.code.push(Instruction::Load(name.clone())),
            Expr::Unary { op, expr } => {
                self.compile_expr(expr)?;
                match op {
                    super::ast::UnaryOp::Neg => self.code.push(Instruction::Neg),
                    super::ast::UnaryOp::Not => self.code.push(Instruction::Not),
                }
            }
            Expr::Binary { op, left, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                let ins = match op {
                    BinaryOp::Add => Instruction::Add,
                    BinaryOp::Sub => Instruction::Sub,
                    BinaryOp::Mul => Instruction::Mul,
                    BinaryOp::Div => Instruction::Div,
                    BinaryOp::Eq => Instruction::Eq,
                    BinaryOp::NotEq => Instruction::NotEq,
                    BinaryOp::Gt => Instruction::Gt,
                    BinaryOp::Gte => Instruction::Gte,
                    BinaryOp::Lt => Instruction::Lt,
                    BinaryOp::Lte => Instruction::Lte,
                    _ => return Err(anyhow!("bytecode 暂不支持该二元运算: {:?}", op)),
                };
                self.code.push(ins);
            }
            Expr::Assignment { target, value } => {
                self.compile_expr(value)?;
                match target {
                    super::ast::AssignTarget::Identifier(name) => {
                        self.code.push(Instruction::Store(name.clone()));
                        self.code.push(Instruction::Load(name.clone()));
                    }
                    _ => return Err(anyhow!("bytecode 暂不支持成员赋值")),
                }
            }
            _ => {
                return Err(anyhow!("bytecode 编译器暂不支持该表达式: {:?}", expr));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct BytecodeVm {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
}

impl BytecodeVm {
    pub fn eval(code: &[Instruction]) -> anyhow::Result<Value> {
        let mut vm = Self::default();
        vm.run(code)
    }

    fn run(&mut self, code: &[Instruction]) -> anyhow::Result<Value> {
        let mut ip = 0usize;
        while ip < code.len() {
            match &code[ip] {
                Instruction::Push(v) => self.stack.push(v.clone()),
                Instruction::Load(name) => {
                    let v = self.vars.get(name).cloned().unwrap_or(Value::Undefined);
                    self.stack.push(v);
                }
                Instruction::Store(name) => {
                    let v = self
                        .stack
                        .pop()
                        .ok_or_else(|| anyhow!("stack underflow when store"))?;
                    self.vars.insert(name.clone(), v);
                }
                Instruction::Add => self.binary_add()?,
                Instruction::Sub => self.binary_num(|a, b| a - b)?,
                Instruction::Mul => self.binary_num(|a, b| a * b)?,
                Instruction::Div => self.binary_num(|a, b| a / b)?,
                Instruction::Eq => self.binary_cmp(|a, b| a == b)?,
                Instruction::NotEq => self.binary_cmp(|a, b| a != b)?,
                Instruction::Gt => self.binary_num_cmp(|a, b| a > b)?,
                Instruction::Gte => self.binary_num_cmp(|a, b| a >= b)?,
                Instruction::Lt => self.binary_num_cmp(|a, b| a < b)?,
                Instruction::Lte => self.binary_num_cmp(|a, b| a <= b)?,
                Instruction::Not => {
                    let v = self
                        .stack
                        .pop()
                        .ok_or_else(|| anyhow!("stack underflow when not"))?;
                    self.stack.push(Value::Bool(!v.truthy()));
                }
                Instruction::Neg => {
                    let v = self
                        .stack
                        .pop()
                        .ok_or_else(|| anyhow!("stack underflow when neg"))?;
                    self.stack.push(Value::Number(-to_number(&v)));
                }
                Instruction::Pop => {
                    let _ = self.stack.pop();
                }
                Instruction::Return => {
                    return Ok(self.stack.pop().unwrap_or(Value::Undefined));
                }
            }
            ip += 1;
        }
        Ok(Value::Undefined)
    }

    fn binary_add(&mut self) -> anyhow::Result<()> {
        let right = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on add"))?;
        let left = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on add"))?;
        match (left, right) {
            (Value::String(a), b) => self.stack.push(Value::String(format!("{a}{b}"))),
            (a, Value::String(b)) => self.stack.push(Value::String(format!("{a}{b}"))),
            (a, b) => self
                .stack
                .push(Value::Number(to_number(&a) + to_number(&b))),
        }
        Ok(())
    }

    fn binary_num<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: Fn(f64, f64) -> f64,
    {
        let right = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on numeric op"))?;
        let left = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on numeric op"))?;
        self.stack
            .push(Value::Number(f(to_number(&left), to_number(&right))));
        Ok(())
    }

    fn binary_cmp<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: Fn(Value, Value) -> bool,
    {
        let right = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on cmp"))?;
        let left = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on cmp"))?;
        self.stack.push(Value::Bool(f(left, right)));
        Ok(())
    }

    fn binary_num_cmp<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: Fn(f64, f64) -> bool,
    {
        let right = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on numeric cmp"))?;
        let left = self
            .stack
            .pop()
            .ok_or_else(|| anyhow!("stack underflow on numeric cmp"))?;
        self.stack
            .push(Value::Bool(f(to_number(&left), to_number(&right))));
        Ok(())
    }
}

pub fn eval_via_bytecode(script: &str) -> anyhow::Result<Value> {
    let program = parse_program(script)?;
    let code = BytecodeCompiler::compile_program(&program)?;
    BytecodeVm::eval(&code)
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

#[cfg(test)]
mod tests {
    use super::eval_via_bytecode;

    #[test]
    fn eval_arithmetic_by_bytecode() {
        let out = eval_via_bytecode("let a = 1 + 2 * 3; a;").unwrap();
        assert_eq!(out.to_string(), "7");
    }

    #[test]
    fn eval_assignment_by_bytecode() {
        let out = eval_via_bytecode("let x = 1; x = x + 9; x;").unwrap();
        assert_eq!(out.to_string(), "10");
    }
}
