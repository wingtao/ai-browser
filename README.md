# Mist Breakout（迷雾突围）- 微信小游戏 MVP

《迷雾突围》是一个 3~5 分钟短局制轻策略微信小游戏原型。  
玩家在 10 回合内管理 **体力 / 补给 / 风险 / 进度**，并在环境逐步适应玩家风格前完成突围。

## 核心特性

- 10 回合撤离循环
- 6 个行动（推进/补给/保守）
- 风格识别（搜索型/冲刺型/保守型/混合型）
- 动态调权（软反制，不改基础规则）
- 局内战术画像 + 局后系统观察报告
- 非极简吸引力视觉（迷雾层、发光 HUD、扫描感）

## 技术栈

- 微信小游戏原生 API
- TypeScript
- Canvas 2D
- esbuild
- Vitest

## 目录结构

```text
docs/                    # PRD细化、UI设计、视觉方向、技术设计
scripts/                 # 平衡模拟脚本
src/
  app/                   # 应用编排
  core/                  # 规则引擎、识别器、调权器
  content/               # 事件模板与文案
  ui/                    # 场景渲染与组件
  telemetry/             # 埋点追踪
tests/                   # 单元测试
```

## 本地开发

```bash
npm install
npm run build
npm run test
npm run simulate
```

构建后产物位于 `dist/main.js`，小游戏入口 `game.js` 会自动加载该文件。

## 导入微信开发者工具

1. 打开微信开发者工具，选择“小游戏”项目。
2. 选择本仓库目录 `/workspace`。
3. AppID 可先使用测试/游客模式（`project.config.json` 已配置 `touristappid`）。
4. 执行 `npm run build` 后即可在模拟器中运行。

## 说明

- 当前为 MVP 原型，重点验证“环境适应型轻策略”体验，不含联网排行、养成和多章节。
- 更多规则细节请查看 `docs/prd-mvp-detailed.md`。