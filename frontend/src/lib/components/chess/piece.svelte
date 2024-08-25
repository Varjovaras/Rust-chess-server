<script lang="ts">
    import { returnCorrectPieceColor } from "$lib/components/chess/utils";
    import type { Square } from "$lib/types";
    export let sq: Square;

    $: pieceSize = `
  w-[11vw] h-[11vw]
  sm:w-[5vw] sm:h-[5vw]
  md:w-[4vw] md:h-[4vw]
  lg:w-[3vw] lg:h-[3vw]
  xl:w-[2.5vw] xl:h-[2.5vw]
  min-w-6 min-h-6
  max-w-16 max-h-16
`;
</script>

<div class="w-full h-full flex items-center justify-center">
    {#if sq.piece === "None"}
        <div
            id={`${sq.file.toLowerCase()}${sq.rank + 1} no piece`}
            class={pieceSize}
        />
    {:else}
        <img
            class={`object-contain ${pieceSize}`}
            src={returnCorrectPieceColor(sq.piece)}
            alt={`${sq.piece} piece`}
            draggable="true"
            id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
        />
    {/if}
</div>
