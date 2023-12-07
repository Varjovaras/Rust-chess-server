use serde::{Deserialize, Serialize};

use crate::{chess::Chess, chessboard::square::Square, moves};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
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
    pub fn piece_move(&self, start_sq: &Square, end_sq: &Square, chess: &mut Chess) -> bool {
        match self {
            Piece::None => false,
            Piece::Pawn(color) => moves::pawn(start_sq, end_sq, chess, color),
            Piece::Knight(_) => moves::knight(start_sq, end_sq),
            Piece::Bishop(_) => moves::bishop(start_sq, end_sq, chess),
            Piece::Rook(_) => moves::rook(start_sq, end_sq, chess),
            Piece::Queen(_) => moves::queen(start_sq, end_sq, chess),
            Piece::King(_) => moves::king(start_sq, end_sq, chess),
        }
    }

    pub fn color(&self) -> &PieceColor {
        match self {
            Piece::None => &PieceColor::None,
            Piece::Pawn(color) => color,
            Piece::Knight(color) => color,
            Piece::Bishop(color) => color,
            Piece::Rook(color) => color,
            Piece::Queen(color) => color,
            Piece::King(color) => color,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Piece::None => "no piece",
            Piece::Pawn(_) => "pawn",
            Piece::Knight(_) => "knight",
            Piece::Bishop(_) => "bishop",
            Piece::Rook(_) => "rook",
            Piece::Queen(_) => "queen",
            Piece::King(_) => "king",
        }
    }

    pub fn possible_moves() {}
}
