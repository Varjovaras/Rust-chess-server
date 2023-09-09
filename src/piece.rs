use crate::{chessboard::square::Square, moves};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pieces {
    NoPiece(PieceColor),
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl Pieces {
    pub fn r#_move(&self, start_sq: &Square, end_sq: &Square, board: &[[Square; 8]; 8]) -> bool {
        match self {
            Pieces::NoPiece(_) => false,
            Pieces::Pawn(color) => moves::pawn(color, start_sq, end_sq, board),
            Pieces::Knight(color) => moves::_knight(color, start_sq, end_sq, board),
            Pieces::Bishop(color) => moves::_bishop(color, start_sq, end_sq, board),
            Pieces::Rook(color) => moves::_rook(color, start_sq, end_sq, board),
            Pieces::Queen(color) => moves::_queen(color, start_sq, end_sq, board),
            Pieces::King(color) => moves::_king(color, start_sq, end_sq, board),
        }
    }
}

impl Default for Pieces {
    fn default() -> Pieces {
        Pieces::NoPiece(PieceColor::White)
    }
}

// pub struct NoPiece {}

// impl PieceTrait for NoPiece {
//     fn new(_color: PieceColor) -> Self {
//         NoPiece {}
//     }

//     fn move_piece(&self, _start_sq: Square, _board: &[[Square; 8]; 8]) -> bool {
//         false
//     }

//     fn color(&self) -> &PieceColor {
//         &PieceColor::NoPiece
//     }
// }

// impl PieceTrait for Pieces {
//     fn new(color: PieceColor) -> Self {
//         Self::NoPiece
//     }
//     fn move_piece(&self, start_sq: Square, board: &[[Square; 8]; 8]) -> bool {
//         false
//     }

//     fn color(&self) -> &PieceColor {
//         &PieceColor::NoPiece
//     }
// }
