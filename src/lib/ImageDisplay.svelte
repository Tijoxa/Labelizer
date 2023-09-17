<script lang="ts">
	import { onMount } from 'svelte';
	import { imagePaths } from './dataStore';
	let canvas: HTMLCanvasElement, image: HTMLImageElement;

	onMount(() => {
		canvas.width = image.width;
		canvas.height = image.height;
	});
</script>

<img bind:this={image} src={$imagePaths ? $imagePaths[0] : ''} alt="To annotate" />
<canvas bind:this={canvas} />

<style>
	img,
	canvas {
		position: absolute;
		top: 0;
		left: 0;
	}
</style>
