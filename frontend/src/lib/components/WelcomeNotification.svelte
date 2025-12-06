<script lang="ts">
    import { fade, fly } from "svelte/transition";
    import { onMount } from "svelte";

    export const duration: number = 8000;
    let show = $state(false);

    onMount(() => {
        // Show notification shortly after component mounts
        setTimeout(() => {
            show = false;
        }, 500);

        // Auto-hide after duration
        if (duration > 0) {
            setTimeout(() => {
                show = false;
            }, duration + 500);
        }
    });
</script>

{#if show}
    <div
        class="fixed bottom-20 sm:bottom-6 left-1/2 transform -translate-x-1/2 px-4 py-3 rounded-md shadow-md border-l-4 max-w-md w-11/12 z-50 bg-blue-100 border-blue-500 text-blue-700"
        in:fly={{ y: 20, duration: 300 }}
        out:fade={{ duration: 200 }}
        role="alert"
    >
        <div class="flex items-center justify-between">
            <div class="flex items-center">
                <span class="mr-2">
                    <svg
                        class="w-5 h-5"
                        fill="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
                            clip-rule="evenodd"
                        ></path>
                    </svg>
                </span>
                <p class="text-sm">
                    There might be a slight delay when moving pieces as move
                    validation happens on the server. Frontend validation will
                    be added in future updates.
                </p>
            </div>
            <button
                onclick={() => (show = false)}
                class="ml-3 text-gray-600 hover:text-gray-800"
            >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path
                        fill-rule="evenodd"
                        d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                        clip-rule="evenodd">x</path
                    >
                </svg>
            </button>
        </div>
    </div>
{/if}
