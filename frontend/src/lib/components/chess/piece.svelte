<script lang="ts">
    import { returnCorrectPieceColor } from "$lib/components/chess/utils";
    import type { Square } from "$lib/types";

    interface Props {
        // import { slide } from 'svelte/transition';
        sq: Square;
    }

    const { sq }: Props = $props();

    const pieceSize = $derived(`
    w-[7.5vw] h-[7.5vw]
    sm:w-[9.5vw] sm:h-[9.5vw]
    md:w-[7.5vw] md:h-[7.5vw]
    lg:w-[5.5vw] lg:h-[5.5vw]
    xl:w-[4.5vw] xl:h-[4.5vw]
    min-w-9 min-h-9
    max-w-23 max-h-23
    aspect-square
  `);
</script>

<div class="w-full h-full flex items-center justify-center">
    {#if sq.piece === "None"}
        <div
            id={`${sq.file.toLowerCase()}${sq.rank + 1} no piece`}
            class={pieceSize}
        ></div>
    {:else}
        <img
            class={`object-contain ${pieceSize}`}
            src={returnCorrectPieceColor(sq.piece)}
            alt={`${sq.piece} piece`}
            draggable="true"
            id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
            style="object-fit: contain; width: 100%; height: 100%;"
        />
    {/if}
</div>
