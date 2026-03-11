import { describe, expect, it } from "vitest";
import { buildAdaptiveWeights, evaluateEnvironmentState } from "../../src/core/adaptiveDirector";

describe("adaptiveDirector", () => {
  it("调权范围不会越界", () => {
    const weights = buildAdaptiveWeights({
      style: "searcher",
      turn: 9,
      isFirstRun: false
    });
    Object.values(weights.actionWeight).forEach((weight) => {
      expect(weight).toBeGreaterThanOrEqual(0.85);
      expect(weight).toBeLessThanOrEqual(1.15);
    });
    Object.values(weights.eventWeight).forEach((weight) => {
      expect(weight).toBeGreaterThanOrEqual(0.85);
      expect(weight).toBeLessThanOrEqual(1.15);
    });
  });

  it("首局保护会降低调权强度", () => {
    const firstRun = buildAdaptiveWeights({
      style: "sprinter",
      turn: 6,
      isFirstRun: true
    });
    const normalRun = buildAdaptiveWeights({
      style: "sprinter",
      turn: 6,
      isFirstRun: false
    });
    expect(Math.abs(firstRun.actionWeight.rest - 1)).toBeLessThan(Math.abs(normalRun.actionWeight.rest - 1));
  });

  it("环境状态按阶段变化", () => {
    expect(evaluateEnvironmentState("mixed", 5)).toBe("unseen");
    expect(evaluateEnvironmentState("searcher", 5)).toBe("adapting");
    expect(evaluateEnvironmentState("searcher", 9)).toBe("responding");
  });
});

