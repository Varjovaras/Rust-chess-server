use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

use super::move_helpers::helpers::{_is_horizontal, _is_vertical};

pub fn _move_rook(
    _color: &PieceColor,
    _start_sq: &Square,
    _end_sq: &Square,
    _chess: &Chess,
) -> bool {
    if !_is_vertical(_start_sq, _end_sq) || !_is_horizontal(_start_sq, _end_sq) {
        return false;
    }
    true
}
