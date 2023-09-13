import { writable } from 'svelte/store';

export const imagePathStore = writable<string | null>(null);
