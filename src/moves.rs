use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

mod bishop;
mod king;
mod knight;
mod move_helpers;
mod pawn;
mod queen;
mod rook;

pub fn pawn(start_sq: &Square, end_sq: &Square, chess: &Chess, color: &PieceColor) -> bool {
    match *color {
        PieceColor::White => pawn::white_pawn::move_white_pawn(start_sq, end_sq, chess),
        PieceColor::Black => pawn::black_pawn::move_black_pawn(start_sq, end_sq, chess),
        PieceColor::None => false,
    }
}

pub fn _knight(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    knight::_move_knight(start_sq, end_sq, chess)
}

pub fn _bishop(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}

pub fn _rook(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}

pub fn _queen(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}

pub fn _king(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}
