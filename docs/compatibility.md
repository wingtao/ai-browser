# 兼容性与运行约束

## 当前实现定位

本项目是“自研内核 MVP”阶段，目标是验证核心链路而非完整 Web 标准兼容。

### 页面兼容性（当前）
- ✅ 静态 HTML 页面
- ✅ 简化 CSS 与文本布局
- ✅ 内联脚本（受 JS 子集与内建 API 限制）
- ⚠️ 不支持完整浏览器特性（复杂 CSS/完整 DOM API/完整 JS 语义）

## JS 兼容性（当前）

### 已支持
- 变量：`let/const/var`
- 控制流：`if/while/for/break/continue`
- 函数与闭包
- 对象字面量、成员访问与赋值
- `new` / `this` / `prototype` 基础行为
- `try/catch/finally` + `throw`
- 内建 API：
  - `setTimeout/clearTimeout/runTasks`
  - `addEventListener/dispatchEvent`
  - `dom_get_text/dom_get_all_text/dom_set_text`
  - `dom_add_event_listener/dom_dispatch_event`

### 未支持（示例）
- 箭头函数、类语法、模块系统
- Promise / async/await
- 完整 BOM/DOM API（createElement、querySelectorAll 全选择器等）

## GUI 运行约束（Linux）

窗口壳基于 `winit`，需要可用显示服务器：
- X11 或 Wayland 会话
- 对应系统库（如 `libxkbcommon-x11`）

在纯无头环境（无 DISPLAY）会出现：
- `Failed to open connection to X server`

可在具备图形环境的机器执行：
```bash
cargo run -- window http://example.com
```
