use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod move_helpers;
pub mod pawn;
pub mod queen;
pub mod rook;

pub fn pawn(start_sq: &Square, end_sq: &Square, chess: &Chess, color: PieceColor) -> bool {
    match color {
        PieceColor::White => pawn::white::move_white_pawn(start_sq, end_sq, chess),
        PieceColor::Black => pawn::black::move_black_pawn(start_sq, end_sq, chess),
        PieceColor::None => false,
    }
}

pub fn knight(start_sq: &Square, end_sq: &Square) -> bool {
    knight::move_piece(start_sq, end_sq)
}

pub fn bishop(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    bishop::move_bishob(start_sq, end_sq, chess)
}

pub fn rook(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    rook::move_piece(start_sq, end_sq, chess)
}

pub fn queen(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    queen::move_piece(start_sq, end_sq, chess)
}

pub fn king(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    king::move_piece(start_sq, end_sq, chess)
}
