import type { PageServerLoad } from "./$types";
import { env } from "$env/dynamic/private";
import { error } from "@sveltejs/kit";
import { startingPosition } from "$lib/components/chess/startingPosition";

export const load: PageServerLoad = async ({ fetch }) => {
    // Use server-side environment variables for backend connection in Docker
    const backendHost = env.BACKEND_HOST || "localhost";
    const backendPort = env.BACKEND_PORT || "8000";
    const apiUrl = `http://${backendHost}:${backendPort}/status`;

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
