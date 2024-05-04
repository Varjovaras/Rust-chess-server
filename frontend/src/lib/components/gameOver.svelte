<script lang="ts">
	import type { Chess } from '$lib/types';

	export let chess: Chess;
</script>

<div class="p-4">
	{#if chess.white_player.victory}
		<h3 class="text-red-700 text-xl">White won</h3>
	{:else if chess.black_player.victory}
		<h3 class="text-red-700 text-xl">Black won</h3>
	{/if}
</div>
