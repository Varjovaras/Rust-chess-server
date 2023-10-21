<script lang="ts">
	import Square from './square.svelte';
	import type { ChessBoard, Piece } from './types';
	export let board: ChessBoard;
	let chessboard: ChessBoard = [[]];
	for (let i = 7; i >= 0; i--) {
		let arr = [];
		for (let j = 7; j >= 0; j--) {
			arr.push(board[j][i]);
		}
		chessboard.push(arr.reverse());
	}

	const pieceSwitch = (piece: Piece) => {
		switch (true) {
			case piece.Pawn !== undefined:
				return `P${piece.Pawn}`;
			case piece.Rook !== undefined:
				return `R${piece.Rook}`;
			case piece.Knight !== undefined:
				return `N${piece.Knight}`;
			case piece.Bishop !== undefined:
				return `B${piece.Bishop}`;
			case piece.Queen !== undefined:
				return `Q${piece.Queen}`;
			case piece.King !== undefined:
				return `K${piece.King}`;
			default:
				return '';
		}
	};
</script>

<div class="flex justify-center">
	<div class="grid grid-cols-8 gap-0">
		{#each chessboard as row}
			<!-- <p> -->
			{#each row as sq}
				{#if sq.color === 'White'}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-200 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						{sq.file}
						{sq.rank[0]}
						<Square {sq} />
					</button>
				{:else}
					<button
						class="lg:h-18 lg:w-18 h-11 w-11 bg-gray-400 text-center hover:bg-cyan-200 hover:text-base focus:bg-teal-500 sm:h-16 sm:w-16"
					>
						{sq.file}
						{sq.rank[0]}
						<Square {sq} />
					</button>
				{/if}
			{/each}
			<!-- </p> -->
		{/each}
	</div>
</div>
