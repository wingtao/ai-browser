# 《迷雾突围》技术设计（MVP）

## 1. 技术栈

- 平台：微信小游戏（原生 API）
- 语言：TypeScript
- 渲染：Canvas 2D
- 打包：esbuild
- 测试：Vitest

---

## 2. 模块划分

## 2.1 core（规则层）

- `actions.ts`：6 行动定义与可用性
- `gameEngine.ts`：回合循环、结算、状态推进
- `styleClassifier.ts`：风格识别
- `adaptiveDirector.ts`：动态调权
- `eventSystem.ts`：事件选择
- `reportGenerator.ts`：局后观察报告
- `stateMachine.ts`：胜负判定

## 2.2 content（内容层）

- `eventTemplates.ts`：12~16 事件模板
- `copywriting.ts`：战术画像文案与建议模板

## 2.3 ui（表现层）

- `renderer.ts`：背景雾层、面板、发光文本等基础绘制
- `renderer.ts` 支持高/低画质切换（雾层粒子数、扫描线开关）
- `scenes/*`：Home/Battle/Result 场景渲染
- `components/Button.ts`：统一按钮组件

## 2.4 app（编排层）

- `GameApp.ts`：场景切换、输入绑定、帧循环、引擎对接

## 2.5 telemetry（埋点）

- `tracker.ts`：事件记录（可落微信本地存储）
- 结算页支持主观可读性反馈事件：`readability_feedback`

---

## 3. 核心数据流

1. 进入首页 -> `enter_home`
2. 开始对局 -> `engine.startRun`
3. 每回合：
   - 读取候选行动
   - 玩家点击行动
   - 引擎结算动作 + 事件
   - 更新资源/风格/环境状态
   - 推送埋点
4. 终局：
   - 生成 `SystemObservationReport`
   - 渲染结算页
   - 支持再来一局

---

## 4. 算法设计

## 4.1 行动池抽样

- 输入：可用行动 + 调权结果
- 输出：3 个不重复行动
- 实现：加权随机 + 逐次移除（无放回）

## 4.2 风格识别

- 窗口：最近 4 次
- 周期：每 2 回合
- 规则：最高分显著领先则归类，否则混合型

## 4.3 动态调权

- 基于风格对行动/事件权重软偏移
- 权重乘子限制 `[0.85, 1.15]`
- 首局系数 0.6
- 第 8~10 回合追加推进压力

---

## 5. 可扩展点

1. 事件模板可追加，不改引擎结构
2. 风格识别可升级为概率分布输出
3. 可接入远程参数表做热调优（后续版本）
4. 可增加视觉特效开关策略（机型分级）

---

## 6. 测试策略

1. `gameEngine`：资源结算、终局判定
2. `styleClassifier`：标签识别准确性
3. `adaptiveDirector`：调权边界不越界
4. `eventSystem`：事件筛选与风格响应
5. `reportGenerator`：报告可解释字段完整
6. `simulateBalance`：多局模拟输出胜率与失败分布

