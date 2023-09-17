import { writable } from 'svelte/store';

export const imagePaths = writable<string[] | null>(null);
export const currentImagePath: string = '';
export const rectangles: number[][] = [];
