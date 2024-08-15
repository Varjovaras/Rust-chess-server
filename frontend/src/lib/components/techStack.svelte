<script lang="ts">
import RustLogo from "./rustLogo.svelte";
</script>

<div>
	<p>
		Built with Rust on the backend and Sveltekit, Typescript and Tailwindcss on
		the frontend
	</p>
	<p>
		Deployed on Vercel <svg
			data-testid="geist-icon"
			height="16"
			stroke-linejoin="round"
			viewBox="0 0 16 16"
			width="16"
			style="color: currentcolor;"
			><path
				fill-rule="evenodd"
				clip-rule="evenodd"
				d="M8 1L16 15H0L8 1Z"
				fill="currentColor"
			></path></svg
		>
	</p>
	<ul class="m-4">
		todo:
		<div class="text-xs">
			<li>
				Improve the smoothness of making a move by utilizing a WebSocket
				connection instead of POST requests.
			</li>
			<li>Allow playing versus other players</li>
			<li>Make dragging a piece on mobile smoother</li>
			<li>Make a game analysis tool using stockfish</li>
		</div>
	</ul>

	<RustLogo />
	<img src="/Tailwind_CSS_Logo.svg" alt="Tailwind css logo" class="w-4 h-4" />
	<img
		src="/Typescript_logo_2020.svg"
		alt="Tailwind css logo"
		class="w-4 h-4"
	/>
</div>
