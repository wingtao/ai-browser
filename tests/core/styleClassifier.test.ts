import { describe, expect, it } from "vitest";
import { classifyStyle } from "../../src/core/styleClassifier";

describe("styleClassifier", () => {
  it("识别搜索型", () => {
    const style = classifyStyle(["search", "forcedSearch", "search", "advance"]);
    expect(style).toBe("searcher");
  });

  it("识别冲刺型", () => {
    const style = classifyStyle(["advance", "sprint", "advance", "search"]);
    expect(style).toBe("sprinter");
  });

  it("识别保守型", () => {
    const style = classifyStyle(["rest", "detour", "rest", "search"]);
    expect(style).toBe("conservative");
  });

  it("分数接近时识别混合型", () => {
    const style = classifyStyle(["search", "forcedSearch", "advance", "sprint"]);
    expect(style).toBe("mixed");
  });
});

