use crate::{chess::Chess, chessboard::square::Square};

pub fn _move_knight(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    _is_knight_move(_start_sq, _end_sq)
}

fn _is_knight_move(_start_sq: &Square, _end_sq: &Square) -> bool {
    let start_sq_file = _start_sq.file as u8;
    let start_sq_rank = _start_sq.rank as u8;
    let end_sq_file = _end_sq.file as u8;
    let end_sq_rank = _end_sq.rank as u8;

    start_sq_file.abs_diff(end_sq_file) == 1 && start_sq_rank.abs_diff(end_sq_rank) == 2
        || start_sq_file.abs_diff(end_sq_file) == 2 && start_sq_rank.abs_diff(end_sq_rank) == 1
}
