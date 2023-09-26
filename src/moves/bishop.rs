use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};
mod bishop_moves;

use self::bishop_moves::_BishopMoveDirection;

use super::move_helpers::_is_diagonal;

pub fn _move_bishob(
    _color: &PieceColor,
    start_sq: &Square,
    end_sq: &Square,
    chess: &Chess,
) -> bool {
    if !_is_diagonal(start_sq, end_sq) {
        return false;
    }

    match _BishopMoveDirection::_new(start_sq, end_sq) {
        Some(move_direction) => move_direction._make_move(start_sq, end_sq, chess),
        None => false,
    }
}
