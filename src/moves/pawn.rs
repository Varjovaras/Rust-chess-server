use crate::{
    chess::Chess,
    chessboard::{rank::Rank, square::Square},
    piece::{PieceColor, Pieces},
};

use super::move_helpers::diagonally_one_square_apart;

pub fn move_white_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::Second {
        white_starting_sq_move(start_sq, end_sq, chess)
    } else {
        normal_move()
    }
}

fn white_starting_sq_move(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if diagonally_one_square_apart(start_sq, end_sq) {
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
            if latest_move.0.rank == Rank::Seventh
                && latest_move.1.rank == Rank::Fifth
                && latest_move.0.piece == Pieces::Pawn(PieceColor::Black)
            {
                return en_passant();
            }
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
