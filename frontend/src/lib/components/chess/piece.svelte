<script lang="ts">
	import { returnCorrectPieceColor } from '$lib/components/chess/utils';
	import type { Square } from '$lib/types';
	export let sq: Square;

	$: pieceSize = `
    w-[8vw] h-[8vw]
    sm:w-[10vw] sm:h-[10vw]
    md:w-[8vw] md:h-[8vw]
    lg:w-[6vw] lg:h-[6vw]
    xl:w-[5vw] xl:h-[5vw]
    min-w-10 min-h-10
    max-w-24 max-h-24
	aspect-square
  `;
</script>

<div class="w-full h-full flex items-center justify-center">
	{#if sq.piece === 'None'}
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
			style="object-fit: contain; width: 100%; height: 100%;"
		/>
	{/if}
</div>
