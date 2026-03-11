import { environmentLabel, styleLabel } from "../../content/copywriting";
import type { ActionId, GameRunState } from "../../core/types";
import type { ButtonViewModel } from "../components/Button";
import { drawButton } from "../components/Button";
import { VIEWPORT } from "../layout";
import { Renderer } from "../renderer";
import { THEME } from "../theme";

interface HitBox {
  id: ActionId;
  x: number;
  y: number;
  width: number;
  height: number;
}

export class BattleScene {
  private renderer: Renderer;
  private hitBoxes: HitBox[] = [];
  private state: GameRunState | null = null;

  constructor(renderer: Renderer) {
    this.renderer = renderer;
  }

  setState(state: GameRunState): void {
    this.state = state;
  }

  render(ctx: CanvasRenderingContext2D): void {
    if (!this.state) {
      return;
    }
    this.renderer.drawBackdrop(ctx);
    this.hitBoxes = [];

    this.drawTopStatus(ctx);
    this.drawResourceCards(ctx);
    this.drawEventPanel(ctx);
    this.drawTacticalPortrait(ctx);
    this.drawActionButtons(ctx);
  }

  onTouch(x: number, y: number): ActionId | null {
    const hit = this.hitBoxes.find((box) => x >= box.x && x <= box.x + box.width && y >= box.y && y <= box.y + box.height);
    return hit?.id ?? null;
  }

  private drawTopStatus(ctx: CanvasRenderingContext2D): void {
    if (!this.state) {
      return;
    }
    const turnText = `第 ${this.state.turn} / 10 回合`;
    const progressRatio = Math.min(1, this.state.resources.progress / 15);
    ctx.textAlign = "left";
    ctx.textBaseline = "middle";
    ctx.fillStyle = THEME.color.textMain;
    ctx.font = THEME.font.subtitle;
    ctx.fillText(turnText, 40, 56);

    this.renderer.drawPanel(ctx, 40, 86, VIEWPORT.width - 80, 46);
    ctx.fillStyle = "rgba(110, 240, 215, 0.26)";
    ctx.fillRect(56, 102, (VIEWPORT.width - 112) * progressRatio, 14);
    ctx.fillStyle = THEME.color.textMuted;
    ctx.font = THEME.font.small;
    ctx.fillText(`撤离进度 ${this.state.resources.progress}/15`, 56, 150);
  }

  private drawResourceCards(ctx: CanvasRenderingContext2D): void {
    if (!this.state) {
      return;
    }
    const labels = [
      { label: "体力", value: this.state.resources.stamina, color: "#90c9ff" },
      { label: "补给", value: this.state.resources.supply, color: "#88efcf" },
      {
        label: "风险",
        value: this.state.resources.risk,
        color: this.state.resources.risk >= 7 ? THEME.color.danger : THEME.color.warning
      },
      { label: "进度", value: this.state.resources.progress, color: THEME.color.accent }
    ];

    const width = (VIEWPORT.width - 100) / 2;
    labels.forEach((item, idx) => {
      const col = idx % 2;
      const row = Math.floor(idx / 2);
      const x = 40 + col * (width + 20);
      const y = 186 + row * 126;
      this.renderer.drawPanel(ctx, x, y, width, 108);
      ctx.fillStyle = THEME.color.textMuted;
      ctx.font = THEME.font.small;
      ctx.fillText(item.label, x + 18, y + 34);
      this.renderer.drawGlowText(ctx, String(item.value), x + 18, y + 78, "bold 36px sans-serif", item.color);
    });
  }

  private drawEventPanel(ctx: CanvasRenderingContext2D): void {
    if (!this.state) {
      return;
    }
    this.renderer.drawPanel(ctx, 40, 456, VIEWPORT.width - 80, 186);
    ctx.fillStyle = THEME.color.textMuted;
    ctx.font = THEME.font.small;
    ctx.textAlign = "left";
    ctx.fillText("局势回响", 58, 488);

    ctx.font = THEME.font.body;
    ctx.fillStyle = THEME.color.textMain;
    const text = this.state.lastResolution?.event.text ?? "迷雾仍在流动，等待你的下一步。";
    wrapText(ctx, text, 58, 530, VIEWPORT.width - 120, 34);
  }

  private drawTacticalPortrait(ctx: CanvasRenderingContext2D): void {
    if (!this.state) {
      return;
    }
    this.renderer.drawPanel(ctx, 40, 672, VIEWPORT.width - 80, 196);
    ctx.fillStyle = THEME.color.accent;
    ctx.font = THEME.font.small;
    ctx.fillText("战术画像", 58, 706);

    ctx.fillStyle = THEME.color.textMain;
    ctx.fillText(`风格：${styleLabel(this.state.style)}   环境：${environmentLabel(this.state.environmentState)}`, 58, 742);
    ctx.font = THEME.font.body;
    wrapText(ctx, this.state.lastResolution?.tacticalPortrait.text ?? "迷雾正在读取你的选择。", 58, 784, VIEWPORT.width - 120, 34);
  }

  private drawActionButtons(ctx: CanvasRenderingContext2D): void {
    if (!this.state) {
      return;
    }
    const baseY = 900;
    const height = 120;
    const width = VIEWPORT.width - 80;
    this.state.options.forEach((option, index) => {
      const y = baseY + index * 136;
      const model: ButtonViewModel = {
        id: option.id,
        text: option.name,
        subtext: option.description,
        x: 40,
        y,
        width,
        height
      };
      drawButton(ctx, model);
      this.hitBoxes.push({
        id: option.id,
        x: model.x,
        y: model.y,
        width: model.width,
        height: model.height
      });
    });
  }
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

