import type { PageLoad } from "./$types";
import { env } from "$env/dynamic/public";
import { error } from "@sveltejs/kit";
import { startingPosition } from "$lib/components/chess/startingPosition";

export const load: PageLoad = async ({ fetch }) => {
	const isDevMode = import.meta.env.DEV;
	const apiUrl = isDevMode ? env.PUBLIC_DEV_URL : env.PUBLIC_PROD_URL;

	try {
		const response = await fetch(apiUrl);
		console.log(response);
		if (!response.ok) {
			throw new Error(`Failed to connect to backend: ${response.statusText}`);
		}
		return {
			status: "ok",
			startingPosition: startingPosition,
		};
	} catch (e) {
		console.error("Error connecting to backend:", e);
		error(500, "no backend found");
	}
};
