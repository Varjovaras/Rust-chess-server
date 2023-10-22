<script lang="ts">
	import type { Piece, Square } from './types';
	export let sq: Square;
	export let whiteInCheck: boolean;
	export let blackInCheck: boolean;

	const pieceSwitch = (piece: Piece | 'None') => {
		if (piece === 'None') return '';
		switch (true) {
			case piece.Pawn && piece.Pawn !== undefined:
				if (piece.Pawn === 'White') return 'p';
				else return `P`;

			case piece.Rook !== undefined:
				if (piece.Rook === 'White') return 'r';
				else return `R`;
			case piece.Knight !== undefined:
				if (piece.Knight === 'White') return 'n';
				else return `N`;

			case piece.Bishop !== undefined:
				if (piece.Bishop === 'White') return 'b';
				else return `B`;
			case piece.Queen !== undefined:
				if (piece.Queen === 'White') return 'q';
				else return `Q`;
			case piece.King !== undefined:
				if (piece.King === 'White') return 'k';
				else return `K`;
			default:
				return '';
		}
	};
</script>

<div>
	{#if sq.piece === 'None'}
		<div />
	{:else}
		{@html pieceSwitch(sq.piece)}
	{/if}
</div>
