import {writable} from "svelte/store";

export const isLoggedIn = writable();
export const isAdmin = writable();
export const secretCounter = writable(0);
export const user = writable();