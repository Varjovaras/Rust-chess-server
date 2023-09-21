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
    let file_direction: isize;
    let rank_direction: isize;
    if start_sq.file > end_sq.file {
        file_direction = -1;
    } else {
        file_direction = 1;
    }
    if start_sq.rank > end_sq.rank {
        rank_direction = -1;
    } else {
        rank_direction = 1;
    }

    loop {
        let i = start_sq.file as usize;
        let j = start_sq.rank as usize;
        chess_board[i - file_direction as usize][j - rank_direction as usize];
        break;
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
