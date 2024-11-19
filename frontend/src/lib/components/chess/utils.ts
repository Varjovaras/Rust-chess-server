import type { ChessBoard, Piece, Square } from "../../types";

export const handleBoardToFront = (chessboard: ChessBoard): ChessBoard => {
	const boardToFront: ChessBoard = [[]];
	for (let i = 7; i >= 0; i--) {
		const arr = [];
		for (let j = 7; j >= 0; j--) {
			arr.push(chessboard[j][i]);
		}
		boardToFront.push(arr.reverse());
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
