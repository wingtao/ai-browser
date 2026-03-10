# JS 引擎设计（v1）

## 目标

实现自研 JS 引擎最小子集，支持：
- 变量声明（let/const/var）
- 函数与闭包
- if/while/for 控制流
- break / continue
- 对象字面量与成员访问
- new / this（基础构造语义）
- try/catch/finally 与 throw（简化异常模型）
- 函数调用与 return

## 组件

## 1. Lexer（`engine/js/lexer.rs`）
- 输入源码字符串，输出 Token 序列。
- 支持：
  - 关键字（function/return/if/while/for/new/this/try/catch/finally/throw 等）
  - 字面量（number/string/bool/null/undefined）
  - 运算符（+ - * / == != > >= < <= && ||）
  - 分隔符（括号、花括号、逗号、分号、冒号、点）

## 2. Parser（`engine/js/parser.rs`）
- 递归下降解析器。
- 输出 AST（`engine/js/ast.rs`）。
- 当前语法覆盖：
  - 语句：变量、函数声明、return、if、while、for、break/continue、try/catch/finally、throw、block、表达式语句
  - 表达式：赋值、逻辑运算、比较、算术、一元、成员访问、调用、对象字面量、new、this

## 3. Runtime（`engine/js/runtime/*`）
- `scope.rs`: 作用域链（Environment）
- `object.rs`: 对象与原型引用（基础）
- `value.rs`: 值系统 + 函数值 + 原生函数值
- `gc.rs`: GC 预留接口（当前占位）

## 4. Interpreter（`engine/js/vm.rs`）
- 解释执行 AST。
- 支持闭包：函数值持有定义时环境（closure env）。
- 支持原生函数注册：`define_native_function`。
- 支持构造调用：`new Fn(...)`，并创建实例对象原型链（基础）。
- 内置 `print`，并支持安装 DOM 桥接 API。

## 已有验证用例

- 算术优先级
- 闭包调用
- while/for + break/continue
- try/catch/finally 与 throw
- new + this 构造实例
- DOM 桥接调用（`dom_set_text` / `dom_get_text` / `dom_get_all_text`）

## 后续扩展

- bytecode 编译与 VM 执行
- mark-sweep GC 完整实现
- 完整 this 绑定规则（方法调用、call/apply/bind）
- 更完整 prototype 重写语义与继承链行为
