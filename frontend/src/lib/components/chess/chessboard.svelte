<script lang="ts">
  import type { Chess, PossibleMoves, Square as SquareType } from "../../types";
  import {
    handleBoardToFront,
    isWhiteTurn,
    legalMove,
  } from "$lib/components/chess/utils";
  import Piece from "./piece.svelte";

  export let chess: Chess;
  export let handleMove: (startSq: string, endSq: string) => Promise<void>;

  let startSq = "";
  let selectedButton: string | null = null;
  let fromSquare = "";
  let toSquare = "";
  let possibleMoves: PossibleMoves = [];

  $: boardToFront = handleBoardToFront(chess.board);
  $: whiteTurn = isWhiteTurn(chess.turn_number);

  const handleClick = async (sq: SquareType) => {
    const file = sq.file.toLowerCase();
    const rank = sq.rank + 1;
    const squareId = file + rank;

    if (!fromSquare) {
      if (sq.piece === "None" || !legalMove(sq, whiteTurn)) {
        console.log(
          sq.piece === "None" ? `No piece on ${squareId}` : "Wrong players turn"
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
    fromSquare = "";
    toSquare = "";
    selectedButton = null;
    possibleMoves = [];
  };

  const handleDragStart = (sq: SquareType) => {
    if (sq.piece === "None") {
      console.log(`No piece on ${sq.file}${sq.rank}`);
      return;
    }
    if (!legalMove(sq, whiteTurn)) {
      return;
    }
    const file = sq.file.toLowerCase();
    const rank = sq.rank + 1;
    const squareId = file + rank;
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

  const handleTouchEnd = async (event: TouchEvent) => {
    if (lastKnownTouchPosition) {
      const targetElement = document.elementFromPoint(
        lastKnownTouchPosition.x,
        lastKnownTouchPosition.y
      ) as HTMLElement;
      const endSq = targetElement.id[0] + targetElement.id[1];
      if (endSq !== startSq) {
        await handleMove(startSq, endSq);
        resetSelection();
      }
    }
    lastKnownTouchPosition = null;
  };
</script>

<div class="flex justify-center items-center">
  <div class="grid grid-cols-8 gap-0">
    {#each boardToFront as row, i}
      {#each row as sq, j}
        {#if chess.white_player.in_check && typeof sq.piece === "object" && sq.piece.King !== undefined && sq.piece.King === "White"}
          <button
            class="h-11 w-11 sm:h-14 sm:w-14 md:h-16 md:w-16 lg:h-18 lg:w-18 xl:w-20 xl:h-20 bg-red-800 text-center hover:bg-red-900"
            class:selected={selectedButton ===
              sq.file.toLowerCase() + (sq.rank + 1)}
            draggable="true"
            on:dragstart={() => handleDragStart(sq)}
            on:dragover|preventDefault
            on:drop|preventDefault={handleDrop}
            id={`${sq.file.toLowerCase()}${sq.rank + 1} white king in check`}
            on:click|preventDefault={() => handleClick(sq)}
            on:touchstart|passive={(event) => handleTouchStart(event, sq)}
            on:touchmove|preventDefault={handleTouchMove}
            on:touchend|passive={handleTouchEnd}
          >
            <Piece {sq} />
          </button>
        {:else if chess.black_player.in_check && typeof sq.piece === "object" && sq.piece.King !== undefined && sq.piece.King === "Black"}
          <button
            class="h-11 w-11 sm:h-14 sm:w-14 md:h-16 md:w-16 lg:h-18 lg:w-18 xl:w-20 xl:h-20 bg-red-800 hover:bg-red-900 text-center"
            class:selected={selectedButton ===
              sq.file.toLowerCase() + (sq.rank + 1)}
            draggable="true"
            on:dragstart={() => handleDragStart(sq)}
            on:dragover|preventDefault
            on:drop={handleDrop}
            id={`${sq.file.toLowerCase()}${sq.rank + 1} black king in check`}
            on:click|preventDefault={() => handleClick(sq)}
            on:touchstart|passive={(event) => handleTouchStart(event, sq)}
            on:touchmove|preventDefault={handleTouchMove}
            on:touchend|passive={handleTouchEnd}
          >
            <Piece {sq} />
          </button>
        {:else if sq.color === "White" && sq.piece !== "None"}
          <button
            class="h-11 w-11 sm:h-14 sm:w-14 md:h-16 md:w-16 lg:h-18 lg:w-18 xl:w-20 xl:h-20 bg-gray-200 text-center hover:bg-gray-600"
            class:selected={selectedButton ===
              sq.file.toLowerCase() + (sq.rank + 1)}
            class:possible_move={possibleMoves.some(
              (move) => move[1][0] === j && move[1][1] === 8 - i
            )}
            draggable="true"
            on:dragstart={() => handleDragStart(sq)}
            on:dragover|preventDefault
            on:drop={handleDrop}
            id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
            on:click|preventDefault={() => handleClick(sq)}
            on:touchstart|passive={(event) => handleTouchStart(event, sq)}
            on:touchmove|preventDefault={handleTouchMove}
            on:touchend|passive={handleTouchEnd}
          >
            <Piece {sq} />
          </button>
        {:else if sq.color === "Black" && sq.piece !== "None"}
          <button
            class="h-11 w-11 sm:h-14 sm:w-14 md:h-16 md:w-16 lg:h-18 lg:w-18 xl:w-20 xl:h-20 bg-gray-400 text-center hover:bg-gray-600"
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
            on:touchstart|passive={(event) => handleTouchStart(event, sq)}
            on:touchmove|preventDefault={handleTouchMove}
            on:touchend|passive={handleTouchEnd}
          >
            <Piece {sq} />
          </button>
        {:else if sq.color === "White"}
          <button
            class="h-11 w-11 sm:h-14 sm:w-14 md:h-16 md:w-16 lg:h-18 lg:w-18 xl:w-20 xl:h-20 bg-gray-200 text-center hover:bg-gray-600"
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
            on:touchstart|passive={(event) => handleTouchStart(event, sq)}
            on:touchmove|preventDefault={handleTouchMove}
            on:touchend|passive={handleTouchEnd}
          >
            <Piece {sq} />
          </button>
        {:else}
          <button
            class="h-11 w-11 sm:h-14 sm:w-14 md:h-16 md:w-16 lg:h-18 lg:w-18 xl:w-20 xl:h-20 bg-gray-400 text-center hover:bg-gray-600"
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
            on:touchstart|passive={(event) => handleTouchStart(event, sq)}
            on:touchmove|preventDefault={handleTouchMove}
            on:touchend|passive={handleTouchEnd}
          >
            <Piece {sq} />
          </button>
        {/if}
      {/each}
    {/each}
  </div>
</div>

<style>
  .selected {
    @apply bg-cyan-800;
  }
  .possible_move {
    @apply bg-cyan-900;
  }
</style>
