use crate::{
    board::{square::Square, ChessBoard},
    piece::PieceColor,
};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn pawn(color: &PieceColor, start_sq: &Square, end_sq: &Square, board: &ChessBoard) -> bool {
    match *color == PieceColor::White {
        true => pawn::move_white_pawn(color, start_sq, end_sq, board),
        false => pawn::move_black_pawn(color, start_sq, end_sq, board),
    }
}

pub fn _knight(color: &PieceColor, start_sq: &Square, end_sq: &Square, board: &ChessBoard) -> bool {
    knight::_move_knight(color, start_sq, end_sq, board)
}

pub fn _bishop(color: &PieceColor, start_sq: &Square, end_sq: &Square, board: &ChessBoard) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _rook(color: &PieceColor, start_sq: &Square, end_sq: &Square, board: &ChessBoard) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _queen(color: &PieceColor, start_sq: &Square, end_sq: &Square, board: &ChessBoard) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _king(color: &PieceColor, start_sq: &Square, end_sq: &Square, board: &ChessBoard) -> bool {
    // pawn::move_pawn();
    true
}
