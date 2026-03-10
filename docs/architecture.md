# 架构说明

## 总览

系统按“内核层 + 应用层”划分：

1. **内核层（engine）**
   - `net`: URL 与网络请求
   - `dom`: HTML 解析与 DOM 树
   - `css`: CSS 子集解析
   - `layout`: 盒模型布局（简化）
   - `paint`: display list
   - `js`: JS 词法/语法/执行
   - `webapi`: DOM 桥接、事件循环

2. **应用层（app）**
   - `shell`: 命令行入口与最小窗口壳
   - `ai`: 页面 AI 功能（总结、问答、智能地址栏、标签整理）

## 请求到渲染链路（当前实现）

1. 地址输入 -> URL 解析（`parse_url`）
2. 网络拉取 HTML（`HttpClient::get_text`）
3. HTML -> token -> DOM（`tokenize_html` / `parse_html`）
4. DOM -> LayoutBox（`block_layout`）
5. LayoutBox -> DrawCommand（`build_display_list`）

## JS 与 DOM 桥接链路

1. JS 脚本由 `Interpreter` 执行。
2. `install_dom_apis` 注册原生函数：
   - `dom_get_text(selector)`
   - `dom_set_text(selector, text)`
3. 原生函数通过 `JsDocumentBinding` 操作 DOM，产生可见更新。

## 设计决策

- 先实现 AST 解释执行，后续再演进到 bytecode VM。
- DOM API 先做最小可用接口，保证链路贯通。
- 事件循环先实现 microtask/macrotask 次序语义，后续再补时间轮/定时器精度。
