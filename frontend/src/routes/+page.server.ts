import { env } from "$env/dynamic/public";
import { chessSchema } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { startingPosition } from "$lib/components/chess/startingPosition";

export const load: PageServerLoad = async () => {
	let apiUrl: string;
	if (import.meta.env.MODE === "development") {
		apiUrl = env.PUBLIC_PROD_WS_URL;
	} else {
		apiUrl = env.PUBLIC_PROD_WS_URL;
	}

	return {
		data: {
			chess: startingPosition,
			url: apiUrl,
		},
	};
};
