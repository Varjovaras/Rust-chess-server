<script lang="ts">
	import Chessboard from './chessboard.svelte';
	import { schema, type Chess } from './types';
	export let data;

	$: chess = data.post;
	$: chessboard = chess.board;
	$: list_of_moves = chess.list_of_moves;
	$: whiteInCheck = chess.white_player.in_check;
	$: blackInCheck = chess.black_player.in_check;
	$: whiteInCheckmate = chess.black_player.victory;
	$: blackInCheckmate = chess.white_player.victory;

	let fromSquare = '';
	let toSquare = '';
	let fromInput;

	async function makeMove(): Promise<Chess> {
		const new_move = [fromSquare, toSquare];
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

	function handleSubmit(event: { preventDefault: () => void }) {
		event.preventDefault();
		console.log(`Move from ${fromSquare} to ${toSquare}`);
		handleClick();
		fromSquare = '';
		toSquare = '';
		fromInput.focus();
	}
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-900">
	<h1
		class="underline mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white text-center"
	>
		Shakking and sniping
	</h1>
	<div class="mt-8 bg-red-300">
		{#if blackInCheckmate}
			<p>White won</p>
		{:else if whiteInCheckmate}
			<p>Black won</p>
		{/if}
	</div>
	<Chessboard {chessboard} {whiteInCheck} {blackInCheck} />

	<form class="grid grid-cols-2 gap-4 mt-8" on:submit={handleSubmit}>
		<label class="col-span-1 bg-red-300">
			<span class="block">Move from:</span>
			<input type="text" class="block w-full" bind:value={fromSquare} bind:this={fromInput} />
		</label>
		<label class="col-span-1 bg-red-300">
			<span class="block">Move to:</span>
			<input type="text" class="block w-full" bind:value={toSquare} />
		</label>
		<button
			type="submit"
			class="col-span-2 bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded"
		>
			Move
		</button>
	</form>
</div>
