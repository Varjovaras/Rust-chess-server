use crate::{chess::Chess, chessboard::square::Square};

pub enum RookMoveDirection {
    Up,
    Right,
    Left,
    Down,
}

impl RookMoveDirection {
    pub fn new(start_sq: &Square, end_sq: &Square) -> Option<RookMoveDirection> {
        if move_is_up(start_sq, end_sq) {
            Some(Self::Up)
        } else if move_is_right(start_sq, end_sq) {
            return Some(Self::Right);
        } else if move_is_left(start_sq, end_sq) {
            return Some(Self::Left);
        } else if move_is_down(start_sq, end_sq) {
            return Some(Self::Down);
        } else {
            None
        }
    }

    pub fn make_move(&self, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
        match self {
            RookMoveDirection::Up => move_up(start_sq, end_sq, chess),
            RookMoveDirection::Right => move_right(start_sq, end_sq, chess),
            RookMoveDirection::Left => move_left(start_sq, end_sq, chess),
            RookMoveDirection::Down => move_down(start_sq, end_sq, chess),
        }
    }
}

fn move_is_up(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file == end_sq.file && start_sq.rank < end_sq.rank
}

fn move_is_right(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file < end_sq.file && start_sq.rank == end_sq.rank
}

fn move_is_left(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file > end_sq.file && start_sq.rank == end_sq.rank
}

fn move_is_down(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file == end_sq.file && start_sq.rank > end_sq.rank
}

fn move_up(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.rank as usize - start_sq.rank as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize][start_sq.rank as usize + i].has_piece() {
            println!(
                "{:?}",
                chess.board[start_sq.file as usize][start_sq.rank as usize + i]
            );
            return false;
        }

        // pr
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
