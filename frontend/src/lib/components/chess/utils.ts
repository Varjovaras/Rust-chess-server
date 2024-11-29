import type { Chess, ChessBoard, Piece, Square } from "../../types";

export const handleBoardToFront = (chessboard: ChessBoard): ChessBoard => {
	const boardToFront: ChessBoard = [[]];
	for (let i = 7; i >= 0; i--) {
		const arr = [];
		for (let j = 7; j >= 0; j--) {
			arr.push(chessboard[j][i]);
		}
		boardToFront.push(arr.toReversed());
	}
	return boardToFront;
};

export const isWhiteTurn = (turnNumber: number): boolean => {
	return turnNumber % 2 === 0;
};
export const legalMove = (sq: Square, whiteTurn: boolean): boolean => {
	const color = whiteTurn ? "White" : "Black";
	return sq.piece && Object.values(sq.piece).includes(color);
};

export const returnCorrectPieceColor = (piece: Piece | "None") => {
	if (piece === "None") return "";
	switch (true) {
		case piece.Pawn !== undefined:
			return piece.Pawn === "White"
				? "/pieces/white_pawn.svg"
				: "/pieces/black_pawn.svg";

		case piece.Rook !== undefined:
			return piece.Rook === "White"
				? "/pieces/white_rook.svg"
				: "/pieces/black_rook.svg";

		case piece.Knight !== undefined:
			return piece.Knight === "White"
				? "/pieces/white_knight.svg"
				: "/pieces/black_knight.svg";

		case piece.Bishop !== undefined:
			return piece.Bishop === "White"
				? "/pieces/white_bishop.svg"
				: "/pieces/black_bishop.svg";

		case piece.Queen !== undefined:
			return piece.Queen === "White"
				? "/pieces/white_queen.svg"
				: "/pieces/black_queen.svg";

		case piece.King !== undefined:
			return piece.King === "White"
				? "/pieces/white_king.svg"
				: "/pieces/black_king.svg";
		default:
			return "";
	}
};

export const nameOfPiece = (piece: Piece): string => {
	if (piece.Pawn) return "Pawn";
	if (piece.Knight) return "Knight";
	if (piece.Bishop) return "Bishop";
	if (piece.Rook) return "Rook";
	if (piece.Queen) return "Queen";
	if (piece.King) return "King";
	if (piece.None) return "None";
	return "Unknown";
};

export const getSquareFromString = (
	square: string,
	chess: Chess,
): Square | null => {
	const sq = chess.board
		.flat()
		.find(
			(sq) =>
				sq.file === square[0].toUpperCase() &&
				sq.rank === Number(square[1]) - 1,
		);
	return sq || null;
};

export const isPawnPromotion = (sq: Square | null, endSq: string): boolean => {
	return (
		sq?.rank !== undefined &&
		(endSq[1] === "8" || endSq[1] === "1") &&
		typeof sq.piece === "object" &&
		sq.piece.Pawn !== undefined
	);
};

export const getPromotionPiece = (
	rank: number,
	endRank: string,
): [number, number] => {
	if (rank === 6 && endRank === "8") {
		return [1, 0];
	}
	if (rank === 1 && endRank === "1") {
		return [1, 1];
	}
	return [0, 0];
};
