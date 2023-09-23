<script lang="ts">
	import { onMount } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { imagePaths } from './dataStore';
	import { RectangleDrawer } from './rectangleDrawer';
	let canvas: HTMLCanvasElement, image: HTMLImageElement;

	let currentImageIndex = 0;

	function goToNext() {
		if ($imagePaths) {
			currentImageIndex = (currentImageIndex + 1) % $imagePaths.length;
		}
	}

	function goToPrev() {
		if ($imagePaths) {
			currentImageIndex = (currentImageIndex - 1 + $imagePaths.length) % $imagePaths.length;
		}
	}

	let drawer: RectangleDrawer;
	onMount(() => {
		drawer = new RectangleDrawer(canvas);
	});
</script>

<div class="display">
	{#if $imagePaths && $imagePaths.length}
		<button on:click={goToPrev}>&larr;</button>
		<img
			bind:this={image}
			src={convertFileSrc($imagePaths?.[currentImageIndex] ?? '')}
			alt="From folder"
		/>
		<canvas bind:this={canvas} />
		<button on:click={goToNext}>&rarr;</button>
	{/if}
</div>

<style>
	.display {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	img,
	canvas {
		position: absolute;
		top: 0;
		left: 0;
	}
</style>
