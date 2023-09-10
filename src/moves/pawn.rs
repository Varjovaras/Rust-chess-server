use crate::{
    board::{rank::Rank, square::Square, ChessBoard},
    piece::PieceColor,
};

pub fn move_white_pawn(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    board: &ChessBoard,
) -> bool {
    if start_sq.rank == Rank::Second {
        return white_starting_sq_move(color, start_sq, end_sq, board);
    }
    true
}

pub fn move_black_pawn(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    board: &ChessBoard,
) -> bool {
    true
}

fn white_starting_sq_move(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    board: &ChessBoard,
) -> bool {
    true
}

fn _black_starting_sq_move(
    color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    board: &ChessBoard,
) -> bool {
    true
}
