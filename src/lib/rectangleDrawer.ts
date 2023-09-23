export class RectangleDrawer {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private isDrawing = false;
    private startX = 0;
    private startY = 0;
    private rectangles: [number, number, number, number][] = []; // Array of [xmin, ymin, xmax, ymax]

    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        this.ctx = this.canvas.getContext('2d')!;

        this.canvas.addEventListener('mousedown', this.handleMouseDown);
        this.canvas.addEventListener('mousemove', this.handleMouseMove);
        this.canvas.addEventListener('mouseup', this.handleMouseUp);
    }

    private handleMouseDown = (event: MouseEvent) => {
        this.isDrawing = true;
        this.startX = event.offsetX;
        this.startY = event.offsetY;
    }

    private handleMouseMove = (event: MouseEvent) => {
        if (!this.isDrawing) return;
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.drawRectangle(this.startX, this.startY, event.offsetX, event.offsetY);
    }

    private handleMouseUp = (event: MouseEvent) => {
        this.isDrawing = false;
        this.rectangles.push([this.startX, this.startY, event.offsetX, event.offsetY]);
    }

    private drawRectangle(x1: number, y1: number, x2: number, y2: number) {
        this.ctx.beginPath();
        this.ctx.rect(x1, y1, x2 - x1, y2 - y1);
        this.ctx.stroke();
    }

    public getRectangles(): [number, number, number, number][] {
        return this.rectangles;
    }
}
