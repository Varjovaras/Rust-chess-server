<script lang="ts">
	import { onMount } from 'svelte';
	import Chessboard from './chessboard.svelte';
	import { schema } from './types';
	export let data;

	$: chess = data.post;
	$: chessboard = data.post.board;
	$: list_of_moves = data.post.list_of_moves;
	$: whiteInCheck = data.post.white_player.in_check;
	$: blackInCheck = data.post.black_player.in_check;
	$: whiteInCheckmate = data.post.black_player.victory;
	$: blackInCheckmate = data.post.white_player.victory;

	async function makeMove() {
		const new_move = ['e2', 'e4'];
		const response = await fetch('http://127.0.0.1:8000/chess', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ list_of_moves, new_move })
		});
		const data = await response.json();
		const validatedChess = schema.parse(data.chess);

		chess = validatedChess;
	}

	function handleClick() {
		makeMove();

		console.log('Button clicked!');
	}
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-900">
	<h1
		class="underline mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white text-center"
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
	<Chessboard {chessboard} {whiteInCheck} {blackInCheck} />
	<p class="text-gray-900 dark:text-white">
		Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
	</p>
	<button on:click={handleClick}>Click me!</button>
</div>
<!-- 
<style lang="postcss">
	:global(html) {
		background-color: theme(colors.red.100);
	}
</style> -->
