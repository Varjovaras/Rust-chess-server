<script>
	import Footer from '$lib/components/footer.svelte';
	import Title from '$lib/components/title.svelte';
	import '../app.css';
</script>

<svelte:head>
	<title>Chess App</title>
</svelte:head>

<div
	class="flex min-h-screen flex-col items-center justify-center bg-gray-900 p-4"
>
	<meta name="description" content="Chess portfolio simulator" />
	<Title />
	<slot />
	<Footer />
</div>
