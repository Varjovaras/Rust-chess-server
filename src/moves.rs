use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

mod bishop;
mod king;
mod knight;
pub mod move_helpers;
pub mod pawn;
mod queen;
mod rook;

pub fn pawn(start_sq: &Square, end_sq: &Square, chess: &Chess, color: &PieceColor) -> bool {
    match *color {
        PieceColor::White => pawn::white_pawn::move_white_pawn(start_sq, end_sq, chess),
        PieceColor::Black => pawn::black_pawn::move_black_pawn(start_sq, end_sq, chess),
        PieceColor::None => false,
    }
}

pub fn knight(start_sq: &Square, end_sq: &Square) -> bool {
    knight::move_knight(start_sq, end_sq)
}

pub fn bishop(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    bishop::move_bishob(start_sq, end_sq, chess)
}

pub fn rook(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    rook::move_rook(start_sq, end_sq, chess)
}

pub fn queen(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    queen::move_queen(start_sq, end_sq, chess)
}

pub fn king(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    king::move_king(start_sq, end_sq, chess)
}
