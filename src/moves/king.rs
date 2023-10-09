use crate::{
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
};

use super::move_helpers::helpers::{is_diagonal, is_horizontal, is_vertical};

pub fn move_king(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if (start_sq.rank as u8).abs_diff(end_sq.rank as u8) == 2 {
        return castling(start_sq, end_sq, chess);
    }

    if is_vertical(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8) == 1
    } else if is_horizontal(start_sq, end_sq) {
        (start_sq.file as u8).abs_diff(end_sq.file as u8) == 1
    } else if is_diagonal(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8)
            == (start_sq.file as u8).abs_diff(end_sq.file as u8)
    } else {
        return false;
    }
}

fn castling(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    match start_sq.file {
        File::E => {}
        _ => return false,
    }

    match (start_sq.rank, end_sq.file) {
        (Rank::First, File::G) => return white_king_side_castling(chess),
        (Rank::First, File::C) => return white_queen_side_castling(chess),
        (Rank::Eighth, File::G) => return black_king_side_castling(chess),
        (Rank::Eighth, File::C) => return black_queen_side_castling(chess),
        _ => return false,
    }
}

fn black_queen_side_castling(chess: &Chess) -> bool {
    todo!()
}

fn black_king_side_castling(chess: &Chess) -> bool {
    todo!()
}

fn white_queen_side_castling(chess: &Chess) -> bool {
    todo!()
}

fn white_king_side_castling(chess: &Chess) -> bool {
    todo!()
}
