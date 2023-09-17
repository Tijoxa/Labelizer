<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { rectangles, currentImagePath } from './dataStore';

	function handleKeyDown(event: KeyboardEvent) {
		if (event.ctrlKey && event.key === 's') {
			event.preventDefault();
			saveRectanglesToFile(currentImagePath);
		}
	}

	function changeExtensionToTxt(filePath: string): string {
		const dotIndex = filePath.lastIndexOf('.');
		if (dotIndex === -1) {
			return filePath + '.txt';
		}
		return filePath.substring(0, dotIndex) + '.txt';
	}

	function saveRectanglesToFile(imagePath: string) {
		const formattedData = rectangles.map((r) => r.join(' ')).join('\n');
		invoke('save_to_file', { path: changeExtensionToTxt(imagePath), data: formattedData });
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeyDown);
		return () => {
			window.removeEventListener('keydown', handleKeyDown);
		};
	});
</script>
