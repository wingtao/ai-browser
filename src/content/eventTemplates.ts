import type { EventTemplate } from "../core/types";

export const EVENT_TEMPLATES: EventTemplate[] = [
  {
    id: "generic-echo-1",
    type: "generic",
    text: "雾层回荡出微弱回声，你判断路线更清晰了。",
    effect: { progress: 1 }
  },
  {
    id: "generic-echo-2",
    type: "generic",
    text: "脚下地面湿滑，你被迫放慢半步。",
    effect: {}
  },
  {
    id: "generic-echo-3",
    type: "generic",
    text: "补给箱残片散落，仍有可用物资。",
    effect: { supply: 1 }
  },
  {
    id: "style-search-1",
    type: "styleResponse",
    styleHint: "searcher",
    text: "迷雾对你的搜寻路径做出回响，周围压迫感加深。",
    effect: { risk: 1 }
  },
  {
    id: "style-sprint-1",
    type: "styleResponse",
    styleHint: "sprinter",
    text: "你推进过快，前方地形提前塌陷。",
    effect: { risk: 1 }
  },
  {
    id: "style-conservative-1",
    type: "styleResponse",
    styleHint: "conservative",
    text: "你保持谨慎，但撤离窗口的读数正在变窄。",
    effect: { progress: -1 }
  },
  {
    id: "pressure-1",
    type: "highPressure",
    minTurn: 6,
    text: "雾压提升，路径标记正在消失。",
    effect: { risk: 1, progress: 1 }
  },
  {
    id: "pressure-2",
    type: "highPressure",
    minTurn: 8,
    text: "后方出现异常噪波，你只能硬顶前压。",
    effect: { progress: 1 }
  },
  {
    id: "pressure-3",
    type: "highPressure",
    minTurn: 8,
    text: "撤离倒计时跳变，局势突然收紧。",
    effect: { risk: 1, progress: 1 }
  },
  {
    id: "comeback-1",
    type: "comeback",
    minTurn: 8,
    text: "雾间裂缝短暂打开，你抓住了推进窗口。",
    effect: { progress: 1, risk: -1 }
  },
  {
    id: "comeback-2",
    type: "comeback",
    minTurn: 7,
    text: "你发现一处临时补给点，呼吸稳定了下来。",
    effect: { supply: 1, stamina: 1 }
  },
  {
    id: "generic-echo-4",
    type: "generic",
    text: "短波雷达捕捉到边缘信号，方向更明确。",
    effect: { progress: 1 }
  },
  {
    id: "style-search-2",
    type: "styleResponse",
    styleHint: "searcher",
    minTurn: 5,
    text: "你对资源的偏好被迷雾捕捉，陷阱概率上升。",
    effect: { risk: 1, supply: 1 }
  },
  {
    id: "style-sprint-2",
    type: "styleResponse",
    styleHint: "sprinter",
    minTurn: 5,
    text: "你连续冲压，体能消耗曲线被放大。",
    effect: {}
  },
  {
    id: "style-conservative-2",
    type: "styleResponse",
    styleHint: "conservative",
    minTurn: 5,
    text: "你选择稳妥路径，迷雾在前方堆起更多距离。",
    effect: { risk: -1 }
  },
  {
    id: "comeback-3",
    type: "comeback",
    minTurn: 9,
    text: "出口信标闪烁，你抓住最后窗口。",
    effect: { progress: 1 }
  }
];

