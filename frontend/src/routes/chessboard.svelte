<script lang="ts">
	import Square from './square.svelte';
	import type { ChessBoard } from './types';
	export let chessboard: ChessBoard;
	export let whiteInCheck: boolean;
	export let blackInCheck: boolean;

	$: boardToFront = handleBoardToFront();

	const handleBoardToFront = (): ChessBoard => {
		let boardToFront: ChessBoard = [[]];
		for (let i = 7; i >= 0; i--) {
			let arr = [];
			for (let j = 7; j >= 0; j--) {
				arr.push(chessboard[j][i]);
			}
			boardToFront.push(arr.reverse());
		}
		return boardToFront;
	};
</script>

<div class="flex justify-center items-center">
	<div class="grid grid-cols-8 gap-0">
		{#each chessboard as row}
			<!-- <p> -->
			{#each row as sq}
				{#if sq.color === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-200 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						<Square {sq} {whiteInCheck} {blackInCheck} />
					</button>
				{:else}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-400 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						<Square {sq} {whiteInCheck} {blackInCheck} />
					</button>
				{/if}
			{/each}
			<!-- </p> -->
		{/each}
	</div>
</div>
