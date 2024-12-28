<script lang="ts">
import { returnCorrectPieceColor } from "$lib/components/chess/utils";
import type { Square } from "$lib/types";

interface Props {
	// import { slide } from 'svelte/transition';
	sq: Square;
}

const { sq }: Props = $props();

const pieceSize = $derived(`
    w-[5vw] h-[5vw]
    sm:w-[10vw] sm:h-[10vw]
    min-w-6 min-h-6
    max-w-16 max-h-16
    aspect-square
  `);
</script>

<div class="w-full h-full flex items-center justify-center content-end">
	{#if sq.piece === 'None'}
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
