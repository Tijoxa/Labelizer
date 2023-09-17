<script lang="ts">
	import { rectangles } from './dataStore';

	let startX: number,
		startY: number,
		isDrawing = false;
	let canvas: HTMLCanvasElement;

	function startDrawing(event: MouseEvent) {
		startX = event.clientX - canvas.offsetLeft;
		startY = event.clientY - canvas.offsetTop;
		isDrawing = true;
	}

	function drawRectangle(event: MouseEvent) {
		if (!isDrawing) return;
		const x = event.clientX - canvas.offsetLeft;
		const y = event.clientY - canvas.offsetTop;

		const ctx = canvas.getContext('2d');
		if (!ctx) {
			throw new Error(
				'2D context not supported or canvas already initialized with another context type.'
			);
		}
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.beginPath();
		ctx.rect(startX, startY, x - startX, y - startY);
		ctx.stroke();

		rectangles.push([startX, startY, x - startX, y - startY]);
	}

	function stopDrawing() {
		isDrawing = false;
	}
</script>

<canvas
	bind:this={canvas}
	on:mousedown={startDrawing}
	on:mousemove={drawRectangle}
	on:mouseup={stopDrawing}
/>
