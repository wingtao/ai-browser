import { generateSystemObservationReport } from "../core/reportGenerator";
import type { ActionId, GameRunState } from "../core/types";
import { GameEngine } from "../core/gameEngine";
import { TouchInput } from "../input/touchInput";
import { TelemetryTracker } from "../telemetry/tracker";
import { VIEWPORT } from "../ui/layout";
import { Renderer } from "../ui/renderer";
import { BattleScene } from "../ui/scenes/BattleScene";
import { HomeScene } from "../ui/scenes/HomeScene";
import { ResultScene } from "../ui/scenes/ResultScene";

type SceneId = "home" | "battle" | "result";

interface WxLike {
  createCanvas?: () => HTMLCanvasElement;
  getSystemInfoSync?: () => { windowWidth: number; windowHeight: number; pixelRatio: number };
  onTouchStart?: (cb: (event: { changedTouches: Array<{ clientX: number; clientY: number }> }) => void) => void;
  getStorageSync?: (key: string) => string;
  setStorageSync?: (key: string, value: string) => void;
}

declare const wx: WxLike | undefined;
declare function requestAnimationFrame(cb: () => void): number;

export class GameApp {
  private readonly wxLike: WxLike;
  private readonly canvas: HTMLCanvasElement;
  private readonly ctx: CanvasRenderingContext2D;
  private readonly renderer: Renderer;
  private readonly homeScene: HomeScene;
  private readonly battleScene: BattleScene;
  private readonly resultScene: ResultScene;
  private readonly tracker: TelemetryTracker;
  private readonly engine: GameEngine;
  private readonly touchInput: TouchInput;
  private scene: SceneId = "home";
  private runIndex = 0;
  private latestState: GameRunState | null = null;
  private scaleX = 1;
  private scaleY = 1;

  constructor() {
    this.wxLike = typeof wx === "undefined" ? {} : wx;
    this.canvas = this.createCanvas();
    const context = this.canvas.getContext("2d");
    if (!context) {
      throw new Error("Canvas 2D context unavailable");
    }
    this.ctx = context;
    this.renderer = new Renderer();
    this.homeScene = new HomeScene(this.renderer);
    this.battleScene = new BattleScene(this.renderer);
    this.resultScene = new ResultScene(this.renderer);
    this.tracker = new TelemetryTracker(this.wxLike);
    this.engine = new GameEngine({ tracker: this.tracker });
    this.touchInput = new TouchInput(this.wxLike);
    this.setupViewport();
  }

  start(): void {
    this.tracker.track("enter_home");
    this.touchInput.bind((point) => this.handleTouch(point.x / this.scaleX, point.y / this.scaleY));
    this.loop();
  }

  private loop(): void {
    this.render();
    if (typeof requestAnimationFrame === "function") {
      requestAnimationFrame(() => this.loop());
    } else {
      setTimeout(() => this.loop(), 16);
    }
  }

  private render(): void {
    this.ctx.clearRect(0, 0, VIEWPORT.width, VIEWPORT.height);
    switch (this.scene) {
      case "home":
        this.homeScene.render(this.ctx);
        break;
      case "battle":
        this.battleScene.render(this.ctx);
        break;
      case "result":
        this.resultScene.render(this.ctx);
        break;
    }
  }

  private handleTouch(x: number, y: number): void {
    if (this.scene === "home") {
      const action = this.homeScene.onTouch(x, y);
      if (action === "start") {
        this.startRun();
      } else if (action === "toggleHelp") {
        this.tracker.track("home_toggle_help");
      }
      return;
    }

    if (this.scene === "battle") {
      const action = this.battleScene.onTouch(x, y);
      if (!action || !this.latestState) {
        return;
      }
      this.applyAction(action);
      return;
    }

    if (this.scene === "result") {
      const action = this.resultScene.onTouch(x, y);
      if (action === "restart") {
        this.tracker.track("click_restart");
        this.startRun();
      }
    }
  }

  private startRun(): void {
    this.runIndex += 1;
    this.latestState = this.engine.startRun(this.runIndex);
    this.battleScene.setState(this.latestState);
    this.scene = "battle";
  }

  private applyAction(actionId: ActionId): void {
    this.latestState = this.engine.pickAction(actionId);
    if (!this.latestState) {
      return;
    }
    if (this.latestState.ended && this.latestState.endReason) {
      const report = generateSystemObservationReport({
        history: this.latestState.actionHistory,
        finalStyle: this.latestState.style,
        endReason: this.latestState.endReason
      });
      this.resultScene.setResult(this.latestState, report);
      this.scene = "result";
      return;
    }
    this.battleScene.setState(this.latestState);
  }

  private createCanvas(): HTMLCanvasElement {
    if (this.wxLike.createCanvas) {
      return this.wxLike.createCanvas();
    }
    const c = document.createElement("canvas");
    document.body.appendChild(c);
    return c;
  }

  private setupViewport(): void {
    const info = this.wxLike.getSystemInfoSync?.();
    if (!info) {
      this.canvas.width = VIEWPORT.width;
      this.canvas.height = VIEWPORT.height;
      return;
    }

    this.scaleX = info.windowWidth / VIEWPORT.width;
    this.scaleY = info.windowHeight / VIEWPORT.height;
    this.canvas.width = Math.floor(VIEWPORT.width * info.pixelRatio);
    this.canvas.height = Math.floor(VIEWPORT.height * info.pixelRatio);
    this.canvas.style.width = `${info.windowWidth}px`;
    this.canvas.style.height = `${info.windowHeight}px`;
    this.ctx.setTransform(info.pixelRatio, 0, 0, info.pixelRatio, 0, 0);
    this.ctx.scale(this.scaleX, this.scaleY);
  }
}

