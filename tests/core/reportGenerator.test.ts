import { describe, expect, it } from "vitest";
import { generateSystemObservationReport } from "../../src/core/reportGenerator";

describe("reportGenerator", () => {
  it("能够生成局后报告核心字段", () => {
    const report = generateSystemObservationReport({
      history: [
        { turn: 1, actionId: "search", styleAtTurn: "mixed", environmentState: "unseen" },
        { turn: 2, actionId: "forcedSearch", styleAtTurn: "searcher", environmentState: "unseen" },
        { turn: 3, actionId: "search", styleAtTurn: "searcher", environmentState: "adapting" },
        { turn: 4, actionId: "advance", styleAtTurn: "searcher", environmentState: "adapting" }
      ],
      finalStyle: "searcher",
      endReason: "riskOverflow"
    });

    expect(report.mainStyle).toBe("searcher");
    expect(report.adaptationStartTurn).toBe(3);
    expect(report.pressureReasons.length).toBeGreaterThan(0);
    expect(report.suggestions.length).toBeGreaterThan(0);
  });
});

