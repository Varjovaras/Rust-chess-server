use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::{
    helpers::{_is_horizontal, _is_vertical},
    rook_move_helpers::_RookMoveDirection,
};

pub fn _move_rook(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    if !_is_vertical(_start_sq, _end_sq) || !_is_horizontal(_start_sq, _end_sq) {
        return false;
    }

    match _RookMoveDirection::_new(_start_sq, _end_sq) {
        Some(move_direction) => move_direction._make_move(_start_sq, _end_sq, _chess),
        None => false,
    }
}
