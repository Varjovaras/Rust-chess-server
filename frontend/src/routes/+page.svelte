<script lang="ts">
	import Chessboard from './chessboard.svelte';
	export let data;

	$: chess = data.post;
	$: chessboard = data.post.board;
	$: whiteInCheck = data.post.white_player.in_check;
	$: blackInCheck = data.post.black_player.in_check;
	$: whiteInCheckmate = data.post.black_player.victory;
	$: blackInCheckmate = data.post.white_player.victory;
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-900">
	<h1
		class="mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white text-center"
	>
		Shakking and sniping
	</h1>
	<div class="mt-8">
		{#if blackInCheck}
			<p>White won</p>
		{:else if whiteInCheckmate}
			<p>Black won</p>
		{/if}
	</div>
	<Chessboard chessboard={chess.board} {whiteInCheck} {blackInCheck} />
	<p class="text-gray-900 dark:text-white">
		Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
	</p>
</div>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.900);
	}
</style>
