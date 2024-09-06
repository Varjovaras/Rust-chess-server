<script lang="ts">
	import type { PiecesEaten } from '$lib/types';

	export let eatenPieces: PiecesEaten;

	$: console.log(eatenPieces);

	type WhitePieces = {
		white: {
			Pawn?: string | undefined;
			Knight?: string | undefined;
			Bishop?: string | undefined;
			Rook?: string | undefined;
			Queen?: string | undefined;
			King?: string | undefined;
			None?: 'None' | undefined;
		}[];
	};

	type BlackPieces = {
		black: {
			Pawn?: string | undefined;
			Knight?: string | undefined;
			Bishop?: string | undefined;
			Rook?: string | undefined;
			Queen?: string | undefined;
			King?: string | undefined;
			None?: 'None' | undefined;
		}[];
	};

	type WhiteOrBlackPieces = WhitePieces | BlackPieces;

	type EatenPieces = {
		white: {
			pawns: number;
			knights: number;
			bishops: number;
			rooks: number;
			queens: number;
			kings: number;
		};
		black: {
			pawns: number;
			knights: number;
			bishops: number;
			rooks: number;
			queens: number;
			kings: number;
		};
	};

	function isWhitePieces(
		eatenPieces: WhiteOrBlackPieces,
	): eatenPieces is WhitePieces {
		return (eatenPieces as WhitePieces).white !== undefined;
	}

	function isBlackPieces(
		eatenPieces: WhiteOrBlackPieces,
	): eatenPieces is BlackPieces {
		return (eatenPieces as BlackPieces).black !== undefined;
	}

	const countEatenPieces = (eatenPieces: WhiteOrBlackPieces): EatenPieces => {
		const eatenWhitePieces = {
			pawns: 0,
			knights: 0,
			bishops: 0,
			rooks: 0,
			queens: 0,
			kings: 0,
		};

		const eatenBlackPieces = {
			pawns: 0,
			knights: 0,
			bishops: 0,
			rooks: 0,
			queens: 0,
			kings: 0,
		};

		if (isWhitePieces(eatenPieces)) {
			for (const piece of eatenPieces.white) {
				if (piece.Pawn) eatenWhitePieces.pawns++;
				if (piece.Knight) eatenWhitePieces.knights++;
				if (piece.Bishop) eatenWhitePieces.bishops++;
				if (piece.Rook) eatenWhitePieces.rooks++;
				if (piece.Queen) eatenWhitePieces.queens++;
				if (piece.King) eatenWhitePieces.kings++;
			}
		}
		if (isBlackPieces(eatenPieces)) {
			for (const piece of eatenPieces.black) {
				if (piece.Pawn) eatenBlackPieces.pawns++;
				if (piece.Knight) eatenBlackPieces.knights++;
				if (piece.Bishop) eatenBlackPieces.bishops++;
				if (piece.Rook) eatenBlackPieces.rooks++;
				if (piece.Queen) eatenBlackPieces.queens++;
				if (piece.King) eatenBlackPieces.kings++;
			}
		}

		return {
			white: eatenWhitePieces,
			black: eatenBlackPieces,
		};
	};

	$: whitePieces = countEatenPieces(eatenPieces).white;
	$: blackPieces = countEatenPieces(eatenPieces).black;
</script>

<h2 class="text-sm h2 flex justify-center">Eaten pieces</h2>
<div class="flex flex-row justify-center text-sm p-2">
	<div class="flex flex-col items-start mr-8">
		<h3 class="h3">White</h3>
		<ul>
			<li>Pawns: {whitePieces.pawns}</li>
			<li>Knights: {whitePieces.knights}</li>
			<li>Bishops: {whitePieces.bishops}</li>
			<li>Rooks: {whitePieces.rooks}</li>
			<li>Queens: {whitePieces.queens}</li>
			<!-- <li>Kings: {whitePieces.kings}</li> -->
		</ul>
	</div>

	<div class="flex flex-col items-start">
		<h3 class="h3">Black</h3>
		<ul>
			<li>Pawns: {blackPieces.pawns}</li>
			<li>Knights: {blackPieces.knights}</li>
			<li>Bishops: {blackPieces.bishops}</li>
			<li>Rooks: {blackPieces.rooks}</li>
			<li>Queens: {blackPieces.queens}</li>
			<!-- <li>Kings: {blackPieces.kings}</li> -->
		</ul>
	</div>
</div>
<!-- {#each blackPieces as piece}
	<div>
		{piece}
	</div>
{/each} -->
