import type { ButtonViewModel } from "../components/Button";
import { drawButton } from "../components/Button";
import { VIEWPORT } from "../layout";
import { Renderer } from "../renderer";
import { THEME } from "../theme";

export type HomeSceneAction = "start" | "toggleHelp" | null;

interface HitBox {
  id: HomeSceneAction;
  x: number;
  y: number;
  width: number;
  height: number;
}

export class HomeScene {
  private renderer: Renderer;
  private hitBoxes: HitBox[] = [];
  private showHelp = false;

  constructor(renderer: Renderer) {
    this.renderer = renderer;
  }

  render(ctx: CanvasRenderingContext2D): void {
    this.renderer.drawBackdrop(ctx);
    this.hitBoxes = [];
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";

    this.renderer.drawGlowText(ctx, "迷雾突围", VIEWPORT.width / 2, 180, "bold 76px sans-serif", "#bde9ff");
    ctx.font = THEME.font.subtitle;
    ctx.fillStyle = THEME.color.textMain;
    ctx.fillText("环境会学习你的习惯。", VIEWPORT.width / 2, 248);
    ctx.fillStyle = THEME.color.accent;
    ctx.fillText("你能在被看懂之前突围吗？", VIEWPORT.width / 2, 292);

    this.renderer.drawPanel(ctx, 56, 360, VIEWPORT.width - 112, 280);
    ctx.fillStyle = THEME.color.textMuted;
    ctx.font = THEME.font.body;
    ctx.textAlign = "left";
    ctx.fillText("· 10 回合内将进度推进至 15", 88, 430);
    ctx.fillText("· 管理体力 / 补给 / 风险 / 进度", 88, 478);
    ctx.fillText("· 迷雾会适应你的决策风格", 88, 526);
    ctx.fillText("· 每局 3~5 分钟，随时重开", 88, 574);

    this.drawButton(ctx, {
      id: "start",
      text: "开始突围",
      subtext: "进入异常迷雾区",
      x: 88,
      y: 710,
      width: VIEWPORT.width - 176,
      height: 118
    });

    this.drawButton(ctx, {
      id: "toggleHelp",
      text: "玩法说明",
      subtext: "查看资源与行动规则",
      x: 88,
      y: 850,
      width: VIEWPORT.width - 176,
      height: 104
    });

    if (this.showHelp) {
      this.renderer.drawPanel(ctx, 54, 990, VIEWPORT.width - 108, 270);
      ctx.fillStyle = THEME.color.textMain;
      ctx.font = THEME.font.small;
      ctx.fillText("前进: +进度 但加压", 84, 1038);
      ctx.fillText("搜索: +补给 但可能抬风险", 84, 1072);
      ctx.fillText("休整: 回体力 但消耗补给", 84, 1106);
      ctx.fillText("第 4 回合后环境会逐步识别你的习惯", 84, 1152);
      ctx.fillText("核心：切节奏，不要固化套路", 84, 1188);
    }
  }

  onTouch(x: number, y: number): HomeSceneAction {
    const hit = this.hitBoxes.find((box) => x >= box.x && x <= box.x + box.width && y >= box.y && y <= box.y + box.height);
    if (!hit) {
      return null;
    }
    if (hit.id === "toggleHelp") {
      this.showHelp = !this.showHelp;
    }
    return hit.id;
  }

  private drawButton(ctx: CanvasRenderingContext2D, button: ButtonViewModel): void {
    drawButton(ctx, button);
    this.hitBoxes.push({
      id: button.id as HomeSceneAction,
      x: button.x,
      y: button.y,
      width: button.width,
      height: button.height
    });
  }
}

