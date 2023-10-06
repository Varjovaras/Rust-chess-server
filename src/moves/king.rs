use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::helpers::{_is_diagonal, _is_horizontal, _is_vertical};

pub fn _move_king(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    if _is_vertical(_start_sq, _end_sq) {
        (_start_sq.rank as u8).abs_diff(_end_sq.rank as u8) == 1
    } else if _is_horizontal(_start_sq, _end_sq) {
        (_start_sq.file as u8).abs_diff(_end_sq.file as u8) == 1
    } else if _is_diagonal(_start_sq, _end_sq) {
        (_start_sq.rank as u8).abs_diff(_end_sq.rank as u8)
            == (_start_sq.file as u8).abs_diff(_end_sq.file as u8)
    } else {
        return false;
    }
}
