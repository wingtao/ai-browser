# AGENTS.md

## Repository overview

This repository contains **two independent products** on separate feature branches (the `main` branch has only a placeholder README):

| Product | Branch | Language | Description |
|---|---|---|---|
| AI Browser | `cursor/browser-ai-application-a582` | Rust | Experimental browser with custom HTML/DOM/CSS/JS engine and AI layer |
| Mist Breakout (迷雾突围) | `cursor/mist-breakout-adaptive-system-60bf` | TypeScript | WeChat Mini Game prototype — light-strategy game with adaptive difficulty |

## Cursor Cloud specific instructions

### Working with branches

All code lives on feature branches. The `main` branch is essentially empty. When working on either product, you must check out the relevant branch or use `git worktree` to set up a parallel working directory.

### AI Browser (Rust)

- **Branch:** `cursor/browser-ai-application-a582`
- **Build:** `cargo build`
- **Test:** `cargo test` (56 tests: 49 unit + 7 integration)
- **Lint:** `cargo clippy` (1 non-blocking warning about `derivable_impls`), `cargo fmt --check`
- **Run examples:** see `README.md` on that branch for all CLI commands (`load`, `js`, `js-bc`, `js-dom`, `history-*`, `bookmark-*`, `summarize`, `ask`, `window`)
- SQLite is bundled via `rusqlite` with the `bundled` feature — no external database needed.
- AI features (`summarize`, `ask`) require env vars `AI_BROWSER_API_KEY`, `AI_BROWSER_BASE_URL`, `AI_BROWSER_MODEL` — these are optional for core functionality.
- The `window` command requires a display server; it will fail in headless environments without `DISPLAY` set.

### Mist Breakout (TypeScript)

- **Branch:** `cursor/mist-breakout-adaptive-system-60bf`
- **Install:** `npm install`
- **Build:** `npm run build` (output: `dist/main.js`)
- **Test:** `npm run test` (Vitest, 13 tests across 5 files)
- **Type check:** `npx tsc --noEmit`
- **Simulate:** `npm run simulate` (runs balance simulation via `tsx`)
- This is a WeChat Mini Game — full visual testing requires WeChat Developer Tools (desktop GUI, not available in CI). Use `npm run test` and `npm run simulate` for automated verification.
