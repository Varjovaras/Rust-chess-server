import type { PageLoad } from "./$types";
import { env } from "$env/dynamic/public";
import { error } from "@sveltejs/kit";
import { startingPosition } from "$lib/components/chess/startingPosition";

export const load: PageLoad = async ({ fetch }) => {
    const isDevMode = import.meta.env.DEV;
    // Provide default values to ensure apiUrl is never undefined
    const apiUrl = isDevMode
        ? env.PUBLIC_DEV_URL || "http://localhost:8000"
        : env.PUBLIC_PROD_URL || "";

    try {
        const response = await fetch(apiUrl);

        if (!response.ok) {
            throw new Error(
                `Failed to connect to backend: ${response.statusText}`,
            );
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
