<script lang="ts">
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
				return 'P';
			case piece.Rook !== undefined:
				return 'R';
			case piece.Knight !== undefined:
				return 'N';
			case piece.Bishop !== undefined:
				return 'B';
			case piece.Queen !== undefined:
				return 'Q';
			case piece.King !== undefined:
				return 'K';
			default:
				return '';
		}
	};
</script>

{#each chessboard as row}
	<p>
		{#each row as sq}
			{sq.file}
			{sq.rank}
			{#if sq.piece !== 'None'}
				{pieceSwitch(sq.piece)}
				<!-- <img src={`./pieces/${sq.piece}.png`} alt="piece" /> -->
			{/if}
		{/each}
	</p>
{/each}

<style>
	/* styles go here */
</style>
