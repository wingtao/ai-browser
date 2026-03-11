import type { ActionDefinition, ActionId, Resources } from "./types";

export const ACTION_DEFINITIONS: Record<ActionId, ActionDefinition> = {
  advance: {
    id: "advance",
    name: "前进",
    category: "progress",
    description: "进度 +2，体力 -1，风险 +1",
    baseEffect: { progress: 2, stamina: -1, risk: 1 },
    canUse: (resources: Resources) => resources.stamina >= 1
  },
  search: {
    id: "search",
    name: "搜索",
    category: "supply",
    description: "补给 +2，25%概率额外风险 +2",
    baseEffect: { supply: 2 },
    canUse: () => true,
    randomEffect: (roll: number) => (roll < 0.25 ? { risk: 2 } : {})
  },
  rest: {
    id: "rest",
    name: "休整",
    category: "conservative",
    description: "体力 +2，补给 -1",
    baseEffect: { stamina: 2, supply: -1 },
    canUse: (resources: Resources) => resources.supply >= 1
  },
  detour: {
    id: "detour",
    name: "绕路",
    category: "conservative",
    description: "进度 +1，体力 -1，风险 -1",
    baseEffect: { progress: 1, stamina: -1, risk: -1 },
    canUse: (resources: Resources) => resources.stamina >= 1
  },
  sprint: {
    id: "sprint",
    name: "冲刺",
    category: "progress",
    description: "进度 +3，体力 -2，风险 +2",
    baseEffect: { progress: 3, stamina: -2, risk: 2 },
    canUse: (resources: Resources) => resources.stamina >= 2
  },
  forcedSearch: {
    id: "forcedSearch",
    name: "强搜",
    category: "supply",
    description: "补给 +3，风险 +2，10%概率额外体力 -1",
    baseEffect: { supply: 3, risk: 2 },
    canUse: () => true,
    randomEffect: (roll: number) => (roll < 0.1 ? { stamina: -1 } : {})
  }
};

export const ACTION_IDS: ActionId[] = Object.keys(ACTION_DEFINITIONS) as ActionId[];

