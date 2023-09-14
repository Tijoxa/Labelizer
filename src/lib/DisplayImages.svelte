<script lang="ts">
	import { imagePathsStore } from './imageStore';

	$: if ($imagePathsStore) {
		console.log('Stored Image Path:', $imagePathsStore);
	}

	function pathToFileURL(path: string) {
		return `file://${path}`;
	}
</script>

<div class="display">
	{#each $imagePathsStore || [] as path (path)}
		<src={pathToFileURL(path)} alt="Image from folder" />
	{/each}
</div>
