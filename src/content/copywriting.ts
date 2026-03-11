import type { EndReason, EnvironmentState, StyleType } from "../core/types";

const STYLE_LABEL: Record<StyleType, string> = {
  searcher: "搜索型",
  sprinter: "冲刺型",
  conservative: "保守型",
  mixed: "混合型"
};

const ENV_LABEL: Record<EnvironmentState, string> = {
  unseen: "未明显识别",
  adapting: "适应中",
  responding: "已响应"
};

const STYLE_ENV_COPY: Record<StyleType, Record<EnvironmentState, string>> = {
  searcher: {
    unseen: "迷雾开始记录你的补给偏好。",
    adapting: "你对资源的偏好已被捕捉，压力在抬升。",
    responding: "搜索路径被看懂，补给诱惑与风险同步放大。"
  },
  sprinter: {
    unseen: "你推进迅速，雾层尚未形成回压。",
    adapting: "你推进过快，前方压力正在提前聚集。",
    responding: "冲压节奏已被识别，鲁莽代价被持续放大。"
  },
  conservative: {
    unseen: "你保持安全节奏，迷雾正在观察。",
    adapting: "你保持了安全，但撤离窗口正在缩小。",
    responding: "保守路径被识别，时间压力正在追上你。"
  },
  mixed: {
    unseen: "你的节奏尚未形成固定模式。",
    adapting: "你在切换策略，迷雾暂时难以锁定。",
    responding: "你持续变换节奏，系统回应保持中性。"
  }
};

const FAILURE_SUGGESTIONS: Record<Exclude<EndReason, "success">, string[]> = {
  staminaDepleted: [
    "中段减少连续冲刺，至少保留一次休整空间。",
    "体力低于 2 时优先选择低消耗推进。"
  ],
  riskOverflow: [
    "风险超过 7 后，优先考虑绕路或休整削峰。",
    "避免连续搜索/强搜导致压力叠层。"
  ],
  evacuationTimeout: [
    "第 6 回合前至少推进到 8，后段才有翻盘空间。",
    "中后段提高前进/冲刺比例，避免节奏过慢。"
  ]
};

const STYLE_SUGGESTIONS: Record<StyleType, string[]> = {
  searcher: ["在第 5 回合后减少搜索频次，改用推进动作收尾。"],
  sprinter: ["每两次高压推进后插入一次保守动作降低崩盘风险。"],
  conservative: ["第 7 回合后提高冲刺权重，避免拖入超时失败。"],
  mixed: ["继续保持节奏切换，但确保后段有明确推进计划。"]
};

export function styleLabel(style: StyleType): string {
  return STYLE_LABEL[style];
}

export function environmentLabel(state: EnvironmentState): string {
  return ENV_LABEL[state];
}

export function tacticalPortraitText(style: StyleType, state: EnvironmentState): string {
  return STYLE_ENV_COPY[style][state];
}

export function reportSuggestions(style: StyleType, endReason: EndReason): string[] {
  if (endReason === "success") {
    return [
      "你已经学会读局势。下一局尝试更早切换节奏，压缩波动。",
      ...STYLE_SUGGESTIONS[style]
    ];
  }
  return [...FAILURE_SUGGESTIONS[endReason], ...STYLE_SUGGESTIONS[style]];
}

