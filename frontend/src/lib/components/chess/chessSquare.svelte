<script lang="ts">
	import type { Chess, PossibleMoves, Square as SquareType } from '../../types';
	import Piece from './piece.svelte';

	export let chess: Chess;
	export let sq: SquareType;
	export let selectedButton: string | null;
	export let possibleMoves: PossibleMoves;
	export let handleClick: (sq: SquareType) => void;
	export let handleDragStart: (sq: SquareType) => void;
	export let handleDrop: (event: DragEvent) => void;
	export let handleTouchStart: (event: TouchEvent, sq: SquareType) => void;
	export let handleTouchMove: (event: TouchEvent) => void;
	export let handleTouchEnd: (event: TouchEvent) => void;

	$: squareId = `${sq.file.toLowerCase()}${sq.rank + 1}`;
	$: isSelected = selectedButton === squareId;
	$: isPossibleMove = possibleMoves.some(
		(move) =>
			move[1][0] === sq.file.charCodeAt(0) - 65 && move[1][1] === sq.rank,
	);
	$: isKingInCheck =
		typeof sq.piece === 'object' &&
		sq.piece.King !== undefined &&
		((sq.piece.King === 'White' && chess.white_player.in_check) ||
			(sq.piece.King === 'Black' && chess.black_player.in_check));

	$: squareColor = sq.color === 'White' ? 'bg-gray-200' : 'bg-gray-400';
	$: hoverColor = 'hover:bg-gray-600';
	$: checkColor = isKingInCheck ? 'bg-red-800 hover:bg-red-900' : '';

	$: squareClass = `
    w-[8vw] h-[8vw]
    sm:w-[10vw] sm:h-[10vw]
    md:w-[8vw] md:h-[8vw]
    lg:w-[6vw] lg:h-[6vw]
    xl:w-[5vw] xl:h-[5vw]
    max-w-24 max-h-24
    min-w-10 min-h-10
    aspect-square
    ${squareColor} ${hoverColor} ${checkColor}
    text-center flex items-center justify-center
    ${isSelected ? 'selected' : ''}
    ${isPossibleMove ? 'possible_move' : ''}
  `;

	$: draggable = sq.piece !== 'None';
</script>

<button
	class={squareClass}
	{draggable}
	on:dragstart={() => handleDragStart(sq)}
	on:dragover|preventDefault
	on:drop|preventDefault={handleDrop}
	id={`${squareId} ${sq.piece}`}
	on:click|preventDefault={() => handleClick(sq)}
	on:touchstart|passive={(event) => handleTouchStart(event, sq)}
	on:touchmove|preventDefault={handleTouchMove}
	on:touchend|passive={handleTouchEnd}
>
	<Piece {sq} />
</button>

<style>
	.selected {
		@apply bg-cyan-800;
	}
	.possible_move {
		@apply bg-cyan-900;
	}
	button {
		aspect-ratio: 1 / 1;
	}
</style>
