<script>
	let folderChosen = false;

	function chooseFolder() {
		// Here, you would typically open a directory chooser.
		// For the sake of this example, we'll just simulate it.
		folderChosen = true;
	}
</script>

<div class="container">
	<h1>Welcome</h1>
	<p>Choose a directory to get started</p>
	<button on:click={chooseFolder}>Choose Folder</button>

	{#if folderChosen}
		<button>Next</button>
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
