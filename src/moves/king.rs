use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::helpers::{is_diagonal, is_horizontal, is_vertical};

pub fn move_king(start_sq: &Square, end_sq: &Square, _chess: &Chess) -> bool {
    todo!("castling");
    if is_vertical(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8) == 1
    } else if is_horizontal(start_sq, end_sq) {
        (start_sq.file as u8).abs_diff(end_sq.file as u8) == 1
    } else if is_diagonal(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8)
            == (start_sq.file as u8).abs_diff(end_sq.file as u8)
    } else {
        return false;
    }
}
