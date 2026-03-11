import { styleLabel } from "../../content/copywriting";
import { endReasonLabel } from "../../core/gameEngine";
import type { GameRunState, SystemObservationReport } from "../../core/types";
import type { ButtonViewModel } from "../components/Button";
import { drawButton } from "../components/Button";
import { VIEWPORT } from "../layout";
import { Renderer } from "../renderer";
import { THEME } from "../theme";

export type ResultAction = "restart" | "readability_clear" | "readability_busy" | null;

export class ResultScene {
  private renderer: Renderer;
  private state: GameRunState | null = null;
  private report: SystemObservationReport | null = null;
  private restartButton: ButtonViewModel | null = null;
  private clearButton: ButtonViewModel | null = null;
  private busyButton: ButtonViewModel | null = null;

  constructor(renderer: Renderer) {
    this.renderer = renderer;
  }

  setResult(state: GameRunState, report: SystemObservationReport): void {
    this.state = state;
    this.report = report;
    this.restartButton = null;
    this.clearButton = null;
    this.busyButton = null;
  }

  render(ctx: CanvasRenderingContext2D): void {
    if (!this.state || !this.report || !this.state.endReason) {
      return;
    }
    this.renderer.drawBackdrop(ctx);
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    const isSuccess = this.state.endReason === "success";
    this.renderer.drawGlowText(
      ctx,
      isSuccess ? "突围成功" : "突围失败",
      VIEWPORT.width / 2,
      140,
      "bold 68px sans-serif",
      isSuccess ? THEME.color.accent : THEME.color.danger
    );
    ctx.font = THEME.font.body;
    ctx.fillStyle = THEME.color.textMain;
    ctx.fillText(endReasonLabel(this.state.endReason), VIEWPORT.width / 2, 198);

    this.renderer.drawPanel(ctx, 52, 248, VIEWPORT.width - 104, 210);
    ctx.textAlign = "left";
    ctx.font = THEME.font.small;
    ctx.fillStyle = THEME.color.textMuted;
    ctx.fillText(`体力：${this.state.resources.stamina}`, 80, 294);
    ctx.fillText(`补给：${this.state.resources.supply}`, 80, 328);
    ctx.fillText(`风险：${this.state.resources.risk}`, 80, 362);
    ctx.fillText(`进度：${this.state.resources.progress}`, 80, 396);
    ctx.fillText(`主要风格：${styleLabel(this.report.mainStyle)}`, 80, 430);

    this.renderer.drawPanel(ctx, 52, 486, VIEWPORT.width - 104, 462);
    ctx.fillStyle = THEME.color.accent;
    ctx.fillText("系统观察报告", 80, 526);
    ctx.fillStyle = THEME.color.textMain;
    ctx.fillText(
      `环境开始适应阶段：${this.report.adaptationStartTurn ? `第 ${this.report.adaptationStartTurn} 回合` : "未明显触发"}`,
      80,
      566
    );
    ctx.fillStyle = THEME.color.textMuted;
    ctx.font = THEME.font.small;
    this.report.pressureReasons.forEach((line, idx) => {
      wrapText(ctx, `- ${line}`, 80, 612 + idx * 56, VIEWPORT.width - 160, 30);
    });
    ctx.fillStyle = THEME.color.textMain;
    ctx.fillText("下一局建议：", 80, 746);
    this.report.suggestions.forEach((line, idx) => {
      wrapText(ctx, `${idx + 1}. ${line}`, 80, 782 + idx * 56, VIEWPORT.width - 160, 30);
    });

    this.restartButton = {
      id: "restart",
      text: "再来一局",
      subtext: "尝试新的节奏切换",
      x: 80,
      y: 980,
      width: VIEWPORT.width - 160,
      height: 108
    };
    drawButton(ctx, this.restartButton);

    this.clearButton = {
      id: "readability_clear",
      text: "界面清晰",
      subtext: "可读性良好",
      x: 80,
      y: 1110,
      width: 278,
      height: 92
    };
    this.busyButton = {
      id: "readability_busy",
      text: "信息偏花",
      subtext: "希望更聚焦",
      x: 392,
      y: 1110,
      width: 278,
      height: 92
    };
    drawButton(ctx, this.clearButton);
    drawButton(ctx, this.busyButton);
  }

  onTouch(x: number, y: number): ResultAction {
    if (!this.restartButton || !this.clearButton || !this.busyButton) {
      return null;
    }
    if (isHit(x, y, this.restartButton)) {
      return "restart";
    }
    if (isHit(x, y, this.clearButton)) {
      return "readability_clear";
    }
    if (isHit(x, y, this.busyButton)) {
      return "readability_busy";
    }
    return null;
  }
}

function isHit(x: number, y: number, button: ButtonViewModel): boolean {
  return (
    x >= button.x &&
    x <= button.x + button.width &&
    y >= button.y &&
    y <= button.y + button.height
  );
}

function wrapText(
  ctx: CanvasRenderingContext2D,
  text: string,
  x: number,
  y: number,
  maxWidth: number,
  lineHeight: number
): void {
  const chars = text.split("");
  let line = "";
  let lineIndex = 0;
  for (const char of chars) {
    const next = line + char;
    if (ctx.measureText(next).width > maxWidth) {
      ctx.fillText(line, x, y + lineIndex * lineHeight);
      line = char;
      lineIndex += 1;
    } else {
      line = next;
    }
  }
  if (line) {
    ctx.fillText(line, x, y + lineIndex * lineHeight);
  }
}

