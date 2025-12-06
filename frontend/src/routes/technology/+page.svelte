<script lang="ts">
    import { fade, fly } from "svelte/transition";
    import RustLogo from "$lib/components/rustLogo.svelte";
    import { onMount } from "svelte";

    let visible = $state(false);

    onMount(() => {
        visible = true;
    });

    const techStack = [
        {
            name: "Rust",
            description:
                "Powers the backend with a WebSocket server, chess engine, and game logic",
            features: [
                "Zero-cost abstractions",
                "Memory safety without garbage collection",
                "Threads without data races",
            ],
        },
        {
            name: "SvelteKit 2",
            description:
                "Modern frontend framework with the latest runes syntax for reactive state",
            features: [
                "Server-side rendering",
                "File-based routing",
                "Efficient reactivity model",
            ],
        },
        {
            name: "TypeScript",
            description:
                "Strongly typed JavaScript for enhanced developer experience",
            features: [
                "Type safety",
                "Better IDE integration",
                "Early error detection",
            ],
        },
        {
            name: "TailwindCSS",
            description: "Utility-first CSS framework for responsive design",
            features: [
                "Responsive design utilities",
                "No need for custom CSS",
                "Rapid UI development",
            ],
        },
    ];

    const architectureDetails = {
        backend: [
            "WebSocket server for real-time game updates",
            "Chess engine with move validation and state management",
            "RESTful API endpoints for game operations",
            "Axum web framework with Tokio async runtime",
        ],
        frontend: [
            "Drag and drop chess piece movement",
            "Touch-friendly mobile interface",
            "Animated piece transitions",
            "Responsive design for all screen sizes",
        ],
    };
</script>

<svelte:head>
    <title>Technology Stack - Chess App</title>
    <meta
        name="description"
        content="Learn about the technology stack powering the Chess App"
    />
</svelte:head>

<section class="flex flex-col items-center justify-start p-8 max-w-5xl mx-auto">
    {#if visible}
        <div in:fade={{ duration: 500 }} class="text-center mb-12 w-full">
            <h1
                class="text-4xl font-bold mb-6 bg-gradient-to-r from-primary-300 to-secondary-300 bg-clip-text text-transparent"
            >
                Built with Modern Technology
            </h1>
            <div class="flex flex-wrap justify-center items-center gap-4 mb-8">
                <div class="badge variant-filled-primary p-3 text-lg">Rust</div>
                <div class="badge variant-filled-secondary p-3 text-lg">
                    SvelteKit 2
                </div>
                <div class="badge variant-filled-success p-3 text-lg">
                    TypeScript
                </div>
                <div class="badge variant-filled-warning p-3 text-lg">
                    TailwindCSS
                </div>
            </div>
            <p class="text-xl text-primary-300">
                A full-stack chess application with real-time gameplay
            </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 w-full mb-12">
            <div
                in:fly={{ y: 20, duration: 500, delay: 200 }}
                class="card p-6 variant-glass-surface"
            >
                <header class="card-header">
                    <h2 class="h2">Backend Architecture</h2>
                </header>
                <section class="p-4">
                    <div class="flex items-center justify-center mb-4">
                        <RustLogo />
                    </div>
                    <ul class="list-disc list-inside space-y-2 text-left">
                        {#each architectureDetails.backend as detail}
                            <li>{detail}</li>
                        {/each}
                    </ul>
                </section>
            </div>

            <div
                in:fly={{ y: 20, duration: 500, delay: 300 }}
                class="card p-6 variant-glass-surface"
            >
                <header class="card-header">
                    <h2 class="h2">Frontend Architecture</h2>
                </header>
                <section class="p-4">
                    <div
                        class="flex items-center justify-center space-x-4 mb-4"
                    >
                        <img
                            src="/Typescript_logo_2020.svg"
                            alt="TypeScript logo"
                            class="w-12 h-12"
                        />
                        <img
                            src="/Tailwind_CSS_Logo.svg"
                            alt="Tailwind CSS logo"
                            class="w-12 h-12"
                        />
                    </div>
                    <ul class="list-disc list-inside space-y-2 text-left">
                        {#each architectureDetails.frontend as detail}
                            <li>{detail}</li>
                        {/each}
                    </ul>
                </section>
            </div>
        </div>

        <div in:fly={{ y: 20, duration: 500, delay: 400 }} class="w-full mb-12">
            <div class="card p-6 variant-glass-surface">
                <header class="card-header">
                    <h2 class="h2">Technology Deep Dive</h2>
                </header>
                <section class="p-4">
                    <div class="space-y-8">
                        {#each techStack as tech, i}
                            <div
                                class="space-y-2"
                                in:fly={{
                                    y: 20,
                                    duration: 300,
                                    delay: 500 + i * 100,
                                }}
                            >
                                <h3 class="h3">{tech.name}</h3>
                                <p class="text-lg">{tech.description}</p>
                                <div class="flex flex-wrap gap-2 mt-2">
                                    {#each tech.features as feature}
                                        <div class="chip variant-soft">
                                            {feature}
                                        </div>
                                    {/each}
                                </div>
                            </div>
                            {#if i < techStack.length - 1}
                                <hr class="opacity-50" />
                            {/if}
                        {/each}
                    </div>
                </section>
            </div>
        </div>

        <div in:fly={{ y: 20, duration: 500, delay: 600 }} class="w-full">
            <div class="card p-6 variant-glass-surface">
                <header class="card-header">
                    <h2 class="h2">Roadmap</h2>
                </header>
                <section class="p-4">
                    <ul
                        class="list-disc list-inside text-left text-lg space-y-4"
                    >
                        <li class="p-2 rounded variant-soft-primary">
                            Real-time multiplayer functionality with matchmaking
                        </li>
                        <li class="p-2 rounded variant-soft-secondary">
                            Advanced move history and game replay features
                        </li>
                        <li class="p-2 rounded variant-soft-tertiary">
                            Stockfish integration for move analysis
                        </li>
                        <li class="p-2 rounded variant-soft-success">
                            User accounts with game statistics and ratings
                        </li>
                        <li class="p-2 rounded variant-soft-warning">
                            Implementation of various chess variants
                        </li>
                    </ul>
                </section>
            </div>
        </div>
    {/if}
</section>

<style>
    /* Additional styling if needed */
    :global(body) {
        scroll-behavior: smooth;
    }
</style>
