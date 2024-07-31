<script lang="ts">
  import type { PossibleMoves, Square } from "$lib/types";
  import Piece from "./piece.svelte";
  type Coordinates = {
    i: number;
    j: number;
  };

  export let sq: Square;
  export let selectedButton: string | null;
  export let possibleMoves: PossibleMoves;
  export let handleDragStart: (sq: Square) => void;
  export let handleDrop: (event: DragEvent) => void;
  export let handleClick: (sq: Square) => Promise<void>;
  export let handleTouchStart: (event: TouchEvent, sq: Square) => void;
  export let handleTouchMove: (event: TouchEvent) => void;
  export let handleTouchEnd: (event: TouchEvent) => void;
  export let buttonClass: string;
  export const draggable = true;
  export let coordinates: Coordinates;
</script>

<button
  class={buttonClass}
  class:selected={selectedButton === sq.file.toLowerCase() + (sq.rank + 1)}
  class:possible_move={possibleMoves.some(
    (move) => move[1][0] === coordinates.j && move[1][1] === 8 - coordinates.i
  )}
  {draggable}
  on:dragstart={() => handleDragStart(sq)}
  on:dragover|preventDefault
  on:drop|preventDefault={handleDrop}
  id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
  on:click|preventDefault={() => handleClick(sq)}
  on:touchstart|preventDefault={(event) => handleTouchStart(event, sq)}
  on:touchmove|preventDefault={handleTouchMove}
  on:touchend|preventDefault={handleTouchEnd}
>
  <Piece {sq} />
</button>
