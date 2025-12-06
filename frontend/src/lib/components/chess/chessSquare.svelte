<script lang="ts">
    import type {
        Chess,
        PossibleMoves,
        Square as SquareType,
    } from "../../types";
    import Piece from "./piece.svelte";
    import {
        SQUARE_TRANSITION_DURATION,
        INVALID_MOVE_DURATION,
    } from "$lib/constants/animation";

    interface Props {
        chess: Chess;
        sq: SquareType;
        selectedButton: string | null;
        possibleMoves: PossibleMoves;
        animatingFrom: string;
        animatingTo: string;
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
        animatingFrom,
        animatingTo,
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

    const isAnimatingFrom = $derived(animatingFrom === squareId);
    const isAnimatingTo = $derived(animatingTo === squareId);

    const squareColor = $derived(
        sq.color === "White" ? "bg-gray-200" : "bg-gray-400",
    );
    const hoverColor = $derived("hover:bg-gray-600");
    const checkColor = $derived(
        isKingInCheck ? "bg-red-800 hover:bg-red-900" : "",
    );

    const squareClass = $derived(`
        w-[8vw] h-[8vw]
        sm:w-[6vw] sm:h-[6vw]
        md:w-[4.5vw] md:h-[4.5vw]
        min-w-8 min-h-8
        ${squareColor} ${hoverColor} ${checkColor}
        text-center flex items-center justify-center piece
        ${isSelected ? "selected" : ""}
        ${isPossibleMove ? "possible_move" : ""}
  `);

    const transitionStyle = `transition: all ${SQUARE_TRANSITION_DURATION}ms ease-in-out;`;
</script>

<button
    class={`
         square
         ${squareClass}
         ${isPossibleMove ? "possible-move" : ""}
     `}
    style={transitionStyle}
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
        <Piece
            {sq}
            {isAnimatingFrom}
            {isAnimatingTo}
            onDragStart={(event) => handleDragStart(sq, event)}
        />
    {:else}
        <div class="w-full h-full"></div>
    {/if}
</button>

<style>
    .dragging {
        cursor: grabbing;
    }

    .can-drag {
        cursor: grab;
    }

    .selected {
        @apply bg-cyan-800;
    }

    .possible_move {
        @apply bg-cyan-900;
    }

    :global(.invalid-move) {
        animation: shake var(--invalid-move-duration, 300ms)
            var(--invalid-move-easing, cubic-bezier(0.36, 0.07, 0.19, 0.97));
        background-color: rgb(153, 27, 27) !important;
        --invalid-move-duration: 300ms;
        --invalid-move-easing: cubic-bezier(0.36, 0.07, 0.19, 0.97);
    }

    @keyframes shake {
        0%,
        100% {
            transform: translateX(0);
        }
        10%,
        30%,
        50%,
        70%,
        90% {
            transform: translateX(-4px);
        }
        20%,
        40%,
        60%,
        80% {
            transform: translateX(4px);
        }
    }
</style>
