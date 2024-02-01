<script lang="ts">
	import { startingPosition } from '$lib/startingPosition';
	import Chessboard from './chessboard.svelte';
	import { schema, type Chess, type Square } from '../lib/types';
	export let data;
	let fromSquare = '';
	let toSquare = '';
	let chess = startingPosition;
	let apiUrl = data.url;

	const handleClick = (sq: Square) => {
		if (fromSquare === '' && sq.piece === 'None') {
			return;
		}
		if (fromSquare === '') {
			fromSquare = sq.file.toLowerCase() + (sq.rank + 1);
			console.log(fromSquare);
		} else {
			toSquare = sq.file.toLowerCase() + (sq.rank + 1);
			console.log(toSquare);
			handleMove(fromSquare, toSquare);
		}
	};
	const handleMove = async (startSq: string, endSq: string) => {
		console.log(`Move from ${startSq} to ${endSq}`);
		try {
			const newChess = await fetchMove(startSq, endSq);
			chess = newChess;
			fromSquare = '';
			toSquare = '';
		} catch (error) {
			console.error(error);
		}
	};
	const fetchMove = async (startSq: string, endSq: string): Promise<Chess> => {
		const newMove = [startSq, endSq];
		console.log(newMove);
		const response = await fetch(`${apiUrl}/api/chess`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ list_of_moves: chess.list_of_moves, new_move: newMove })
		});
		const data = await response.json();
		const validatedChess = schema.parse(data.chess);
		return validatedChess;
	};
	const handleReset = async () => {
		try {
			console.log('Resetting board');
			const response = await fetch(`${apiUrl}/api/chess`);
			const newChess = await response.json();
			const validatedChess = schema.parse(newChess);
			chess = validatedChess;
		} catch (error) {
			console.error(error);
		}
	};
</script>

<div class="text-cyan-50 text-center text-lg font-semibold">
	<p class="mb-2">Move by clicking on a piece and then on the square you want to move it to.</p>
	<p>You can also drag and drop pieces.</p>
</div>
<div class="mt-8 bg-red-300">
	{#if chess.white_player.victory}
		<p>White won</p>
	{:else if chess.black_player.victory}
		<p>Black won</p>
	{/if}
</div>
<Chessboard {chess} {handleClick} {handleMove} />

<button
	on:click={handleReset}
	class="bg-cyan-50 my-4 hover:bg-gray-100 text-gray-900 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
>
	Reset board
</button>
<div class="text-cyan-50 text-center text-lg font-semibold">
	<h2>Built with Rust on the backend and Sveltekit, Typescript and Tailwindcss on the frontend</h2>
	<h2 class="">Not yet implemented features:</h2>
	<div>
		<p>Improving performance by</p>
	</div>
</div>
<!-- <form class="grid grid-cols-2 gap-4 mt-8">
		<label class="col-span-1 bg-red-300">''
			<span class="block">Move from:</span>
			<input type="text" class="block w-full" bind:value={fromSquare} />
		</label>
		<label class="col-span-1 bg-red-300">
			<span class="block">Move to:</span>
			<input type="text" class="block w-full" bind:value={toSquare} />
		</label>
	</form> -->
