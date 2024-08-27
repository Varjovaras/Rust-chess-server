import { env } from "$env/dynamic/public";
import { chessSchema } from "$lib/types";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { startingPosition } from "$lib/components/chess/startingPosition";

export const load: PageServerLoad = async ({ fetch }) => {
	let apiUrl: string;
	if (import.meta.env.MODE === "development") {
		apiUrl = env.PUBLIC_DEV_URL;
	} else {
		apiUrl = env.PUBLIC_PROD_URL;
	}

	try {
		// const response = await fetch(`${apiUrl}/api/chess`);
		// if (!response.ok) {
		// 	throw new Error("Failed to fetch chess data");
		// }

		const data = startingPosition;
		const chess = chessSchema.parse(data);
		return {
			data: {
				chess,
				url: apiUrl,
			},
		};
	} catch (e) {
		return error(500, { message: "Backend not online" });
	}
};
