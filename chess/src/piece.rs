use serde::{Deserialize, Serialize};

use crate::{chess::Chess, chessboard::square::Square, moves};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum PieceColor {
    White,
    Black,
    None,
}

impl PieceColor {
    #[must_use]
    pub const fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
            Self::None => Self::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Piece {
    #[default]
    None,
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl Piece {
    #[must_use]
    pub fn piece_move(self, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
        match self {
            Self::None => false,
            Self::Pawn(color) => moves::pawn(start_sq, end_sq, chess, color),
            Self::Knight(_) => moves::knight(start_sq, end_sq),
            Self::Bishop(_) => moves::bishop(start_sq, end_sq, chess),
            Self::Rook(_) => moves::rook(start_sq, end_sq, chess),
            Self::Queen(_) => moves::queen(start_sq, end_sq, chess),
            Self::King(_) => moves::king(start_sq, end_sq, chess),
        }
    }

    #[must_use]
    pub const fn color(self) -> PieceColor {
        match self {
            Self::None => PieceColor::None,
            Self::Pawn(color)
            | Self::Knight(color)
            | Self::Bishop(color)
            | Self::Rook(color)
            | Self::Queen(color)
            | Self::King(color) => color,
        }
    }

    #[must_use]
    pub const fn is_king(self) -> bool {
        matches!(self, Self::King(_))
    }

    #[must_use]
    pub const fn is_rook(self) -> bool {
        matches!(self, Self::Rook(_))
    }

    #[must_use]
    pub const fn is_pawn(self) -> bool {
        matches!(self, Self::Pawn(_))
    }
}

impl From<Option<&str>> for Piece {
    fn from(value: Option<&str>) -> Self {
        match value {
            Some("QUEEN") => Self::Queen(PieceColor::None),
            Some("ROOK") => Self::Rook(PieceColor::None),
            Some("BISHOP") => Self::Bishop(PieceColor::None),
            Some("KNIGHT") => Self::Knight(PieceColor::None),
            _ => Self::None,
        }
    }
}

#[must_use]
pub const fn promoted_piece_to_i32tuple(promoted_piece: Option<Piece>) -> (i32, i32) {
    match promoted_piece {
        Some(Piece::Queen(PieceColor::White)) => (1, 0),
        Some(Piece::Queen(PieceColor::Black)) => (1, 1),
        Some(Piece::Rook(PieceColor::White)) => (2, 0),
        Some(Piece::Rook(PieceColor::Black)) => (2, 1),
        Some(Piece::Bishop(PieceColor::White)) => (3, 0),
        Some(Piece::Bishop(PieceColor::Black)) => (3, 1),
        Some(Piece::Knight(PieceColor::White)) => (4, 0),
        Some(Piece::Knight(PieceColor::Black)) => (4, 1),
        _ => (0, 0),
    }
}

#[must_use]
#[allow(clippy::module_name_repetitions)]
pub const fn tuple_to_promoted_piece(promoted_piece: (i32, i32)) -> Option<Piece> {
    match promoted_piece {
        (1, 0) => Some(Piece::Queen(PieceColor::White)),
        (1, 1) => Some(Piece::Queen(PieceColor::Black)),
        (2, 0) => Some(Piece::Rook(PieceColor::White)),
        (2, 1) => Some(Piece::Rook(PieceColor::Black)),
        (3, 0) => Some(Piece::Bishop(PieceColor::White)),
        (3, 1) => Some(Piece::Bishop(PieceColor::Black)),
        (4, 0) => Some(Piece::Knight(PieceColor::White)),
        (4, 1) => Some(Piece::Knight(PieceColor::Black)),
        _ => None,
    }
}
