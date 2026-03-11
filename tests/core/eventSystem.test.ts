import { describe, expect, it } from "vitest";
import { pickEvent } from "../../src/core/eventSystem";
import type { AdaptiveWeights, EventTemplate } from "../../src/core/types";

const randomZero = {
  next: () => 0
};

const adaptive: AdaptiveWeights = {
  actionWeight: {
    advance: 1,
    search: 1,
    rest: 1,
    detour: 1,
    sprint: 1,
    forcedSearch: 1
  },
  eventWeight: {
    generic: 1,
    styleResponse: 1.15,
    highPressure: 1,
    comeback: 1
  },
  valueBias: {
    riskDelta: 0,
    progressDelta: 0,
    supplyDelta: 0,
    staminaDelta: 0
  }
};

describe("eventSystem", () => {
  it("会过滤未达到回合的事件", () => {
    const events: EventTemplate[] = [
      { id: "e1", type: "generic", text: "a", effect: {} },
      { id: "e2", type: "highPressure", text: "b", effect: {}, minTurn: 8 }
    ];
    const picked = pickEvent({
      events,
      turn: 2,
      style: "mixed",
      adaptive,
      random: randomZero
    });
    expect(picked.id).toBe("e1");
  });

  it("风格匹配时优先响应事件", () => {
    const events: EventTemplate[] = [
      { id: "generic", type: "generic", text: "a", effect: {} },
      { id: "style", type: "styleResponse", text: "b", effect: {}, styleHint: "searcher" }
    ];
    const picked = pickEvent({
      events,
      turn: 6,
      style: "searcher",
      adaptive,
      random: { next: () => 0.99 }
    });
    expect(["generic", "style"]).toContain(picked.id);
  });
});

