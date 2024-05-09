<script lang="ts">
	import Square from './square.svelte';
	import type { Chess, PossibleMoves, Square as SquareType } from '../../types';
	import {
		handleBoardToFront,
		isWhiteTurn,
		legalMove,
	} from '$lib/components/chess/utils';

	export let chess: Chess;
	export let handleMove: (startSq: string, endSq: string) => Promise<void>;

	let startSq = '';
	let selectedButton: string | null = null;
	let fromSquare = '';
	let toSquare = '';
	let possibleMoves: PossibleMoves = [];

	$: boardToFront = handleBoardToFront(chess.board);
	$: whiteTurn = isWhiteTurn(chess.turn_number);

	const handleClick = async (sq: SquareType) => {
		let file = sq.file.toLowerCase();
		let rank = sq.rank + 1;
		let squareId = file + rank;

		if (!fromSquare) {
			if (sq.piece === 'None' || !legalMove(sq, whiteTurn)) {
				console.log(
					sq.piece === 'None' ? `No piece on ${squareId}` : 'Wrong players turn'
				);
				resetSelection();
				return;
			}
			fromSquare = squareId;
			selectedButton = squareId;
			possibleMoves = sq.possible_moves;
		} else {
			toSquare = squareId;
			await handleMove(fromSquare, toSquare);
			resetSelection();
		}
	};

	const resetSelection = () => {
		fromSquare = '';
		toSquare = '';
		selectedButton = null;
		possibleMoves = [];
	};

	const handleDragStart = (sq: SquareType) => {
		if (sq.piece === 'None') {
			console.log(`No piece on ${sq.file}${sq.rank}`);
			return;
		}
		if (!legalMove(sq, whiteTurn)) {
			return;
		}
		let file = sq.file.toLowerCase();
		let rank = sq.rank + 1;
		let squareId = file + rank;
		selectedButton = squareId;
		possibleMoves = sq.possible_moves;
		startSq = squareId;
	};

	const handleDrop = (event: DragEvent) => {
		const targetElement = event.target as HTMLElement;
		handleMove(startSq, targetElement.id[0] + targetElement.id[1]);
		resetSelection();
	};

	const handleTouchStart = (event: TouchEvent, sq: SquareType) => {
		handleDragStart(sq);
	};

	type TouchPosition = {
		x: number;
		y: number;
	};

	let lastKnownTouchPosition: null | TouchPosition = null;

	const handleTouchMove = (event: TouchEvent) => {
		lastKnownTouchPosition = {
			x: event.touches[0].clientX,
			y: event.touches[0].clientY,
		};
	};
	const handleTouchEnd = (event: TouchEvent) => {
		if (lastKnownTouchPosition) {
			const targetElement = document.elementFromPoint(
				lastKnownTouchPosition.x,
				lastKnownTouchPosition.y
			) as HTMLElement;
			handleMove(startSq, targetElement.id[0] + targetElement.id[1]);
		}
		resetSelection();
		lastKnownTouchPosition = null;
		event.preventDefault();
	};
</script>

<div class="flex justify-center items-center pb-4">
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
						id={`${sq.file.toLowerCase()}${sq.rank + 1} white king in check`}
						on:click|preventDefault={() => handleClick(sq)}
						on:touchstart|preventDefault={(event) =>
							handleTouchStart(event, sq)}
						on:touchmove|preventDefault={handleTouchMove}
						on:touchend|preventDefault={handleTouchEnd}
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
						id={`${sq.file.toLowerCase()}${sq.rank + 1} black king in check`}
						on:click|preventDefault={() => handleClick(sq)}
						on:touchstart|preventDefault={(event) =>
							handleTouchStart(event, sq)}
						on:touchmove|preventDefault={handleTouchMove}
						on:touchend|preventDefault={handleTouchEnd}
					>
						<Square {sq} />
					</button>
				{:else if sq.color === 'White' && sq.piece !== 'None'}
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
						id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
						on:click|preventDefault={() => handleClick(sq)}
						on:touchstart|preventDefault={(event) =>
							handleTouchStart(event, sq)}
						on:touchmove|preventDefault={handleTouchMove}
						on:touchend|preventDefault={handleTouchEnd}
					>
						<Square {sq} />
					</button>
				{:else if sq.color === 'Black' && sq.piece !== 'None'}
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
						id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
						on:click|preventDefault={() => handleClick(sq)}
						on:touchstart|preventDefault={(event) =>
							handleTouchStart(event, sq)}
						on:touchmove|preventDefault={handleTouchMove}
						on:touchend|preventDefault={handleTouchEnd}
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
						draggable="false"
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
						on:click|preventDefault={() => handleClick(sq)}
						on:touchstart|preventDefault={(event) =>
							handleTouchStart(event, sq)}
						on:touchmove|preventDefault={handleTouchMove}
						on:touchend|preventDefault={handleTouchEnd}
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
						draggable="false"
						on:dragover|preventDefault
						on:drop|preventDefault={handleDrop}
						id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
						on:click|preventDefault={() => handleClick(sq)}
						on:touchstart|preventDefault={(event) =>
							handleTouchStart(event, sq)}
						on:touchmove|preventDefault={handleTouchMove}
						on:touchend|preventDefault={handleTouchEnd}
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
