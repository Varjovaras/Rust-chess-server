<script>
	import Footer from '$lib/components/footer.svelte';
	import Title from '$lib/components/title.svelte';
	import '../app.css';
	// class="flex min-h-screen flex-col items-center justify-center bg-gray-900 p-4"
</script>

<svelte:head>
	<title>Chess App</title>
</svelte:head>

<div class="flex flex-col items-center justify-center p-4 font-mono">
	<meta name="description" content="Chess portfolio simulator" />
	<Title />
	<slot />
	<Footer />
</div>
