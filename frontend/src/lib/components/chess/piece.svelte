<script lang="ts">
	import { returnCorrectPieceColor } from '$lib/components/chess/utils';
	import type { Square } from '$lib/types';
	export let sq: Square;
</script>

<div>
	{#if sq.piece === 'None'}
		<div id={`${sq.file.toLowerCase()}${sq.rank + 1} no piece`} />
	{:else}
		<img
			class="mx-auto block max-w-full w-full h-full"
			src={returnCorrectPieceColor(sq.piece)}
			alt={'square with a piece'}
			draggable="true"
			id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
		/>
	{/if}
</div>
