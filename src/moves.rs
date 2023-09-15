use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

mod bishop;
mod king;
mod knight;
mod move_helpers;
mod pawn;
mod queen;
mod rook;

pub fn pawn(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &mut Chess) -> bool {
    match *color {
        PieceColor::White => pawn::move_white_pawn(start_sq, end_sq, chess),
        PieceColor::Black => pawn::move_black_pawn(start_sq, end_sq, chess),
    }
}

pub fn _knight(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    knight::_move_knight(color, start_sq, end_sq, chess)
}

pub fn _bishop(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _rook(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _queen(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    // pawn::move_pawn();
    true
}

pub fn _king(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    // pawn::move_pawn();
    true
}
