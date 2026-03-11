export type ActionId =
  | "advance"
  | "search"
  | "rest"
  | "detour"
  | "sprint"
  | "forcedSearch";

export type ActionCategory = "progress" | "supply" | "conservative";

export type StyleType = "searcher" | "sprinter" | "conservative" | "mixed";

export type EnvironmentState = "unseen" | "adapting" | "responding";

export type EventType = "generic" | "styleResponse" | "highPressure" | "comeback";

export interface Resources {
  stamina: number;
  supply: number;
  risk: number;
  progress: number;
}

export interface ActionEffect {
  stamina?: number;
  supply?: number;
  risk?: number;
  progress?: number;
}

export interface ActionDefinition {
  id: ActionId;
  name: string;
  category: ActionCategory;
  description: string;
  baseEffect: ActionEffect;
  canUse: (resources: Resources) => boolean;
  randomEffect?: (roll: number) => ActionEffect;
}

export interface ActionOption {
  id: ActionId;
  name: string;
  category: ActionCategory;
  description: string;
}

export interface EventTemplate {
  id: string;
  type: EventType;
  text: string;
  effect: ActionEffect;
  minTurn?: number;
  styleHint?: StyleType;
}

export interface AdaptiveWeights {
  actionWeight: Record<ActionId, number>;
  eventWeight: Record<EventType, number>;
  valueBias: {
    riskDelta: number;
    progressDelta: number;
    supplyDelta: number;
    staminaDelta: number;
  };
}

export interface TurnActionRecord {
  turn: number;
  actionId: ActionId;
  styleAtTurn: StyleType;
  environmentState: EnvironmentState;
}

export type EndReason = "staminaDepleted" | "riskOverflow" | "evacuationTimeout" | "success";

export interface TacticalPortrait {
  style: StyleType;
  environmentState: EnvironmentState;
  text: string;
}

export interface TurnResolution {
  selectedAction: ActionOption;
  actionDelta: ActionEffect;
  event: EventTemplate;
  eventDelta: ActionEffect;
  finalDelta: ActionEffect;
  tacticalPortrait: TacticalPortrait;
}

export interface RoundSnapshot {
  turn: number;
  resources: Resources;
  options: ActionOption[];
  tacticalPortrait: TacticalPortrait;
}

export interface GameRunState {
  runIndex: number;
  turn: number;
  resources: Resources;
  options: ActionOption[];
  actionHistory: TurnActionRecord[];
  style: StyleType;
  environmentState: EnvironmentState;
  firstRunComebackUsed: boolean;
  ended: boolean;
  endReason?: EndReason;
  lastResolution?: TurnResolution;
}

export interface SystemObservationReport {
  mainStyle: StyleType;
  adaptationStartTurn: number | null;
  pressureReasons: string[];
  suggestions: string[];
}

export interface TrackerEvent {
  name: string;
  timestamp: number;
  payload?: Record<string, unknown>;
}

