<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { imagePathStore } from './imageStore';

	$: if ($imagePathStore) {
		console.log('Stored Image Path:', $imagePathStore);
	}
</script>

<div class="display">
	{#if $imagePathStore}
		<src={$imagePathStore} alt="Selected image" />
	{/if}
</div>
