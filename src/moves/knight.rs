use crate::chessboard::square::Square;

pub fn _move_knight(start_sq: &Square, end_sq: &Square) -> bool {
    let start_sq_file = start_sq.file as u8;
    let start_sq_rank = start_sq.rank as u8;
    let end_sq_file = end_sq.file as u8;
    let end_sq_rank = end_sq.rank as u8;

    start_sq_file.abs_diff(end_sq_file) == 1 && start_sq_rank.abs_diff(end_sq_rank) == 2
        || start_sq_file.abs_diff(end_sq_file) == 2 && start_sq_rank.abs_diff(end_sq_rank) == 1
}
