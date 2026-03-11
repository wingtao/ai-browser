import { GAME_CONFIG } from "./config";
import type { EndReason, Resources } from "./types";

export function evaluateEndReason(resources: Resources, playedTurn: number): EndReason | null {
  if (resources.stamina <= 0) {
    return "staminaDepleted";
  }
  if (resources.risk >= GAME_CONFIG.riskFailThreshold) {
    return "riskOverflow";
  }
  if (resources.progress >= GAME_CONFIG.targetProgress) {
    return "success";
  }
  if (playedTurn >= GAME_CONFIG.maxTurns) {
    return "evacuationTimeout";
  }
  return null;
}

