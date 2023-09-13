<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<div class="page">
	<h1>Welcome</h1>
	<button on:click={() => goto('/home')}>Go to home</button>
</div>

<style>
	.page {
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
