import { VIEWPORT } from "./layout";
import { THEME } from "./theme";

interface FogParticle {
  x: number;
  y: number;
  r: number;
  speed: number;
  alpha: number;
}

export class Renderer {
  private fogParticles: FogParticle[];
  private startTime = Date.now();

  constructor() {
    this.fogParticles = Array.from({ length: 22 }).map((_, idx) => ({
      x: (idx * 97) % VIEWPORT.width,
      y: (idx * 151) % VIEWPORT.height,
      r: 80 + (idx % 5) * 26,
      speed: 0.2 + (idx % 4) * 0.06,
      alpha: 0.05 + (idx % 6) * 0.015
    }));
  }

  clear(ctx: CanvasRenderingContext2D): void {
    ctx.clearRect(0, 0, VIEWPORT.width, VIEWPORT.height);
  }

  drawBackdrop(ctx: CanvasRenderingContext2D): void {
    const gradient = ctx.createLinearGradient(0, 0, 0, VIEWPORT.height);
    gradient.addColorStop(0, THEME.color.bgTop);
    gradient.addColorStop(1, THEME.color.bgBottom);
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, VIEWPORT.width, VIEWPORT.height);

    this.drawFog(ctx);
    this.drawScanLine(ctx);
  }

  drawPanel(ctx: CanvasRenderingContext2D, x: number, y: number, width: number, height: number): void {
    ctx.save();
    ctx.fillStyle = THEME.color.panel;
    roundRect(ctx, x, y, width, height, 18);
    ctx.fill();
    ctx.strokeStyle = THEME.color.panelStroke;
    ctx.lineWidth = 2;
    roundRect(ctx, x, y, width, height, 18);
    ctx.stroke();
    ctx.restore();
  }

  drawGlowText(
    ctx: CanvasRenderingContext2D,
    text: string,
    x: number,
    y: number,
    font: string,
    color: string
  ): void {
    ctx.save();
    ctx.font = font;
    ctx.fillStyle = color;
    ctx.shadowColor = color;
    ctx.shadowBlur = 20;
    ctx.fillText(text, x, y);
    ctx.restore();
  }

  private drawFog(ctx: CanvasRenderingContext2D): void {
    const elapsed = (Date.now() - this.startTime) / 1000;
    for (const particle of this.fogParticles) {
      const dx = (particle.x + elapsed * 10 * particle.speed) % (VIEWPORT.width + particle.r * 2);
      const dy = (particle.y + Math.sin(elapsed * particle.speed) * 24 + VIEWPORT.height) % VIEWPORT.height;
      const g = ctx.createRadialGradient(dx, dy, particle.r * 0.15, dx, dy, particle.r);
      g.addColorStop(0, `rgba(146, 203, 255, ${particle.alpha})`);
      g.addColorStop(1, "rgba(80, 130, 200, 0)");
      ctx.fillStyle = g;
      ctx.beginPath();
      ctx.arc(dx, dy, particle.r, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  private drawScanLine(ctx: CanvasRenderingContext2D): void {
    const elapsed = (Date.now() - this.startTime) / 1000;
    const y = ((elapsed * 46) % (VIEWPORT.height + 80)) - 40;
    const gradient = ctx.createLinearGradient(0, y - 22, 0, y + 22);
    gradient.addColorStop(0, "rgba(110, 240, 215, 0)");
    gradient.addColorStop(0.5, "rgba(110, 240, 215, 0.12)");
    gradient.addColorStop(1, "rgba(110, 240, 215, 0)");
    ctx.fillStyle = gradient;
    ctx.fillRect(0, y - 22, VIEWPORT.width, 44);
  }
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

