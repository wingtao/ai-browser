import { describe, expect, it } from "vitest";
import { GameEngine } from "../../src/core/gameEngine";
import { SeededRandomProvider } from "../../src/core/random";

describe("gameEngine", () => {
  it("开局可生成3个行动选项", () => {
    const engine = new GameEngine({ random: new SeededRandomProvider(7) });
    const state = engine.startRun(1);
    expect(state.turn).toBe(1);
    expect(state.options.length).toBe(3);
    expect(state.ended).toBe(false);
  });

  it("每次行动后会推进回合或结束", () => {
    const engine = new GameEngine({ random: new SeededRandomProvider(11) });
    let state = engine.startRun(1);
    const firstAction = state.options[0].id;
    state = engine.pickAction(firstAction);
    if (!state.ended) {
      expect(state.turn).toBe(2);
      expect(state.options.length).toBeGreaterThan(0);
    } else {
      expect(state.endReason).toBeDefined();
    }
  });

  it("对局最多进行10回合后结束", () => {
    const engine = new GameEngine({ random: new SeededRandomProvider(2026) });
    let state = engine.startRun(1);
    let guard = 0;
    while (!state.ended && guard < 20) {
      const actionId = state.options[0].id;
      state = engine.pickAction(actionId);
      guard += 1;
    }
    expect(state.ended).toBe(true);
    expect(guard).toBeLessThanOrEqual(10);
    expect(state.endReason).toBeDefined();
  });
});

