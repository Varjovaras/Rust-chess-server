<script lang="ts">
	import Chessboard from './chessboard.svelte';
	import { schema, type Chess, type Square } from './types';
	export let data;
	let fromSquare = '';
	let toSquare = '';
	let chess = data.post;
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
			handleSubmit();
		}
	};

	const handleSubmit = () => {
		console.log(`Move from ${fromSquare} to ${toSquare}`);
		handleMove();
		fromSquare = '';
		toSquare = '';
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

	const handleMove = async () => {
		try {
			const newChess = await fetchMove();
			chess = newChess;
		} catch (error) {
			console.error(error);
		}
	};

	const fetchMove = async (): Promise<Chess> => {
		const newMove = [fromSquare, toSquare];
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
</script>

<div class="flex min-h-screen flex-col items-center justify-center bg-gray-900">
	<h1
		class="underline mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white text-center"
	>
		Chess app
	</h1>
	<div class="mt-8 bg-red-300">
		{#if chess.white_player.victory}
			<p>White won</p>
		{:else if chess.black_player.victory}
			<p>Black won</p>
		{/if}
	</div>
	<Chessboard {chess} {handleClick} />

	<button
		on:click={handleReset}
		class="bg-gray-200 hover:bg-gray-100 text-gray-900 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
	>
		Reset board
	</button>
	<!-- <form class="grid grid-cols-2 gap-4 mt-8">
		<label class="col-span-1 bg-red-300">
			<span class="block">Move from:</span>
			<input type="text" class="block w-full" bind:value={fromSquare} />
		</label>
		<label class="col-span-1 bg-red-300">
			<span class="block">Move to:</span>
			<input type="text" class="block w-full" bind:value={toSquare} />
		</label>
	</form> -->
	<div class="text-2xl font-bold text-center text-blue-500">pasa</div>

	<footer class="mt-5">
		<a
			href="https://github.com/varjovaras"
			target="_blank"
			rel="noopener noreferrer"
			class="text-blue-500 hover:text-blue-800"
		>
			<img src="/github-mark.png" alt="GitHub" class="w-8 h-8" />
		</a>
	</footer>
</div>
