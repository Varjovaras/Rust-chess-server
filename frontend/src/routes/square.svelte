<script lang="ts">
	import type { Piece, Square } from '../lib/types';
	export let sq: Square;

	const pieceSwitch = (piece: Piece | 'None') => {
		if (piece === 'None') return '';
		switch (true) {
			case piece.Pawn && piece.Pawn !== undefined:
				if (piece.Pawn === 'White') return '/pieces/white_pawn.svg';
				else return `/pieces/black_pawn.svg`;

			case piece.Rook !== undefined:
				if (piece.Rook === 'White') return '/pieces/white_rook.svg';
				else return `/pieces/black_rook.svg`;

			case piece.Knight !== undefined:
				if (piece.Knight === 'White') return '/pieces/white_knight.svg';
				else return `/pieces/black_knight.svg`;

			case piece.Bishop !== undefined:
				if (piece.Bishop === 'White') return '/pieces/white_bishop.svg';
				else return `/pieces/black_bishop.svg`;

			case piece.Queen !== undefined:
				if (piece.Queen === 'White') return '/pieces/white_queen.svg';
				else return `/pieces/black_queen.svg`;

			case piece.King !== undefined:
				if (piece.King === 'White') return '/pieces/white_king.svg';
				else return `/pieces/black_king.svg`;
			default:
				return '';
		}
	};
</script>

<div>
	{#if sq.piece === 'None'}
		<div id={sq.file.toLowerCase() + sq.rank} />
		<!-- {#if } -->
	{:else}
		<img
			class="flex justify-center items-center w-full h-full"
			src={pieceSwitch(sq.piece)}
			alt={''}
			draggable="true"
			id={sq.file.toLowerCase() + (sq.rank + 1)}
		/>
	{/if}
</div>
