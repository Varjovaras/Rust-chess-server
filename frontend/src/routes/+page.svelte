<script lang="ts">
	import Chessboard from './chessboard.svelte';
	import { schema, type Chess, listOfMovesSchema } from './types';
	export let data;

	$: chess = data.post;
	$: chessboard = chess.board;
	$: list_of_moves = chess.list_of_moves;
	$: whiteInCheck = chess.white_player.in_check;
	$: blackInCheck = chess.black_player.in_check;
	$: whiteInCheckmate = chess.black_player.victory;
	$: blackInCheckmate = chess.white_player.victory;

	async function makeMove(): Promise<Chess> {
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
		return validatedChess;
	}

	async function pas2(): Promise<Chess> {
		const new_move = ['e7', 'e5'];
		const response = await fetch('http://127.0.0.1:8000/chess', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ list_of_moves, new_move })
		});
		const data = await response.json();
		const validatedChess = schema.parse(data.chess);
		return validatedChess;
	}

	async function handleClick() {
		try {
			const newChess = await makeMove();
			chess = newChess;
		} catch (error) {
			console.error(error);
		}
	}

	async function pasaClick() {
		try {
			const newChess = await pas2();
			chess = newChess;
		} catch (error) {
			console.error(error);
		}
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
	<button class="bg-red-600" on:click={handleClick}>Click me!</button>
	<button class="bg-red-600" on:click={pasaClick}>pasa me!</button>
</div>
<!-- 
<style lang="postcss">
	:global(html) {
		background-color: theme(colors.red.100);
	}
</style> -->
