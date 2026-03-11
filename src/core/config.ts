import type { ActionId, Resources } from "./types";

export const GAME_CONFIG = {
  maxTurns: 10,
  targetProgress: 15,
  riskFailThreshold: 10,
  initialResources: {
    stamina: 6,
    supply: 3,
    risk: 0,
    progress: 0
  } as Resources,
  resourceClamp: {
    stamina: { min: -5, max: 12 },
    supply: { min: 0, max: 12 },
    risk: { min: 0, max: 15 },
    progress: { min: 0, max: 20 }
  },
  actionPoolSize: 3,
  styleUpdateInterval: 2,
  styleWindowSize: 4,
  firstRunProtectionFactor: 0.6,
  adaptiveRange: {
    min: 0.85,
    max: 1.15
  }
};

export const BASE_ACTION_WEIGHTS: Record<ActionId, number> = {
  advance: 1,
  search: 1,
  rest: 1,
  detour: 1,
  sprint: 1,
  forcedSearch: 1
};

