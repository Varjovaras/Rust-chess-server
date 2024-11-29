<script lang="ts">
	import Footer from "$lib/components/footer.svelte";
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

<div class="min-h-screen flex flex-col pb-16 overflow-hidden">
	<Header />

	<main class="w-full flex-1 overflow-auto">
		{@render children?.()}
	</main>
	<Footer />
</div>

<style>
	:global(html, body) {
		height: 100%;
		width: 100%;
		margin: 0;
		padding: 0;
	}

	main {
		padding-bottom: 60px; /* Adjust this value based on the height of your footer */
	}
</style>
