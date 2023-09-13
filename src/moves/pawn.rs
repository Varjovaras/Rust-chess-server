use crate::{
    chess::Chess,
    chessboard::{rank::Rank, square::Square},
};

pub fn move_white_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::Second {
        white_starting_sq_move(start_sq, end_sq, chess)
    } else {
        normal_move()
    }
}

fn white_starting_sq_move(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.file != end_sq.file {
        return white_capture(start_sq, end_sq, chess);
    }
    let rank = end_sq.rank;
    // if

    true
}

fn en_passant() -> bool {
    true
}

fn normal_move() -> bool {
    true
}

fn white_capture(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    match chess.latest_move {
        Some(latest_move) => {
            return true;
        }
        None => {
            return false;
        }
    }
    true
}

pub fn move_black_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    true
}

fn _black_starting_sq_move(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    true
}
