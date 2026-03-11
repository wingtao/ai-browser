export interface TouchPoint {
  x: number;
  y: number;
}

type TouchHandler = (touch: TouchPoint) => void;

export class TouchInput {
  private handler?: TouchHandler;
  private readonly wx: {
    onTouchStart?: (cb: (event: { changedTouches: Array<{ clientX: number; clientY: number }> }) => void) => void;
  };

  constructor(wxLike: TouchInput["wx"]) {
    this.wx = wxLike;
  }

  bind(handler: TouchHandler): void {
    this.handler = handler;
    this.wx.onTouchStart?.((event) => {
      const point = event.changedTouches[0];
      if (!point || !this.handler) {
        return;
      }
      this.handler({
        x: point.clientX,
        y: point.clientY
      });
    });
  }
}

