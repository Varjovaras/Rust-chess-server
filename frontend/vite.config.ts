import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
	// optimizeDeps: {
	//     exclude: ["fsevents"],
	// },
	css: {
		// tolerate invalid CSS coming from dependencies (e.g. @skeletonlabs/skeleton)
		// that lightningcss rejects during minification
		lightningcss: {
			errorRecovery: true
		}
	},
	plugins: [
		sveltekit({
			// Consult https://kit.svelte.dev/docs/integrations#preprocessors
			// for more information about preprocessors
			preprocess: vitePreprocess(),

			// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
			// If your environment is not supported or you settled on a specific environment, switch out the adapter.
			// See https://kit.svelte.dev/docs/adapters for more information about adapters.
			adapter: adapter(),
			paths: {
				base: (process.env.BASE_PATH || "") as "" | `/${string}`,
				// public-facing origin, previously configured via adapter-node's ORIGIN env var
				origin: "https://hcbull.com"
			}
		})
	]
});
