export interface RandomProvider {
  next(): number;
}

export class MathRandomProvider implements RandomProvider {
  next(): number {
    return Math.random();
  }
}

export class SeededRandomProvider implements RandomProvider {
  private seed: number;

  constructor(seed = 42) {
    this.seed = seed >>> 0;
  }

  next(): number {
    this.seed = (1664525 * this.seed + 1013904223) >>> 0;
    return this.seed / 0xffffffff;
  }
}

export function weightedPick<T>(
  entries: Array<{ item: T; weight: number }>,
  random: RandomProvider
): T {
  const safeEntries = entries.filter((entry) => entry.weight > 0);
  if (safeEntries.length === 0) {
    return entries[0].item;
  }

  const total = safeEntries.reduce((sum, entry) => sum + entry.weight, 0);
  let cursor = random.next() * total;

  for (const entry of safeEntries) {
    cursor -= entry.weight;
    if (cursor <= 0) {
      return entry.item;
    }
  }

  return safeEntries[safeEntries.length - 1].item;
}

