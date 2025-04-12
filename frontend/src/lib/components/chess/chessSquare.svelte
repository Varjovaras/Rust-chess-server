<script lang="ts">
    import { animationStore } from "$lib/stores/animationStore";
    import type {
        Chess,
        PossibleMoves,
        Square as SquareType,
    } from "../../types";
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
                move[1][0] === sq.file.charCodeAt(0) - 65 &&
                move[1][1] === sq.rank,
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

    // Updated square sizing for better mobile display
    const squareClass = $derived(`
		w-[10vw] h-[10vw]
        sm:w-[8vw] sm:h-[8vw]
        md:w-[6vw] md:h-[6vw]
        min-w-10 min-h-10
        transition-all duration-100 ease-in-out
        ${squareColor} ${hoverColor} ${checkColor}
        text-center flex items-center justify-center piece
        ${isSelected ? "selected" : ""}
        ${isPossibleMove ? "possible_move" : ""}
  `);
    const draggable = $derived(sq.piece !== "None");

    const isAnimating = $derived(
        $animationStore.isAnimating &&
            ($animationStore.fromSquare === squareId ||
                $animationStore.toSquare === squareId),
    );
    const isSource = $derived($animationStore.fromSquare === squareId);
    const isDestination = $derived($animationStore.toSquare === squareId);
</script>

<button
    class={`
         square
         ${squareClass}
         ${isAnimating ? "piece-animating" : ""}
         ${isSource ? "piece-source" : ""}
         ${isDestination ? "piece-destination" : ""}
         ${isPossibleMove ? "possible-move" : ""}
     `}
    {draggable}
    ondragstart={(event) => handleDragStart(sq, event)}
    ondragover={(event) => event.preventDefault()}
    ondrop={handleDrop}
    id={squareId}
    onclick={() => handleClick(sq)}
    ontouchstart={(event) => {
        event.preventDefault();
        handleTouchStart(event, sq);
    }}
    ontouchmove={handleTouchMove}
    ontouchend={handleTouchEnd}
>
    {#if sq.piece !== "None"}
        <Piece {sq} {isAnimating} {isSource} />
    {:else}
        <!-- Empty placeholder to maintain layout -->
        <div class="w-full h-full"></div>
    {/if}
</button>

<style>
    .piece-animating {
        position: relative;
        z-index: 50;
    }

    .piece-source :global(.piece-image) {
        opacity: 0.3;
    }

    .piece-destination {
        z-index: 500;
    }

    /* Add these new classes */
    .dragging {
        cursor: grabbing;
    }

    .can-drag {
        cursor: grab;
    }
    .selected {
        @apply bg-cyan-800;
        transition: background-color 100ms cubic-bezier(0.4, 0, 0.2, 1);
    }

    .possible_move {
        @apply bg-cyan-900;
        transition: background-color 100ms cubic-bezier(0.4, 0, 0.2, 1);
    }
</style>
