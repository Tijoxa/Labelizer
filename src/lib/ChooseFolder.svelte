<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';
	import { imagePaths } from './dataStore';

	async function chooseFolder() {
		try {
			const response = (await invoke('choose_folder')) as string[];
			if (response) {
				imagePaths.set(response);
			}
		} catch (error) {
			console.error('Error choosing folder:', error);
		}
	}
</script>

<div class="choose_folder">
	<h2>Choose a directory to get started</h2>
	<button on:click={chooseFolder}>Choose Folder</button>

	{#if imagePaths}
		<button on:click={() => goto('/display')}>Next</button>
	{/if}
</div>

<style>
	.choose_folder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100vh;
		font-family: Arial, sans-serif;
	}

	button {
		margin-top: 20px;
		padding: 10px 20px;
	}
</style>
