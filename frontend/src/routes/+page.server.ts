import { env } from "$env/dynamic/public";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { chessSchema } from "$lib/types";

export const load: PageServerLoad = async ({ fetch }) => {
	let wsUrl: string;
	if (import.meta.env.MODE === "development") {
		wsUrl = env.PUBLIC_DEV_WS_URL;
	} else {
		wsUrl = env.PUBLIC_PROD_WS_URL;
	}

	const ws = new WebSocket(wsUrl);

	ws.onopen = () => {
		console.log("WebSocket is connected");
	};

	ws.onmessage = (event) => {
		console.log(`Received data: ${event.data["chess"]}`);
	};

	ws.onerror = (errorEvent) => {
		console.error("WebSocket error: ", errorEvent);
	};
	ws.onclose = (event) => {
		console.log("WebSocket is closed with event: ", event);
	};
};
