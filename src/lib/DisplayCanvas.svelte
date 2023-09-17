<script lang="ts">
	import { imagePaths } from './dataStore';

	let canvas: HTMLCanvasElement;
	let currentIndex = 0;

	function handleKeyPress(event: KeyboardEvent, action: () => void) {
		if (event.key === 'Enter') {
			action();
		}
	}

	function goToPrevious() {
		if ($imagePaths && currentIndex > 0) {
			currentIndex--;
		}
	}

	function goToNext() {
		if ($imagePaths && currentIndex < $imagePaths.length - 1) {
			currentIndex++;
		}
	}
</script>

<div class="canvas-container">
	<span
		class="arrow"
		on:click={goToPrevious}
		on:keydown={(e) => handleKeyPress(e, goToPrevious)}
		tabindex="0"
		role="button">&larr;</span
	>
	<canvas bind:this={canvas} id="imageCanvas" />
	<span
		class="arrow"
		on:click={goToNext}
		on:keydown={(e) => handleKeyPress(e, goToNext)}
		tabindex="0"
		role="button">&rarr;</span
	>
</div>

<style>
	.canvas-container {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
	}

	.arrow {
		cursor: pointer;
		font-size: 2rem;
		margin: 0 10px;
	}

	canvas {
		border: 2px solid rgb(71, 71, 71);
	}
</style>
