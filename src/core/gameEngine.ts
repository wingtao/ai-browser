import { tacticalPortraitText } from "../content/copywriting";
import { EVENT_TEMPLATES } from "../content/eventTemplates";
import { ACTION_DEFINITIONS, ACTION_IDS } from "./actions";
import { buildAdaptiveWeights, evaluateEnvironmentState } from "./adaptiveDirector";
import { GAME_CONFIG } from "./config";
import { pickEvent } from "./eventSystem";
import type { RandomProvider } from "./random";
import { MathRandomProvider, weightedPick } from "./random";
import { evaluateEndReason } from "./stateMachine";
import { classifyStyle } from "./styleClassifier";
import type {
  ActionEffect,
  ActionId,
  ActionOption,
  EndReason,
  GameRunState,
  Resources,
  RoundSnapshot,
  TacticalPortrait,
  TurnResolution
} from "./types";

function mergeEffect(base: ActionEffect, extra?: ActionEffect): ActionEffect {
  return {
    stamina: (base.stamina ?? 0) + (extra?.stamina ?? 0),
    supply: (base.supply ?? 0) + (extra?.supply ?? 0),
    risk: (base.risk ?? 0) + (extra?.risk ?? 0),
    progress: (base.progress ?? 0) + (extra?.progress ?? 0)
  };
}

function clampResource<K extends keyof Resources>(key: K, value: number): number {
  const rule = GAME_CONFIG.resourceClamp[key];
  return Math.max(rule.min, Math.min(rule.max, value));
}

function applyEffect(resources: Resources, effect: ActionEffect): Resources {
  return {
    stamina: clampResource("stamina", resources.stamina + (effect.stamina ?? 0)),
    supply: clampResource("supply", resources.supply + (effect.supply ?? 0)),
    risk: clampResource("risk", resources.risk + (effect.risk ?? 0)),
    progress: clampResource("progress", resources.progress + (effect.progress ?? 0))
  };
}

function toActionOption(actionId: ActionId): ActionOption {
  const def = ACTION_DEFINITIONS[actionId];
  return {
    id: def.id,
    name: def.name,
    category: def.category,
    description: def.description
  };
}

export interface TrackerLike {
  track(name: string, payload?: Record<string, unknown>): void;
}

export class GameEngine {
  private random: RandomProvider;
  private tracker?: TrackerLike;
  private state!: GameRunState;

  constructor(params?: { random?: RandomProvider; tracker?: TrackerLike }) {
    this.random = params?.random ?? new MathRandomProvider();
    this.tracker = params?.tracker;
  }

  startRun(runIndex: number): GameRunState {
    const style = "mixed";
    const environmentState = "unseen";
    this.state = {
      runIndex,
      turn: 1,
      resources: { ...GAME_CONFIG.initialResources },
      options: [],
      actionHistory: [],
      style,
      environmentState,
      firstRunComebackUsed: false,
      ended: false
    };

    this.state.options = this.generateActionOptions();
    this.track("start_battle", { runIndex });
    this.track("show_actions", { turn: this.state.turn, options: this.state.options.map((opt) => opt.id) });
    return this.getState();
  }

  getState(): GameRunState {
    return JSON.parse(JSON.stringify(this.state)) as GameRunState;
  }

  getSnapshot(): RoundSnapshot {
    return {
      turn: this.state.turn,
      resources: { ...this.state.resources },
      options: [...this.state.options],
      tacticalPortrait: this.currentPortrait()
    };
  }

  pickAction(actionId: ActionId): GameRunState {
    if (this.state.ended) {
      return this.getState();
    }

    const selectedOption = this.state.options.find((option) => option.id === actionId);
    if (!selectedOption) {
      throw new Error(`Action ${actionId} is not available in this turn`);
    }

    const actionDef = ACTION_DEFINITIONS[actionId];
    const actionRandomEffect = actionDef.randomEffect?.(this.random.next()) ?? {};
    const actionDelta = mergeEffect(actionDef.baseEffect, actionRandomEffect);

    const adaptive = buildAdaptiveWeights({
      style: this.state.style,
      turn: this.state.turn,
      isFirstRun: this.state.runIndex === 1
    });

    const pickedEvent = pickEvent({
      events: EVENT_TEMPLATES,
      turn: this.state.turn,
      style: this.state.style,
      adaptive,
      random: this.random
    });

    const eventDelta: ActionEffect = {
      ...pickedEvent.effect
    };

    if (
      this.state.environmentState === "responding" &&
      (pickedEvent.type === "styleResponse" || pickedEvent.type === "highPressure") &&
      adaptive.valueBias.riskDelta &&
      this.random.next() < 0.5
    ) {
      eventDelta.risk = (eventDelta.risk ?? 0) + adaptive.valueBias.riskDelta;
    }
    if (pickedEvent.type === "highPressure" && adaptive.valueBias.progressDelta) {
      eventDelta.progress = (eventDelta.progress ?? 0) + adaptive.valueBias.progressDelta;
    }

    const finalDelta = this.applyFirstRunProtection(mergeEffect(actionDelta, eventDelta));
    const nextResources = applyEffect(this.state.resources, finalDelta);
    const portrait = this.currentPortrait();

    const resolution: TurnResolution = {
      selectedAction: selectedOption,
      actionDelta,
      event: pickedEvent,
      eventDelta,
      finalDelta,
      tacticalPortrait: portrait
    };

    this.state.resources = nextResources;
    this.state.lastResolution = resolution;

    this.state.actionHistory.push({
      turn: this.state.turn,
      actionId,
      styleAtTurn: this.state.style,
      environmentState: this.state.environmentState
    });

    this.track("choose_action", { turn: this.state.turn, actionId });
    this.track("style_label_update", { turn: this.state.turn, style: this.state.style });
    this.track("trigger_event", { turn: this.state.turn, eventId: pickedEvent.id, eventType: pickedEvent.type });

    const endReason = evaluateEndReason(this.state.resources, this.state.turn);
    if (endReason) {
      this.state.ended = true;
      this.state.endReason = endReason;
      this.state.options = [];
      this.track("battle_end", { endReason, turn: this.state.turn, resources: this.state.resources });
      this.track("failure_reason", { endReason });
      return this.getState();
    }

    this.state.turn += 1;

    if (this.state.actionHistory.length % GAME_CONFIG.styleUpdateInterval === 0) {
      const recentActions = this.state.actionHistory
        .slice(-GAME_CONFIG.styleWindowSize)
        .map((item) => item.actionId);
      this.state.style = classifyStyle(recentActions);
    }

    this.state.environmentState = evaluateEnvironmentState(this.state.style, this.state.turn);
    this.state.options = this.generateActionOptions();
    this.track("show_actions", { turn: this.state.turn, options: this.state.options.map((opt) => opt.id) });
    this.track("show_tactical_portrait", {
      turn: this.state.turn,
      style: this.state.style,
      env: this.state.environmentState
    });

    return this.getState();
  }

  private generateActionOptions(): ActionOption[] {
    const adaptive = buildAdaptiveWeights({
      style: this.state.style,
      turn: this.state.turn,
      isFirstRun: this.state.runIndex === 1
    });
    const available = ACTION_IDS.filter((actionId) => ACTION_DEFINITIONS[actionId].canUse(this.state.resources));

    const picked: ActionId[] = [];
    const pool = [...available];
    while (picked.length < GAME_CONFIG.actionPoolSize && pool.length > 0) {
      const selected = weightedPick(
        pool.map((actionId) => ({
          item: actionId,
          weight: adaptive.actionWeight[actionId]
        })),
        this.random
      );
      picked.push(selected);
      const idx = pool.indexOf(selected);
      if (idx >= 0) {
        pool.splice(idx, 1);
      }
    }
    const ensured = this.ensureProgressOption(picked, pool);
    return ensured.map(toActionOption);
  }

  private currentPortrait(): TacticalPortrait {
    return {
      style: this.state.style,
      environmentState: this.state.environmentState,
      text: tacticalPortraitText(this.state.style, this.state.environmentState)
    };
  }

  private track(name: string, payload?: Record<string, unknown>): void {
    this.tracker?.track(name, payload);
  }

  private applyFirstRunProtection(delta: ActionEffect): ActionEffect {
    if (this.state.runIndex !== 1) {
      return delta;
    }
    const adjusted = { ...delta };
    if (this.state.turn <= 5 && (adjusted.risk ?? 0) >= 2) {
      adjusted.risk = Math.max(0, (adjusted.risk ?? 0) - 1);
    }
    if (!this.state.firstRunComebackUsed && this.state.turn >= 9 && this.state.resources.progress <= 9) {
      adjusted.progress = (adjusted.progress ?? 0) + 1;
      adjusted.risk = (adjusted.risk ?? 0) - 1;
      this.state.firstRunComebackUsed = true;
    }
    return adjusted;
  }

  private ensureProgressOption(selected: ActionId[], remainingPool: ActionId[]): ActionId[] {
    const hasProgress = selected.some(
      (actionId) => ACTION_DEFINITIONS[actionId].category === "progress"
    );
    if (hasProgress) {
      return selected;
    }

    const candidate =
      [...selected, ...remainingPool].find(
        (actionId) => ACTION_DEFINITIONS[actionId].category === "progress"
      ) ?? null;

    if (!candidate || selected.length === 0) {
      return selected;
    }

    const replacementIndex = selected.findIndex(
      (actionId) => ACTION_DEFINITIONS[actionId].category !== "progress"
    );
    if (replacementIndex >= 0) {
      selected[replacementIndex] = candidate;
    }
    return selected;
  }
}

export function endReasonLabel(endReason: EndReason): string {
  switch (endReason) {
    case "success":
      return "成功突围";
    case "staminaDepleted":
      return "体力耗尽";
    case "riskOverflow":
      return "风险失控";
    case "evacuationTimeout":
      return "撤离超时";
  }
}

