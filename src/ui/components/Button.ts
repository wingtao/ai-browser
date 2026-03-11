import { THEME } from "../theme";

export interface ButtonViewModel {
  id: string;
  text: string;
  x: number;
  y: number;
  width: number;
  height: number;
  subtext?: string;
  disabled?: boolean;
}

export function drawButton(ctx: CanvasRenderingContext2D, model: ButtonViewModel): void {
  const radius = 18;
  const { x, y, width, height } = model;
  ctx.save();

  const grad = ctx.createLinearGradient(x, y, x + width, y + height);
  grad.addColorStop(0, model.disabled ? "rgba(90, 98, 116, 0.6)" : THEME.color.button);
  grad.addColorStop(1, model.disabled ? "rgba(66, 70, 90, 0.65)" : "#21406d");

  ctx.fillStyle = grad;
  roundRect(ctx, x, y, width, height, radius);
  ctx.fill();

  if (!model.disabled) {
    ctx.shadowColor = THEME.color.buttonGlow;
    ctx.shadowBlur = 24;
    ctx.strokeStyle = "rgba(157, 220, 255, 0.62)";
    ctx.lineWidth = 2;
    roundRect(ctx, x, y, width, height, radius);
    ctx.stroke();
  }

  ctx.shadowBlur = 0;
  ctx.fillStyle = model.disabled ? "#a1a6b4" : THEME.color.textMain;
  ctx.font = THEME.font.button;
  ctx.textAlign = "left";
  ctx.textBaseline = "middle";
  ctx.fillText(model.text, x + 24, y + height * 0.42);

  if (model.subtext) {
    ctx.font = THEME.font.small;
    ctx.fillStyle = model.disabled ? "#8f95a4" : THEME.color.textMuted;
    ctx.fillText(model.subtext, x + 24, y + height * 0.73);
  }

  ctx.restore();
}

function roundRect(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  radius: number
): void {
  ctx.beginPath();
  ctx.moveTo(x + radius, y);
  ctx.lineTo(x + width - radius, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
  ctx.lineTo(x + width, y + height - radius);
  ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);
  ctx.lineTo(x + radius, y + height);
  ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
  ctx.lineTo(x, y + radius);
  ctx.quadraticCurveTo(x, y, x + radius, y);
  ctx.closePath();
}

