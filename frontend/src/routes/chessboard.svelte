<script lang="ts">
	import type { ChessBoard } from './types';
	export let board: ChessBoard;
	let chessboard: ChessBoard = [[]];
	for (let i = 7; i >= 0; i--) {
		let arr = [];
		for (let j = 7; j >= 0; j--) {
			arr.push(board[j][i]);
		}
		chessboard.push(arr.reverse());
	}
</script>

{#each chessboard as row}
	<p>
		{#each row as sq}
			{sq.file} {sq.rank} {' '}
		{/each}
	</p>
{/each}

<style>
	/* styles go here */
</style>
