# AGENTS.md

## Repository overview

AI Browser 代码在 `cursor/browser-ai-application-a582` 及衍生分支（如 `cursor/repo-optimization-b2d9`）上维护；`main` 仅为占位 README，**无需合并到 main**。

## Cursor Cloud 说明

### 构建与验证

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

- SQLite 通过 `rusqlite` 的 `bundled` 特性内置，无需外部数据库。
- AI 命令（`summarize`、`ask`）可选环境变量：`AI_BROWSER_API_KEY`、`AI_BROWSER_BASE_URL`、`AI_BROWSER_MODEL`。
- `window` 需要图形显示（`DISPLAY`）；无头环境请用 CLI 子命令验证。

### 常用命令

见分支 `README.md`：`load`、`js`、`js-bc`、`js-dom`、`history-*`、`bookmark-*`、`summarize`、`ask`、`window`。
