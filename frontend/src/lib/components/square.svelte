<script lang="ts">
	import type { Piece, Square } from '../types';
	export let sq: Square;

	const returnCorrectPieceColor = (piece: Piece | 'None') => {
		if (piece === 'None') return '';
		switch (true) {
			case piece.Pawn && piece.Pawn !== undefined:
				return piece.Pawn === 'White'
					? '/pieces/white_pawn.svg'
					: '/pieces/black_pawn.svg';

			case piece.Rook !== undefined:
				return piece.Rook === 'White'
					? '/pieces/white_rook.svg'
					: '/pieces/black_rook.svg';

			case piece.Knight !== undefined:
				return piece.Knight === 'White'
					? '/pieces/white_knight.svg'
					: '/pieces/black_knight.svg';

			case piece.Bishop !== undefined:
				return piece.Bishop === 'White'
					? '/pieces/white_bishop.svg'
					: '/pieces/black_bishop.svg';

			case piece.Queen !== undefined:
				return piece.Queen === 'White'
					? '/pieces/white_queen.svg'
					: '/pieces/black_queen.svg';

			case piece.King !== undefined:
				return piece.King === 'White'
					? '/pieces/white_king.svg'
					: '/pieces/black_king.svg';
			default:
				return '';
		}
	};
</script>

<div>
	{#if sq.piece === 'None'}
		<div id={`${sq.file.toLowerCase()}${sq.rank + 1} no piece`} />
	{:else}
		<img
			class="flex justify-center items-center w-full h-full"
			src={returnCorrectPieceColor(sq.piece)}
			alt={'square with a piece'}
			draggable="true"
			id={`${sq.file.toLowerCase()}${sq.rank + 1} ${sq.piece}`}
		/>
	{/if}
</div>
