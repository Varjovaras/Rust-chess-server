<script lang="ts">
    import Header from "$lib/components/header.svelte";
    import "../app.css";
    import { Modal, initializeStores } from "@skeletonlabs/skeleton";
    interface Props {
        children?: import("svelte").Snippet;
    }

    const { children }: Props = $props();
    initializeStores();
</script>

<svelte:head>
    <title>Chess App</title>
    <meta name="description" content="Chess portfolio simulator" />
</svelte:head>

<Modal />

<div class="min-h-screen flex flex-col">
    <Header />

    <main class="w-full flex-1">
        {@render children?.()}
    </main>
    <!-- <Footer /> -->
</div>

<style>
    :global(html, body) {
        height: 100%;
        width: 100%;
        margin: 0;
        padding: 0;
    }
</style>
