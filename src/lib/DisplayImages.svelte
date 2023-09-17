<script lang="ts">
	import { imagePaths } from './dataStore';

	$: if ($imagePaths) {
		console.log('Stored Image Path:', $imagePaths);
	}

	function pathToFileURL(path: string) {
		return `file://${path}`;
	}
</script>

<!-- TODO: give file_path to backend, render image and send this with canva to draw on, and switch next previous with arrow key -->

<div class="display">
	{#each $imagePaths || [] as path (path)}
		<p1>{pathToFileURL(path)}</p1>
		<!-- <img src={pathToFileURL(path)} alt="From folder" /> -->
	{/each}
</div>
