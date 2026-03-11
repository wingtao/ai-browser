import type { ActionId, StyleType } from "./types";

const STYLE_SCORE_MAP: Record<ActionId, Exclude<StyleType, "mixed">> = {
  search: "searcher",
  forcedSearch: "searcher",
  advance: "sprinter",
  sprint: "sprinter",
  rest: "conservative",
  detour: "conservative"
};

export function classifyStyle(history: ActionId[]): StyleType {
  if (history.length === 0) {
    return "mixed";
  }

  const score = {
    searcher: 0,
    sprinter: 0,
    conservative: 0
  };

  for (const actionId of history) {
    const mapped = STYLE_SCORE_MAP[actionId];
    score[mapped] += 1;
  }

  const sorted = Object.entries(score).sort((a, b) => b[1] - a[1]);
  const [topKey, topValue] = sorted[0] as [Exclude<StyleType, "mixed">, number];
  const secondValue = sorted[1][1];

  if (topValue < 2) {
    return "mixed";
  }

  if (topValue - secondValue <= 0) {
    return "mixed";
  }

  return topKey;
}

