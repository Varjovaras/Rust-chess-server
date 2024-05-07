import type { ChessBoard } from "./types";

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

