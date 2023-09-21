use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

use super::move_helpers::is_diagonal;

pub fn move_bishob(color: &PieceColor, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if !is_diagonal(start_sq, end_sq) {
        return false;
    }
    if pieces_on_the_way(start_sq, end_sq, chess) {
        return false;
    }
    up_and_left(start_sq, end_sq)
        || up_and_right(start_sq, end_sq)
        || down_and_left(start_sq, end_sq)
        || down_and_right(start_sq, end_sq)
}

fn pieces_on_the_way(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let chess_board = &chess.board;
    let file_direction = start_sq.file as i8 - end_sq.file as i8;
    let rank_direction = start_sq.rank as i8 - end_sq.file as i8;

    for i in 0..file_direction {
        todo!("")
    }

    true
}

fn up_and_left(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file > end_sq.file && start_sq.rank < end_sq.rank
}

fn up_and_right(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file < end_sq.file && start_sq.rank < end_sq.rank
}

fn down_and_left(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file > end_sq.file && start_sq.rank > end_sq.rank
}

fn down_and_right(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file < end_sq.file && start_sq.rank > end_sq.rank
}
