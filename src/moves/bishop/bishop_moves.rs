use crate::{
    chess::Chess,
    chessboard::square::Square,
    moves::move_helpers::{
        move_is_down_and_left, move_is_down_and_right, move_is_up_and_left, move_is_up_and_right,
    },
};

pub enum _BishopMoveDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl _BishopMoveDirection {
    pub fn _new(start_sq: &Square, end_sq: &Square) -> Option<_BishopMoveDirection> {
        if move_is_up_and_left(start_sq, end_sq) {
            return Some(Self::UpLeft);
        } else if move_is_up_and_right(start_sq, end_sq) {
            return Some(Self::UpRight);
        } else if move_is_down_and_left(start_sq, end_sq) {
            return Some(Self::DownLeft);
        } else if move_is_down_and_right(start_sq, end_sq) {
            return Some(Self::DownRight);
        } else {
            None
        }
    }

    pub fn _make_move(&self, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
        match self {
            _BishopMoveDirection::UpLeft => _move_top_left(start_sq, end_sq, chess),
            _BishopMoveDirection::UpRight => _move_top_right(start_sq, end_sq, chess),
            _BishopMoveDirection::DownLeft => _move_down_left(start_sq, end_sq, chess),
            _BishopMoveDirection::DownRight => _move_down_right(start_sq, end_sq, chess),
        }
    }
}

fn _move_top_left(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.file as usize - end_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize - i][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}

fn _move_top_right(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.file as usize - start_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize + i][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}

fn _move_down_left(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.file as usize - end_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize - i][start_sq.rank as usize - i].has_piece() {
            return false;
        }
    }
    true
}

fn _move_down_right(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.file as usize - start_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize + i][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}
