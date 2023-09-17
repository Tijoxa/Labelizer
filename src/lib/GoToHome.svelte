<script lang="ts">
	import { goto } from '$app/navigation';

	function navigateHome() {
		goto('/');
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			navigateHome();
		}
	}
</script>

<div
	class="home-button"
	on:click={navigateHome}
	on:keydown={handleKeyPress}
	tabindex="0"
	role="button"
	aria-label="Navigate to main menu"
>
	<span class="home-icon" />
	🏠
</div>

<style>
	.home-button {
		position: absolute;
		top: 10px;
		right: 10px;
		display: inline-flex;
		align-items: center;
		cursor: pointer;
	}

	.home-icon {
		margin-right: 8px;
		content: '🏠';
	}
</style>
