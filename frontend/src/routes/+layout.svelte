<script>
	import Footer from '$lib/components/footer.svelte';
	import Header from '$lib/components/header.svelte';
	import Title from '$lib/components/title.svelte';
	import '../app.css';
	import { initializeStores, Modal } from '@skeletonlabs/skeleton';

	initializeStores();
</script>

<svelte:head>
	<title>Chess App</title>
	<meta name="description" content="Chess portfolio simulator" />
</svelte:head>

<Modal />
<Header />
<div
	class="flex flex-col items-center justify-center space-y-4 my-4 max-w-screen-xl min-h-screen mx-auto overflow-auto flex-shrink-0"
>
	<slot />
</div>
<Footer />

<style>
	:global(body) {
		overscroll-behavior-y: none;
	}
	:global(html) {
		overflow-y: auto;
	}
</style>
