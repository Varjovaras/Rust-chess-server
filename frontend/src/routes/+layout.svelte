<script lang="ts">
    import Footer from "$lib/components/footer.svelte";
    import Header from "$lib/components/header.svelte";
    import "../app.css";
    import { Modal, initializeStores } from "@skeletonlabs/skeleton";
    interface Props {
        children?: import('svelte').Snippet;
    }

    let { children }: Props = $props();

    initializeStores();
</script>

<svelte:head>
    <title>Chess App</title>
    <meta name="description" content="Chess portfolio simulator" />
</svelte:head>

<Modal />

<div class="flex flex-col h-screen w-screen">
    <Header />
    <main class="flex-1 overflow-auto content-center">
        {@render children?.()}
    </main>
    <Footer />
</div>

<style>
    :global(html, body) {
        height: 100%;
        width: 100%;
    }
</style>
