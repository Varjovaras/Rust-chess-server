use crate::{chess::Chess, chessboard::square::Square, moves};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pieces {
    None,
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl Pieces {
    pub fn move_is_legal(&self, start_sq: &Square, end_sq: &Square, chess: &mut Chess) -> bool {
        match self {
            Pieces::None => false,
            Pieces::Pawn(color) => moves::pawn(start_sq, end_sq, chess, color),
            Pieces::Knight(_) => moves::_knight(start_sq, end_sq, chess),
            Pieces::Bishop(_) => moves::_bishop(start_sq, end_sq, chess),
            Pieces::Rook(_) => moves::_rook(start_sq, end_sq, chess),
            Pieces::Queen(_) => moves::_queen(start_sq, end_sq, chess),
            Pieces::King(_) => moves::_king(start_sq, end_sq, chess),
        }
    }

    pub fn color(&self) -> &PieceColor {
        match self {
            Pieces::None => &PieceColor::None,
            Pieces::Pawn(color) => color,
            Pieces::Knight(color) => color,
            Pieces::Bishop(color) => color,
            Pieces::Rook(color) => color,
            Pieces::Queen(color) => color,
            Pieces::King(color) => color,
        }
    }
}

impl Default for Pieces {
    fn default() -> Pieces {
        Pieces::None
    }
}
