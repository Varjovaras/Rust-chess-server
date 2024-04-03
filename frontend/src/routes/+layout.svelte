<script>
	import '../app.css';
</script>

<svelte:head>
	<title>Chess App</title>
</svelte:head>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-900">
	<h1
		class=" mb-4 mt-4 text-4xl font-extrabold leading-none tracking-tight text-cyan-50 md:text-5xl lg:text-6xl dark:text-white text-center"
	>
		Chess app
	</h1>
	<h3 class="font-semibold text-cyan-200 my-4 text-center">
		Built with Rust on the backend and Sveltekit, Typescript and Tailwindcss on the frontend
	</h3>
	<slot />
	<footer class="mt-5">
		<div class="flex items-center text-gray-500 mt-2 pb-2">
			<a href="https://github.com/varjovaras" target="_blank" rel="noopener noreferrer" class="">
				<img src="/github-mark.png" alt="GitHub" class="w-8 h-8 mr-2" />
			</a>
			<p>Author: Kristjan Rajaniemi</p>
		</div>
	</footer>
</div>
