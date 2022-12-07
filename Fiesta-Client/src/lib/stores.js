import {writable} from "svelte/store";

export const isLoggedIn = writable(false);
export const isAdmin = writable(false);
export const secretCounter = writable(0);
export const user = writable();