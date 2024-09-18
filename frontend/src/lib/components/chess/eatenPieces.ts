type WhitePieces = {
	white: {
		Pawn?: string | undefined;
		Knight?: string | undefined;
		Bishop?: string | undefined;
		Rook?: string | undefined;
		Queen?: string | undefined;
		King?: string | undefined;
		None?: "None" | undefined;
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
		None?: "None" | undefined;
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

export const countEatenPieces = (
	eatenPieces: WhiteOrBlackPieces,
): EatenPieces => {
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
