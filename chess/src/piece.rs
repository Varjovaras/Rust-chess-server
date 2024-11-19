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
