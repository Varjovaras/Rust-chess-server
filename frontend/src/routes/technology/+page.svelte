<script lang="ts">
	import RustLogo from '$lib/components/rustLogo.svelte';
</script>

<section class="flex flex-col items-center justify-center min-h-screen p-4">
	<div class="text-center mb-8">
		<h1 class="text-2xl font-bold mb-4">Built with Rust and SvelteKit</h1>
		<p class="text-lg">
			Backend: Rust | Frontend: SvelteKit, TypeScript, TailwindCSS
		</p>
		<p class="text-lg flex items-center justify-center mt-2">
			Deployed on Vercel
			<svg
				data-testid="geist-icon"
				height="16"
				stroke-linejoin="round"
				viewBox="0 0 16 16"
				width="16"
				class="ml-1"
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

	<div class=" shadow-md rounded-lg p-6 w-full max-w-md">
		<h2 class="text-xl font-semibold mb-4">Todo:</h2>
		<ul class="list-disc list-inside text-left text-sm">
			<li class="mb-2">
				Improve the smoothness of making a move by utilizing a WebSocket
				connection instead of POST requests.
			</li>
			<li class="mb-2">Allow playing versus other players</li>
			<li class="mb-2">Make dragging a piece on mobile smoother</li>
			<li class="mb-2">Make a game analysis tool using stockfish</li>
		</ul>
	</div>

	<div class="flex items-center justify-center mt-8 space-x-4">
		<RustLogo />
		<img
			src="/Tailwind_CSS_Logo.svg"
			alt="Tailwind CSS logo"
			class="w-12 h-12"
		/>
		<img
			src="/Typescript_logo_2020.svg"
			alt="TypeScript logo"
			class="w-12 h-12"
		/>
	</div>
</section>
