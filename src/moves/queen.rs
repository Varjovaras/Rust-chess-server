use crate::{chess::Chess, chessboard::square::Square};

use super::{
    bishop,
    move_helpers::helpers::{is_diagonal, is_horizontal, is_vertical},
    rook,
};

pub fn move_queen(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if is_diagonal(start_sq, end_sq) {
        bishop::move_bishob(start_sq, end_sq, chess)
    } else if is_horizontal(start_sq, end_sq) || is_vertical(start_sq, end_sq) {
        rook::move_rook(start_sq, end_sq, chess)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {

    // use super::*;
    #[test]
    fn rook_move_works() {}
}
