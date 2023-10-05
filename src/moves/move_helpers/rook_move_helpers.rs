use crate::{
    chess::{self, Chess},
    chessboard::square::Square,
};

pub enum _RookMoveDirection {
    Up,
    Right,
    Left,
    Down,
}

impl _RookMoveDirection {
    pub fn _new(start_sq: &Square, end_sq: &Square) -> Option<_RookMoveDirection> {
        if _move_is_up(start_sq, end_sq) {
            Some(Self::Up)
        } else if _move_is_right(start_sq, end_sq) {
            return Some(Self::Right);
        } else if _move_is_left(start_sq, end_sq) {
            return Some(Self::Left);
        } else if _move_is_down(start_sq, end_sq) {
            return Some(Self::Down);
        } else {
            None
        }
    }

    pub fn _make_move(&self, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
        match self {
            _RookMoveDirection::Up => move_up(start_sq, end_sq, chess),
            _RookMoveDirection::Right => move_right(start_sq, end_sq, chess),
            _RookMoveDirection::Left => move_left(start_sq, end_sq, chess),
            _RookMoveDirection::Down => move_down(start_sq, end_sq, chess),
        }
    }
}

fn _move_is_up(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file == end_sq.file && start_sq.rank < end_sq.rank
}

fn _move_is_right(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file < end_sq.file && start_sq.rank == end_sq.rank
}

fn _move_is_left(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file > end_sq.file && start_sq.rank == end_sq.rank
}

fn _move_is_down(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file == end_sq.file && start_sq.rank > end_sq.rank
}

fn move_up(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.rank as usize - start_sq.rank as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}

fn move_right(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.file as usize - start_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize + i][start_sq.rank as usize].has_piece() {
            return false;
        }
    }
    true
}

fn move_left(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.file as usize - end_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize - i][start_sq.rank as usize].has_piece() {
            return false;
        }
    }
    true
}

fn move_down(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.rank as usize - end_sq.rank as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize][start_sq.rank as usize - i].has_piece() {
            return false;
        }
    }
    true
}
