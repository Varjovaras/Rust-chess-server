use crate::{
    castling::Castling,
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
    piece::{PieceColor, Pieces},
};

use super::move_helpers::helpers::{is_diagonal, is_horizontal, is_vertical};

pub fn move_king(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if move_is_castling(start_sq, end_sq, &chess.castling) {
        return true;
    } else if is_vertical(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8) == 1
    } else if is_horizontal(start_sq, end_sq) {
        (start_sq.file as u8).abs_diff(end_sq.file as u8) == 1
    } else if is_diagonal(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8)
            == (start_sq.file as u8).abs_diff(end_sq.file as u8)
    } else {
        false
    }
}

pub fn move_is_castling(start_sq: &Square, end_sq: &Square, castling: &Vec<Castling>) -> bool {
    if start_sq.piece == Pieces::King(PieceColor::White)
        || start_sq.piece == Pieces::King(PieceColor::Black)
    {
        return false;
    };

    if !(start_sq.file == File::E
        && (start_sq.file as u8).abs_diff(end_sq.file as u8) == 2
        && (end_sq.file == File::G || end_sq.file == File::C)
        && (start_sq.rank == Rank::First || start_sq.rank == Rank::Eighth)
        && start_sq.rank == end_sq.rank)
    {
        return false;
    }
    match (start_sq.rank, end_sq.file) {
        (Rank::First, File::G) => castling[0].castling_allowed(),
        (Rank::First, File::C) => castling[1].castling_allowed(),
        (Rank::Eighth, File::G) => castling[2].castling_allowed(),
        (Rank::Eighth, File::C) => castling[3].castling_allowed(),
        _ => return false,
    }
}
