<script lang="ts">
	import Chessboard from './chessboard.svelte';
	import { schema } from './types';
	export let data;

	let chess = data.post;
	$: listOfMoves = chess.list_of_moves;
	$: whiteInCheck = chess.white_player.in_check;
	$: blackInCheck = chess.black_player.in_check;
	$: whiteInCheckmate = chess.black_player.victory;
	$: blackInCheckmate = chess.white_player.victory;
	// $: {
	// 	console.log(chess);
	// }

	const handleReset = async () => {
		try {
			console.log('Resetting board');
			const response = await fetch(`https://chessbackend.shuttleapp.rs/api/chess`);
			const newChess = await response.json();
			// console.log(JSON.stringify(chess));
			const validatedChess = schema.parse(newChess);
			chess = validatedChess;
		} catch (error) {
			console.error(error);
		}
	};
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
	<Chessboard {chess} />

	<button
		on:click={handleReset}
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-5"
	>
		Reset board
	</button>
</div>
