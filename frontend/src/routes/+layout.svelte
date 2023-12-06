<script>
	import '../app.css';
</script>

<svelte:head>
	<title>Chess</title>
</svelte:head>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-900">
	<h1
		class=" mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white text-center"
	>
		Chess app
	</h1>
	<slot />
	<footer class="mt-5">
		<a
			href="https://github.com/varjovaras"
			target="_blank"
			rel="noopener noreferrer"
			class="text-blue-500 hover:text-blue-800"
		>
			<img src="/github-mark.png" alt="GitHub" class="w-8 h-8" />
		</a>
		<p class="text-gray-500 mt-2">Author: Kristjan Rajaniemi</p>
	</footer>
</div>
