<script>
	import Footer from '$lib/components/footer.svelte';
	import Header from '$lib/components/header.svelte';
	import '../app.css';
	import { initializeStores, Modal } from '@skeletonlabs/skeleton';

	initializeStores();
</script>

<svelte:head>
	<title>Chess App</title>
	<meta name="description" content="Chess portfolio simulator" />
</svelte:head>

<Modal />

<div class="flex flex-col h-screen max-w-screen-xl w-screen">
	<Header />
	<main class="flex-1 overflow-hidden p-4">
		<slot />
	</main>
	<Footer />
</div>

<style>
	:global(html, body) {
		height: 100%;
		overflow: hidden;
	}
</style>
