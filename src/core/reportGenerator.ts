import { reportSuggestions } from "../content/copywriting";
import type { EndReason, StyleType, SystemObservationReport, TurnActionRecord } from "./types";

function dominantStyle(history: TurnActionRecord[]): StyleType {
  const count: Record<StyleType, number> = {
    searcher: 0,
    sprinter: 0,
    conservative: 0,
    mixed: 0
  };
  for (const action of history) {
    count[action.styleAtTurn] += 1;
  }
  return (Object.entries(count).sort((a, b) => b[1] - a[1])[0][0] as StyleType) || "mixed";
}

function adaptationStart(history: TurnActionRecord[]): number | null {
  const first = history.find((item) => item.environmentState !== "unseen");
  return first ? first.turn : null;
}

function pressureReasons(history: TurnActionRecord[], endReason: EndReason): string[] {
  const latest = history.slice(-4);
  const records = latest.map((item) => `T${item.turn}:${item.actionId}`);

  if (endReason === "success") {
    return [
      `你在后段采用 ${records.join(" → ")} 的节奏完成突围。`,
      "系统虽已响应你的风格，但你通过切换动作保持主动。"
    ];
  }

  if (endReason === "riskOverflow") {
    return [`末段动作序列 ${records.join(" → ")} 导致风险持续堆积。`];
  }
  if (endReason === "staminaDepleted") {
    return [`末段动作序列 ${records.join(" → ")} 造成体力透支。`];
  }
  return [`末段动作序列 ${records.join(" → ")} 推进不足，错过撤离窗口。`];
}

export function generateSystemObservationReport(params: {
  history: TurnActionRecord[];
  finalStyle: StyleType;
  endReason: EndReason;
}): SystemObservationReport {
  const { history, endReason, finalStyle } = params;
  const mainStyle = dominantStyle(history);
  return {
    mainStyle,
    adaptationStartTurn: adaptationStart(history),
    pressureReasons: pressureReasons(history, endReason),
    suggestions: reportSuggestions(finalStyle, endReason).slice(0, 3)
  };
}

