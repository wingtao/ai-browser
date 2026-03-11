import type { TrackerEvent } from "../core/types";

interface WxLikeStorage {
  getStorageSync?: (key: string) => string | undefined;
  setStorageSync?: (key: string, value: string) => void;
}

const STORAGE_KEY = "mist_breakout_telemetry";

export class TelemetryTracker {
  private events: TrackerEvent[] = [];
  private wxStorage?: WxLikeStorage;

  constructor(wxStorage?: WxLikeStorage) {
    this.wxStorage = wxStorage;
    this.load();
  }

  track(name: string, payload?: Record<string, unknown>): void {
    this.events.push({
      name,
      timestamp: Date.now(),
      payload
    });
    this.persist();
  }

  allEvents(): TrackerEvent[] {
    return [...this.events];
  }

  clear(): void {
    this.events = [];
    this.persist();
  }

  private load(): void {
    if (!this.wxStorage?.getStorageSync) {
      return;
    }
    const raw = this.wxStorage.getStorageSync(STORAGE_KEY);
    if (!raw) {
      return;
    }
    try {
      const parsed = JSON.parse(raw) as TrackerEvent[];
      if (Array.isArray(parsed)) {
        this.events = parsed;
      }
    } catch {
      this.events = [];
    }
  }

  private persist(): void {
    if (!this.wxStorage?.setStorageSync) {
      return;
    }
    this.wxStorage.setStorageSync(STORAGE_KEY, JSON.stringify(this.events));
  }
}

