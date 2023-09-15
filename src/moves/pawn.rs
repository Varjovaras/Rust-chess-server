use crate::{
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
    piece::Pieces,
};

use super::move_helpers::{diagonally_one_square_apart, square_column_diff};

pub fn move_white_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::Second {
        white_starting_sq_move(start_sq, end_sq, chess)
    } else if diagonally_one_square_apart(start_sq, end_sq) {
        return white_capture(start_sq, end_sq, chess);
    } else {
        one_square_forward(end_sq)
    }
}

fn white_starting_sq_move(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if diagonally_one_square_apart(start_sq, end_sq) {
        white_capture(start_sq, end_sq, chess)
    } else {
        let column_diff = square_column_diff(start_sq, end_sq);
        match column_diff {
            1 => return one_square_forward(end_sq),
            2 => return two_squares_forward(start_sq, end_sq, chess),
            _ => return false,
        };
    }
}

fn white_en_passant() -> bool {
    true
}

fn one_square_forward(end_sq: &Square) -> bool {
    if end_sq.piece != Pieces::NoPiece() {
        false
    } else {
        true
    }
}

fn two_squares_forward(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let in_between_sq = chess.get_square(
        File::from(start_sq.file as u8),
        Rank::from(start_sq.rank as u8 + 1),
    );
    if end_sq.has_piece() || in_between_sq.has_piece() {
        false
    } else {
        true
    }
}

fn white_capture(_start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    //check for en passant
    match chess.latest_move {
        Some(latest_move) => {
            if latest_move.0.rank == Rank::Seventh
                && latest_move.1.rank == Rank::Fifth
                && latest_move.0.piece == Pieces::Pawn(latest_move.2)
            {
                println!("Move is en passant");
                return white_en_passant();
            }
        }
        None => {
            println!("Move is not en passant");
        }
    }
    if end_sq.is_empty() {
        false
    } else {
        true
    }
}

pub fn move_black_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    true
}

fn _black_starting_sq_move(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    true
}
