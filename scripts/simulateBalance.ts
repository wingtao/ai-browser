import { GameEngine } from "../src/core/gameEngine";
import { SeededRandomProvider } from "../src/core/random";
import type { ActionId, GameRunState } from "../src/core/types";

type StrategyName = "newbie" | "adaptive";

function chooseAction(strategy: StrategyName, state: GameRunState): ActionId {
  const optionIds = state.options.map((item) => item.id);
  const prefer = (ids: ActionId[]): ActionId | null => ids.find((id) => optionIds.includes(id)) ?? null;

  if (strategy === "newbie") {
    if (state.resources.supply <= 1) {
      return prefer(["search", "forcedSearch", "advance"]) ?? optionIds[0];
    }
    if (state.turn >= 8) {
      return prefer(["sprint", "advance", "detour"]) ?? optionIds[0];
    }
    return prefer(["search", "advance", "rest"]) ?? optionIds[0];
  }

  // adaptive strategy
  if (state.resources.risk >= 8) {
    return prefer(["detour", "rest", "advance"]) ?? optionIds[0];
  }
  if (state.resources.stamina <= 2) {
    return prefer(["rest", "search", "detour"]) ?? optionIds[0];
  }
  if (state.turn >= 8) {
    return prefer(["sprint", "advance", "detour"]) ?? optionIds[0];
  }
  if (state.resources.progress < 7 && state.turn >= 5) {
    return prefer(["advance", "sprint", "detour"]) ?? optionIds[0];
  }
  return prefer(["advance", "search", "detour"]) ?? optionIds[0];
}

function emptyStats() {
  return {
    success: 0,
    staminaDepleted: 0,
    riskOverflow: 0,
    evacuationTimeout: 0
  };
}

function simulateNewbieFirstRun(rounds: number): void {
  const stats = emptyStats();

  for (let i = 0; i < rounds; i += 1) {
    const engine = new GameEngine({ random: new SeededRandomProvider(1000 + i) });
    let state = engine.startRun(1);
    while (!state.ended) {
      const action = chooseAction("newbie", state);
      state = engine.pickAction(action);
    }
    stats[state.endReason!] += 1;
  }

  const winRate = ((stats.success / rounds) * 100).toFixed(2);
  console.log(`\n[newbie-first-run] 总局数=${rounds}, 胜率=${winRate}%`);
  console.log(stats);
}

function simulateAdaptiveAfterPractice(rounds: number): void {
  const stats = emptyStats();

  for (let i = 0; i < rounds; i += 1) {
    const engine = new GameEngine({ random: new SeededRandomProvider(2000 + i) });

    for (let warmup = 1; warmup <= 3; warmup += 1) {
      let warmupState = engine.startRun(warmup);
      while (!warmupState.ended) {
        warmupState = engine.pickAction(chooseAction("adaptive", warmupState));
      }
    }

    let state = engine.startRun(4);
    while (!state.ended) {
      state = engine.pickAction(chooseAction("adaptive", state));
    }
    stats[state.endReason!] += 1;
  }

  const winRate = ((stats.success / rounds) * 100).toFixed(2);
  console.log(`\n[adaptive-after-3-runs] 总局数=${rounds}, 胜率=${winRate}%`);
  console.log(stats);
}

function simulate(strategy: StrategyName, rounds: number): void {
  const stats = {
    success: 0,
    staminaDepleted: 0,
    riskOverflow: 0,
    evacuationTimeout: 0
  };

  const random = new SeededRandomProvider(strategy === "newbie" ? 1001 : 2002);
  const engine = new GameEngine({ random });

  for (let i = 0; i < rounds; i += 1) {
    let state = engine.startRun(i + 1);
    while (!state.ended) {
      const action = chooseAction(strategy, state);
      state = engine.pickAction(action);
    }
    stats[state.endReason!] += 1;
  }

  const winRate = ((stats.success / rounds) * 100).toFixed(2);
  console.log(`\n[${strategy}] 总局数=${rounds}, 胜率=${winRate}%`);
  console.log(stats);
}

simulateNewbieFirstRun(2000);
simulateAdaptiveAfterPractice(2000);
simulate("newbie", 2000);
simulate("adaptive", 2000);

