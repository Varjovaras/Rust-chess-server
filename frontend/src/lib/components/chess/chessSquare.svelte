<script lang="ts">
	import type { Chess, PossibleMoves, Square as SquareType } from "../../types";
	import Piece from "./piece.svelte";

	interface Props {
		chess: Chess;
		sq: SquareType;
		selectedButton: string | null;
		possibleMoves: PossibleMoves;
		handleClick: (sq: SquareType) => void;
		handleDragStart: (sq: SquareType, event: DragEvent) => void;
		handleDrop: (event: DragEvent) => void;
		handleTouchStart: (event: TouchEvent, sq: SquareType) => void;
		handleTouchMove: (event: TouchEvent) => void;
		handleTouchEnd: (event: TouchEvent) => void;
	}

	const {
		chess,
		sq,
		selectedButton,
		possibleMoves,
		handleClick,
		handleDragStart,
		handleDrop,
		handleTouchStart,
		handleTouchMove,
		handleTouchEnd,
	}: Props = $props();

	const squareId = $derived(`${sq.file.toLowerCase()}${sq.rank + 1}`);
	const isSelected = $derived(selectedButton === squareId);
	const isPossibleMove = $derived(
		possibleMoves.some(
			(move) =>
				move[1][0] === sq.file.charCodeAt(0) - 65 && move[1][1] === sq.rank,
		),
	);
	const isKingInCheck = $derived(
		typeof sq.piece === "object" &&
			sq.piece.King !== undefined &&
			((sq.piece.King === "White" && chess.players[0].in_check) ||
				(sq.piece.King === "Black" && chess.players[1].in_check)),
	);

	const squareColor = $derived(
		sq.color === "White" ? "bg-gray-200" : "bg-gray-400",
	);
	const hoverColor = $derived("hover:bg-gray-600");
	const checkColor = $derived(
		isKingInCheck ? "bg-red-800 hover:bg-red-900" : "",
	);

	const squareClass = $derived(`
		w-[6vw] h-[6vw]
    max-w-16 max-h-16
    min-w-10 min-h-10
    ${squareColor} ${hoverColor} ${checkColor}
    text-center flex items-center justify-center
    ${isSelected ? "selected" : ""}
    ${isPossibleMove ? "possible_move" : ""}
  `);
	const draggable = $derived(sq.piece !== "None");
</script>

<button
	class={squareClass}
	{draggable}
	ondragstart={(event) => handleDragStart(sq, event)}
	ondragover={(event) => {
		event.preventDefault(); // Add this line
	}}
	ondrop={handleDrop}
	id={`${squareId} ${sq.piece}`}
	onclick={() => handleClick(sq)}
	ontouchstart={(event) => handleTouchStart(event, sq)}
	ontouchmove={handleTouchMove}
	ontouchend={handleTouchEnd}
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
