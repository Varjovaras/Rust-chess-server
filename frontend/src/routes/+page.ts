import type { PageLoad } from "./$types";
import { env } from "$env/dynamic/public";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ fetch }) => {
	const isDevMode = import.meta.env.DEV;
	const apiUrl = isDevMode ? env.PUBLIC_DEV_URL : env.PUBLIC_PROD_URL;

	try {
		const response = await fetch(apiUrl);
		if (!response.ok) {
			throw new Error(`Failed to connect to backend: ${response.statusText}`);
		}
		// const data = await response.json();
		return {
			status: "ok",
			// data,
		};
	} catch (e) {
		console.error("Error connecting to backend:", e);
		error(500, "no backend found");
	}
};
