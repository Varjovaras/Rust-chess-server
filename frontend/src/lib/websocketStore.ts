// src/lib/websocketStore.ts
import { writable } from "svelte/store";

interface WebSocketStore {
	subscribe: (callback: (socket: WebSocket | null) => void) => () => void;
	send: (message: string) => void;
}

export const createWebSocketStore = (url: string): WebSocketStore => {
	const { subscribe, set } = writable<WebSocket | null>(null);
	let socket: WebSocket | null = null;

	if (typeof window !== "undefined") {
		socket = new WebSocket(url);
		socket.addEventListener("open", () => set(socket));
		socket.addEventListener("close", () => set(null));
	}

	return {
		subscribe,
		send: (message: string) => {
			if (socket && socket.readyState === WebSocket.OPEN) {
				socket.send(message);
			} else {
				console.error("WebSocket is not open. Message not sent.");
			}
		},
	};
};
