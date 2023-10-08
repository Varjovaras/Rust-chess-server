use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::{
    helpers::{is_horizontal, is_vertical},
    rook_move_helpers::RookMoveDirection,
};

pub fn move_rook(start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    if !is_vertical(start_sq, _end_sq) || !is_horizontal(start_sq, _end_sq) {
        return false;
    }

    match RookMoveDirection::new(start_sq, _end_sq) {
        Some(move_direction) => move_direction.make_move(start_sq, _end_sq, _chess),
        None => false,
    }
}
