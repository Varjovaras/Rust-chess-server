<script lang="ts">
    interface Props {
        isConnected: boolean;
        onReset: () => void;
    }

    const { isConnected, onReset }: Props = $props();
</script>

<div class="space-y-6 lg:pr-8">
    <div class="card variant-ghost-surface p-6 md:p-8 space-y-4">
        <div class="space-y-2">
            <h1
                class="h1 font-bold bg-gradient-to-br from-primary-500 to-secondary-500 bg-clip-text text-transparent leading-tight"
            >
                Chess In Rust
            </h1>
            <div class="flex gap-2 flex-wrap">
                <span class="chip variant-filled-primary">Rust</span>
                <span class="chip variant-filled-secondary">Svelte</span>
            </div>
        </div>

        <hr class="!border-t-2 opacity-50" />

        <div class="space-y-3">
            <div class="space-y-2">
                <div class="flex items-start gap-3">
                    <span class="text-primary-500 text-2xl">⚡</span>
                    <div>
                        <h3 class="h4 font-semibold">Built with Rust</h3>
                        <p class="text-surface-600-300-token">
                            The actual app is a very fast rust library. Backend
                            performance could be improved a ton by making chess
                            lookup based on bitboards.
                            <a
                                class="text-primary-500 hover:text-primary-600 transition-colors duration-200"
                                href="https://www.chessprogramming.org/Bitboards"
                                target="_blank"
                                rel="noopener noreferrer"
                                >More info on bitboards here</a
                            >
                        </p>
                    </div>
                </div>

                <div class="flex items-start gap-3">
                    <span class="text-secondary-500 text-2xl">🎨</span>
                    <div>
                        <h3 class="h4 font-semibold">
                            Frontend demo in Svelte
                        </h3>
                        <p class="text-surface-600-300-token">
                            Frontend calls backend API through WebSocket for
                            instant move updates.
                        </p>
                    </div>
                </div>

                <div class="flex items-start gap-3">
                    <span class="text-tertiary-500 text-2xl">🚀</span>
                    <div>
                        <h3 class="h4 font-semibold">Todo List</h3>
                        <p class="text-surface-600-300-token">
                            Multiplayer gameplay, PGN support (both export and
                            import), analysis, going back moves, and more
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <hr class="!border-t-2 opacity-50" />

        <div class="flex items-center gap-3 flex-wrap">
            <div
                class="badge {isConnected
                    ? 'variant-filled-success'
                    : 'variant-filled-error'}"
            >
                {isConnected ? "● Connected" : "● Disconnected"}
            </div>
            <button onclick={onReset} class="btn variant-ringed-primary btn-sm">
                Reset Game
            </button>
        </div>
    </div>
</div>
