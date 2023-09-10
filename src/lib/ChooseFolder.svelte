<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let folderChosen = false;

	async function chooseFolder() {
		try {
			const response = (await invoke('choose_folder')) as { success: boolean };
			if (response.success) {
				folderChosen = true;
			}
		} catch (error) {
			console.error('Error choosing folder:', error);
		}
	}
</script>

<div class="container">
	<h1>Welcome</h1>
	<p>Choose a directory to get started</p>
	<button on:click={chooseFolder}>Choose Folder</button>

	<!-- make it work -->
	{#if folderChosen}
		<button>Go!</button>
	{/if}
</div>

<style>
	.container {
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
