use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

mod bishop;
mod king;
mod knight;
mod move_helpers;
mod pawn;
mod queen;
mod rook;

pub fn pawn(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    match *color {
        PieceColor::White => pawn::white_pawn::move_white_pawn(start_sq, end_sq, chess),
        PieceColor::Black => pawn::black_pawn::move_black_pawn(start_sq, end_sq, chess),
    }
}

pub fn _knight(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    knight::_move_knight(color, start_sq, end_sq, chess)
}

pub fn _bishop(_color: &PieceColor, _start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}

pub fn _rook(_color: &PieceColor, _start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}

pub fn _queen(_color: &PieceColor, _start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}

pub fn _king(_color: &PieceColor, _start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    true
}
