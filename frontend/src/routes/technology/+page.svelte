<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import RustLogo from '$lib/components/rustLogo.svelte';
	import { onMount } from 'svelte';

	let visible = $state(false);

	onMount(() => {
		visible = true;
	});
</script>

<section class="flex flex-col items-center justify-start p-8">
	{#if visible}
		<div
			in:fade={{ duration: 500 }}
			out:fade={{ duration: 300 }}
			class="text-center mb-8"
		>
			<h1 class="font-bold mb-4 h1">Built with Rust and SvelteKit</h1>
			<p class="text-xl text-primary-300">Backend: Rust</p>
			<p class="text-xl text-primary-300">
				Frontend: SvelteKit, TypeScript, TailwindCSS
			</p>
			<p
				class="text-xl flex items-center justify-center mt-2 text-secondary-200"
			>
				Deployed on Vercel
				<svg
					data-testid="geist-icon"
					height="20"
					stroke-linejoin="round"
					viewBox="0 0 16 16"
					width="20"
					class="ml-2"
					style="color: currentcolor;"
				>
					<path
						fill-rule="evenodd"
						clip-rule="evenodd"
						d="M8 1L16 15H0L8 1Z"
						fill="currentColor"
					></path>
				</svg>
			</p>
		</div>

		<div
			in:fly={{ y: 20, duration: 500, delay: 200 }}
			class="bg-white shadow-lg rounded-lg p-6 w-full max-w-md mb-8"
		>
			<h2 class="text-2xl font-semibold mb-4 text-gray-800">Todo:</h2>
			<ul class="list-disc list-inside text-left text-lg text-gray-600">
				<li class="mb-3">Allow playing versus other players</li>
				<li class="mb-3">Make dragging a piece smoother</li>
				<li class="mb-3">Make a game analysis tool using stockfish</li>
			</ul>
		</div>

		<div
			in:fly={{ y: 20, duration: 500, delay: 400 }}
			class="flex items-center justify-center space-x-8"
		>
			<div>
				<RustLogo />
			</div>
			<img
				src="/Tailwind_CSS_Logo.svg"
				alt="Tailwind CSS logo"
				class="w-16 h-16"
			/>
			<img
				src="/Typescript_logo_2020.svg"
				alt="TypeScript logo"
				class="w-16 h-16"
			/>
		</div>
	{/if}
</section>

<svelte:head>
	<title>Technology Stack - My Chess App</title>
</svelte:head>
