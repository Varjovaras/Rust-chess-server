import { join } from "node:path";
import type { Config } from "tailwindcss";

// 1. Import the Skeleton plugin
import { skeleton } from "@skeletonlabs/tw-plugin";

const config = {
    // 2. Opt for dark mode to be handled via the class method
    darkMode: "class",
    content: [
        "./src/**/*.{html,js,svelte,ts}",
        // 3. Append the path to the Skeleton package
        join(
            require.resolve("@skeletonlabs/skeleton"),
            "../**/*.{html,js,svelte,ts}",
        ),
    ],
    theme: {
        extend: {},
    },
    plugins: [
        skeleton({
            themes: {
                preset: [
                    // Enable 'enhancements' per each registered theme:
                    { name: "skeleton", enhancements: true },
                    { name: "modern", enhancements: true },
                    { name: "rocket", enhancements: true },
                    { name: "seafoam", enhancements: true },
                    { name: "vintage", enhancements: true },
                    { name: "sahara", enhancements: true },
                    { name: "hamlindigo", enhancements: true },
                    { name: "gold-nouveau", enhancements: true },
                    { name: "crimson", enhancements: true },
                ],
            },
        }),
    ],
} satisfies Config;

export default config;
