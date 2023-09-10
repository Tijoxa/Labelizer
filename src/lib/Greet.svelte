<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';

	let name = '';
	let greetMsg = '';

	async function greet() {
		greetMsg = await invoke('greet', { name });
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			greet();
		}
	}
</script>

<div>
	<input
		id="greet-input"
		placeholder="Enter a name..."
		bind:value={name}
		on:keydown={handleKeydown}
	/>
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>

	{#if greetMsg != ''}
		<button on:click={() => goto('/home')}>Next</button>
	{/if}
</div>
