<script lang="ts">
	import Square from './square.svelte';
	import { schema, type Chess, type ChessBoard, type ListOfMoves } from './types';
	export let chess: Chess;

	let fromSquare = '';
	let toSquare = '';
	let fromInput: HTMLInputElement;

	$: boardToFront = handleBoardToFront(chess.board);

	const handleBoardToFront = (chessboard: ChessBoard): ChessBoard => {
		let boardToFront: ChessBoard = [[]];
		for (let i = 7; i >= 0; i--) {
			let arr = [];
			for (let j = 7; j >= 0; j--) {
				arr.push(chessboard[j][i]);
			}
			boardToFront.push(arr.reverse());
		}
		return boardToFront;
	};

	async function makeMove(): Promise<Chess> {
		const new_move = [fromSquare, toSquare];
		const response = await fetch('http://127.0.0.1:8000/chess', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ list_of_moves: chess.list_of_moves, new_move })
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

<div class="flex justify-center items-center">
	<div class="grid grid-cols-8 gap-0">
		{#each boardToFront as row}
			{#each row as sq}
				{#if chess.white_player.in_check && sq.color === 'White' && sq.piece !== 'None' && sq.piece.King === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-900 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						<Square {sq} />
					</button>
				{:else if chess.black_player.in_check && sq.piece !== 'None' && sq.piece.King === 'Black'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-900 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						<Square {sq} />
					</button>
				{:else if sq.color === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-200 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						<Square {sq} />
					</button>
				{:else}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-400 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						<Square {sq} />
					</button>
				{/if}
			{/each}
			<!-- </p> -->
		{/each}
	</div>
</div>

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
