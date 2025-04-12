import type {
    Chess,
    ChessBoard,
    Piece,
    PossibleMoves,
    Square,
} from "../../types";

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

export const isMoveLegal = (startSq: Square, endSq: Square) => {
    const endSqFileCoordinate = endFileAsCoordinate(endSq.file);
    if (!endSqFileCoordinate) return false;
    const endSqCoordinates: [number, number] = [
        endSqFileCoordinate,
        endSq.rank - 1,
    ];

    const moveExists = startSq.possible_moves.some(
        (move) => move[1] === endSqCoordinates,
    );

    return moveExists;
};

export const isPossibleToMovePiece = (
    startSq: Square,
    whiteTurn: boolean,
): boolean => {
    const color = whiteTurn ? "White" : "Black";
    if (!startSq.piece || startSq.piece === "None") {
        console.log("No piece on square");
        return false;
    }
    if (!Object.values(startSq.piece).includes(color)) {
        console.log(
            `Piece on square ${startSq.file}${startSq.rank} is not color ${color}`,
        );
        return false;
    }

    return true;
};

const endFileAsCoordinate = (endSqFile: string): number | null => {
    switch (endSqFile.toLowerCase()) {
        case "a":
            return 0;
        case "b":
            return 1;
        case "c":
            return 2;
        case "d":
            return 3;
        case "e":
            return 4;
        case "f":
            return 5;
        case "g":
            return 6;
        case "h":
            return 7;
        default:
            return null;
    }
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

export const isInPossibleMoves = (
    fromSq: Square,
    toSq: Square,
    possibleMoves: PossibleMoves,
): boolean => {
    const toCoord: [number, number] = [
        toSq.file.charCodeAt(0) - 65, // Convert 'A'-'H' to 0-7
        toSq.rank,
    ];

    return possibleMoves.some(
        (move) => move[1][0] === toCoord[0] && move[1][1] === toCoord[1],
    );
};

export function isValidMove(
    toSq: Square,
    possibleMoves: PossibleMoves,
): boolean {
    // Convert to coordinate format
    const toFile = Number(toSq.file);
    const toRank = toSq.rank;

    // Check if move exists in possible moves
    return possibleMoves.some(
        (move) => move[1][0] === toFile && move[1][1] === toRank,
    );
}
