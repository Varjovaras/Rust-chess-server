import type { PageServerLoad } from "./$types";
import { BACKEND_HOST, BACKEND_PORT } from "$app/env/private";
import { error } from "@sveltejs/kit";
import { startingPosition } from "#lib/components/chess/startingPosition.js";

export const load: PageServerLoad = async ({ fetch }) => {
    // Use server-side environment variables for backend connection in Docker
    const backendHost = BACKEND_HOST || "localhost";
    const backendPort = BACKEND_PORT || "8000";
    const apiUrl = `http://${backendHost}:${backendPort}/status`;

    try {
        const response = await fetch(apiUrl);

        if (!response.ok) {
            throw new Error(
                `Failed to connect to backend: ${response.statusText}`,
            );
        }

        return { status: "ok", startingPosition };
    } catch(e) {
        console.error("Error connecting to backend:", e);
        error(500, "Backend connection failed");
    }
};
