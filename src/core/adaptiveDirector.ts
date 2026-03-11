import { BASE_ACTION_WEIGHTS, GAME_CONFIG } from "./config";
import type {
  ActionId,
  AdaptiveWeights,
  EnvironmentState,
  EventType,
  StyleType
} from "./types";

function clampAdaptiveMultiplier(multiplier: number): number {
  return Math.max(GAME_CONFIG.adaptiveRange.min, Math.min(GAME_CONFIG.adaptiveRange.max, multiplier));
}

function createNeutralWeights(): AdaptiveWeights {
  return {
    actionWeight: { ...BASE_ACTION_WEIGHTS },
    eventWeight: {
      generic: 1,
      styleResponse: 1,
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
}

function offsetAction(
  weights: AdaptiveWeights,
  actionId: ActionId,
  deltaPercent: number
): void {
  const next = 1 + deltaPercent;
  weights.actionWeight[actionId] = clampAdaptiveMultiplier(weights.actionWeight[actionId] * next);
}

function offsetEvent(
  weights: AdaptiveWeights,
  eventType: EventType,
  deltaPercent: number
): void {
  const next = 1 + deltaPercent;
  const value = clampAdaptiveMultiplier(weights.eventWeight[eventType] * next);
  weights.eventWeight[eventType] = value;
}

export function evaluateEnvironmentState(style: StyleType, turn: number): EnvironmentState {
  if (turn <= 3 || style === "mixed") {
    return "unseen";
  }
  if (turn <= 7) {
    return "adapting";
  }
  return "responding";
}

export function buildAdaptiveWeights(params: {
  style: StyleType;
  turn: number;
  isFirstRun: boolean;
}): AdaptiveWeights {
  const { style, turn, isFirstRun } = params;
  const weights = createNeutralWeights();

  const protection = isFirstRun ? GAME_CONFIG.firstRunProtectionFactor : 1;
  const turnPressureBoost = turn >= 8 ? 0.08 : 0;

  switch (style) {
    case "searcher":
      offsetAction(weights, "search", -0.15 * protection);
      offsetAction(weights, "forcedSearch", -0.12 * protection);
      offsetAction(weights, "advance", 0.12 * protection);
      offsetAction(weights, "sprint", 0.1 * protection);
      offsetEvent(weights, "styleResponse", 0.12 * protection);
      break;
    case "sprinter":
      offsetAction(weights, "rest", 0.12 * protection);
      offsetAction(weights, "detour", 0.12 * protection);
      offsetAction(weights, "sprint", -0.1 * protection);
      offsetEvent(weights, "highPressure", 0.1 * protection);
      break;
    case "conservative":
      offsetAction(weights, "rest", -0.12 * protection);
      offsetAction(weights, "detour", -0.1 * protection);
      offsetAction(weights, "advance", 0.12 * protection);
      offsetAction(weights, "sprint", 0.1 * protection);
      offsetEvent(weights, "highPressure", 0.1 * protection);
      weights.valueBias.progressDelta = 1;
      break;
    case "mixed":
    default:
      offsetEvent(weights, "generic", 0.05);
      break;
  }

  if (turnPressureBoost > 0) {
    offsetAction(weights, "advance", turnPressureBoost);
    offsetAction(weights, "sprint", turnPressureBoost);
    offsetEvent(weights, "highPressure", turnPressureBoost);
  }

  return weights;
}

