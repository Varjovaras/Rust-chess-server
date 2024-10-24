<script lang="ts">
	import { createBubbler, preventDefault, passive } from 'svelte/legacy';

	const bubble = createBubbler();
	import type { Chess, PossibleMoves, Square as SquareType } from '../../types';
	import Piece from './piece.svelte';

	interface Props {
		chess: Chess;
		sq: SquareType;
		selectedButton: string | null;
		possibleMoves: PossibleMoves;
		handleClick: (sq: SquareType) => void;
		handleDragStart: (sq: SquareType) => void;
		handleDrop: (event: DragEvent) => void;
		handleTouchStart: (event: TouchEvent, sq: SquareType) => void;
		handleTouchMove: (event: TouchEvent) => void;
		handleTouchEnd: (event: TouchEvent) => void;
	}

	let {
		chess,
		sq,
		selectedButton,
		possibleMoves,
		handleClick,
		handleDragStart,
		handleDrop,
		handleTouchStart,
		handleTouchMove,
		handleTouchEnd
	}: Props = $props();

	let squareId = $derived(`${sq.file.toLowerCase()}${sq.rank + 1}`);
	let isSelected = $derived(selectedButton === squareId);
	let isPossibleMove = $derived(possibleMoves.some(
		(move) =>
			move[1][0] === sq.file.charCodeAt(0) - 65 && move[1][1] === sq.rank,
	));
	let isKingInCheck =
		$derived(typeof sq.piece === 'object' &&
		sq.piece.King !== undefined &&
		((sq.piece.King === 'White' && chess.players[0].in_check) ||
			(sq.piece.King === 'Black' && chess.players[1].in_check)));

	let squareColor = $derived(sq.color === 'White' ? 'bg-gray-200' : 'bg-gray-400');
	let hoverColor = $derived('hover:bg-gray-600');
	let checkColor = $derived(isKingInCheck ? 'bg-red-800 hover:bg-red-900' : '');

	let squareClass = $derived(`
    w-[11vw] h-[11vw]
    sm:w-[7vw] sm:h-[7vw]
    md:w-[5vw] md:h-[5vw]
    lg:w-[6vw] lg:h-[6vw]
    xl:w-[3.5vw] xl:h-[3.5vw]
    max-w-20 max-h-20
    min-w-8 min-h-8
    ${squareColor} ${hoverColor} ${checkColor}
    text-center flex items-center justify-center
    ${isSelected ? 'selected' : ''}
    ${isPossibleMove ? 'possible_move' : ''}
  `);
	let draggable = $derived(sq.piece !== 'None');
</script>

<button
	class={squareClass}
	{draggable}
	ondragstart={() => handleDragStart(sq)}
	ondragover={preventDefault(bubble('dragover'))}
	ondrop={preventDefault(handleDrop)}
	id={`${squareId} ${sq.piece}`}
	onclick={preventDefault(() => handleClick(sq))}
	use:passive={['touchstart', () => (event) => handleTouchStart(event, sq)]}
	ontouchmove={preventDefault(handleTouchMove)}
	use:passive={['touchend', () => handleTouchEnd]}
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
</style>
