use serde::{Deserialize, Serialize};

use crate::{chess::Chess, chessboard::square::Square, moves};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum PieceColor {
    White,
    Black,
    None,
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
    pub fn piece_move(self, start_sq: Square, end_sq: Square, chess: &Chess) -> bool {
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

    pub const fn name(&self) -> &str {
        match self {
            Self::None => "no piece",
            Self::Pawn(_) => "pawn",
            Self::Knight(_) => "knight",
            Self::Bishop(_) => "bishop",
            Self::Rook(_) => "rook",
            Self::Queen(_) => "queen",
            Self::King(_) => "king",
        }
    }
}
