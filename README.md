# AI Browser（自研内核 + AI 应用层）

这是一个 **不复用 Chromium/WebKit/Gecko/Electron 内核** 的实验型浏览器项目，使用 Rust 从零构建最小可用链路，并叠加 AI 能力。

> 当前阶段：MVP 内核骨架 + HTML/DOM 解析 + JS 子集解释执行 + 简化 WebAPI 桥接 + AI 应用层基础模块。

## 已实现能力

### 浏览器内核（基础）
- URL 解析（`engine/net/url`）
- HTTP 拉取（`engine/net/http_client`）
- HTML tokenizer + parser（`engine/dom`）
- DOM 文本提取与标题提取
- CSS 子集解析（基础规则解析）
- 简化块布局与 display list 构建

### JS 引擎（v1 子集）
- Lexer：关键字/标识符/数字/字符串/操作符
- Parser：变量声明、函数声明、if/while/for、break/continue、try/catch/finally、throw、new/this
- Interpreter：作用域链、闭包、对象属性读写、构造调用（new）、原生函数调用

### WebAPI（简化）
- 事件循环（microtask/macrotask 简化模型）
- DOM 绑定：
  - `querySelector` / `querySelectorAll`
  - `textContent` / `setTextContent`
  - `innerHTML`
  - `createElement` / `appendChild` / `removeChild`
  - 兼容别名：`dom_get_text` / `dom_get_all_text` / `dom_set_text`
- 定时器：可取消 `setTimeout/clearTimeout` 基础调度模型
- console 参数格式化支持

### AI 应用层（基础模块）
- 智能地址栏意图识别（导航/搜索/命令）
- 页面上下文抽取
- 页面总结/问答调用器（OpenAI-compatible API）
- 标签页分组建议算法（按域名聚类）

## 项目结构

```text
src/
  app/
    ai/           # AI 应用层
    shell/        # CLI 壳与最小窗口启动
  engine/
    net/          # URL/HTTP
    dom/          # HTML/DOM
    css/          # CSS 子集
    layout/       # 布局
    paint/        # 绘制指令
    js/           # 自研 JS 引擎
    webapi/       # DOM/BOM 桥接与事件循环
tests/            # 集成测试
```

## 运行方式

### 1) 编译与测试
```bash
cargo test
```

### 2) 拉取网页并做文本预览
```bash
cargo run -- load https://example.com
```

### 3) 运行 JS 脚本
```bash
cargo run -- js "let a = 1 + 2 * 3; a;"
```

### 3.1) 运行 JS 字节码路径（实验性子集）
```bash
cargo run -- js-bc "let a = 1 + 2 * 3; a;"
```

### 3.2) 在 DOM 上下文运行 JS
```bash
cargo run -- js-dom "<html><body><div><p>A</p></div></body></html>" "let el=createElement('span','B'); appendChild('div',el); innerHTML('div');"
```

JS 运行时额外支持（简化）：
- `setTimeout(fn, delay)` / `clearTimeout(id)` / `runTasks()`
- `addEventListener` / `dispatchEvent`（全局事件）
- 在 DOM 上下文中：`dom_add_event_listener` / `dom_dispatch_event`（简化冒泡）

### 4) 启动最小窗口循环
```bash
cargo run -- window
```

窗口模式快捷键：
- `N` 新建标签页（about:blank）
- `O` 打开示例页（http://example.com）
- `R` 刷新
- `B` 后退
- `F` 前进
- `W` 关闭当前标签
- `1-9` 切换标签
- `Q` 退出

### 5) 使用 AI 总结网页
```bash
cargo run -- summarize http://example.com
```

### 6) 基于网页上下文问答
```bash
cargo run -- ask http://example.com "这个页面讲了什么？"
```

### 7) 历史与书签（本地 SQLite）
```bash
# 可选：指定数据库路径
export AI_BROWSER_DB=./ai_browser.db

cargo run -- history-add http://example.com "Example Domain"
cargo run -- history-list 20
cargo run -- history-clear

cargo run -- bookmark-add http://example.com "Example Domain"
cargo run -- bookmark-list
cargo run -- bookmark-remove http://example.com
```

## AI 能力配置

设置以下环境变量后可接入兼容 Chat Completions 的模型服务：

```bash
export AI_BROWSER_API_KEY=your_key
export AI_BROWSER_BASE_URL=https://api.openai.com/v1
export AI_BROWSER_MODEL=gpt-4o-mini
```

在企业代理或自签证书环境下，如 HTTPS 报证书错误，可临时启用：

```bash
export AI_BROWSER_INSECURE_TLS=1
```

## 后续路线

- 完善 DOM API（更完整选择器、节点引用与属性系统）
- 增加多标签导航壳与持久化数据层（历史/书签）
- 引入更完整的 CSS 布局模型和绘制管线

## 设计文档

- `docs/architecture.md`
- `docs/js-engine.md`
- `docs/compatibility.md`