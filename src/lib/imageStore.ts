import { writable } from 'svelte/store';

export const imagePathsStore = writable<string[] | null>(null);
