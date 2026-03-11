import { weightedPick, type RandomProvider } from "./random";
import type { AdaptiveWeights, EventTemplate, EventType, StyleType } from "./types";

function eventTypeWeight(eventType: EventType, adaptive: AdaptiveWeights): number {
  return adaptive.eventWeight[eventType] ?? 1;
}

function styleWeight(event: EventTemplate, style: StyleType): number {
  if (!event.styleHint) {
    return 1;
  }
  return event.styleHint === style ? 1.25 : 0.65;
}

export function pickEvent(params: {
  events: EventTemplate[];
  turn: number;
  style: StyleType;
  adaptive: AdaptiveWeights;
  random: RandomProvider;
}): EventTemplate {
  const { events, turn, style, adaptive, random } = params;
  const available = events.filter((event) => !event.minTurn || turn >= event.minTurn);
  const weighted = available.map((event) => ({
    item: event,
    weight: eventTypeWeight(event.type, adaptive) * styleWeight(event, style)
  }));

  return weightedPick(weighted, random);
}

