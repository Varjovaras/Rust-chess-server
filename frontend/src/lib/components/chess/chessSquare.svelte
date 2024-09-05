<script lang="ts">
    import type {
        Chess,
        PossibleMoves,
        Square as SquareType,
    } from "../../types";
    import Piece from "./piece.svelte";

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
        typeof sq.piece === "object" &&
        sq.piece.King !== undefined &&
        ((sq.piece.King === "White" && chess.white_player.in_check) ||
            (sq.piece.King === "Black" && chess.black_player.in_check));

    $: squareColor = sq.color === "White" ? "bg-gray-200" : "bg-gray-400";
    $: hoverColor = "hover:bg-gray-600";
    $: checkColor = isKingInCheck ? "bg-red-800 hover:bg-red-900" : "";

    $: squareClass = `
    w-[11vw] h-[11vw]
    sm:w-[7vw] sm:h-[7vw]
    md:w-[5vw] md:h-[5vw]
    lg:w-[4vw] lg:h-[4vw]
    xl:w-[3.5vw] xl:h-[3.5vw]
    max-w-20 max-h-20
    min-w-8 min-h-8
    ${squareColor} ${hoverColor} ${checkColor}
    text-center flex items-center justify-center
    ${isSelected ? "selected" : ""}
    ${isPossibleMove ? "possible_move" : ""}
  `;
    $: draggable = sq.piece !== "None";
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
</style>
