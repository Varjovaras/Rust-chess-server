<script lang="ts">
	import Square from './square.svelte';
	import type {
		Chess,
		ChessBoard,
		PossibleMoves,
		Square as SquareType,
	} from '../types';

	export let chess: Chess;
	export let handleMove: (startSq: string, endSq: string) => Promise<void>;

	let startSq = '';
	let selectedButton: string | null = null;
	let fromSquare = '';
	let toSquare = '';
	let possibleMoves: PossibleMoves = [];

	const handleBoardToFront = (chessboard: ChessBoard): ChessBoard => {
		const boardToFront: ChessBoard = [[]];
		for (let i = 7; i >= 0; i--) {
			const arr = [];
			for (let j = 7; j >= 0; j--) {
				arr.push(chessboard[j][i]);
			}
			boardToFront.push(arr.reverse());
		}
		return boardToFront;
	};

	const handleClick = async (sq: SquareType) => {
		if (fromSquare === '' && sq.piece === 'None') {
			selectedButton = null;
			possibleMoves = [];
			return;
		}
		let file = sq.file.toLowerCase();
		let rank = sq.rank + 1;
		if (fromSquare === '') {
			fromSquare = file + rank;
			selectedButton = file + rank;
			possibleMoves = sq.possible_moves;
		} else if (fromSquare !== '') {
			toSquare = file + rank;
			await handleMove(fromSquare, toSquare);
			fromSquare = '';
			toSquare = '';
			selectedButton = null;
			possibleMoves = [];
		}
	};

	// biome-ignore lint/suspicious/noConfusingLabels: <explanation>
	$: boardToFront = handleBoardToFront(chess.board);

	const handleDragStart = (sq: SquareType) => {
		let file = sq.file.toLowerCase();
		let rank = sq.rank + 1;
		selectedButton = file + rank;
		possibleMoves = sq.possible_moves;
		startSq = file + rank;
	};

	const handleDrop = (event: DragEvent) => {
		const targetElement = event.target as HTMLElement;

		if (targetElement.id !== '[object Object]' && targetElement.id !== '') {
			handleMove(startSq, targetElement.id);
			selectedButton = null;
		}
		selectedButton = null;
		possibleMoves = [];
	};

	// ...
</script>

<div class="flex justify-center items-center">
	<div class="grid grid-cols-8 gap-0">
		{#each boardToFront as row, i}
			{#each row as sq, j}
				{#if chess.white_player.in_check && typeof sq.piece === 'object' && sq.piece.King !== undefined && sq.piece.King === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-800 text-center sm:h-16 sm:w-16 hover:bg-red-900"
						class:selected={selectedButton ===
							sq.file.toLowerCase() + (sq.rank + 1)}
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file.toLowerCase() + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{:else if chess.black_player.in_check && typeof sq.piece === 'object' && sq.piece.King !== undefined && sq.piece.King === 'Black'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-red-800 hover:bg-red-900 text-center sm:h-16 sm:w-16"
						class:selected={selectedButton ===
							sq.file.toLowerCase() + (sq.rank + 1)}
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file.toLowerCase() + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{:else if sq.color === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-200 text-center sm:h-16 sm:w-16 hover:bg-gray-600"
						class:selected={selectedButton ===
							sq.file.toLowerCase() + (sq.rank + 1)}
						class:possible_move={possibleMoves.some(
							(move) => move[1][0] === j && move[1][1] === 8 - i
						)}
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file.toLowerCase() + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{:else}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-400 text-center sm:h-16 sm:w-16 hover:bg-gray-600"
						class:selected={selectedButton ===
							sq.file.toLowerCase() + (sq.rank + 1)}
						class:possible_move={possibleMoves.some(
							(move) => move[1][0] === j && move[1][1] === 8 - i
						)}
						draggable="true"
						on:dragstart={() => handleDragStart(sq)}
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={sq.file.toLowerCase() + (sq.rank + 1)}
						on:click|preventDefault={() => handleClick(sq)}
					>
						<Square {sq} />
					</button>
				{/if}
			{/each}
		{/each}
	</div>
</div>

<style>
	.selected {
		@apply bg-cyan-600;
	}
	.possible_move {
		@apply bg-red-900;
	}
</style>
