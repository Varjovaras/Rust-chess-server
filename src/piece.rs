use crate::{chessboard::square::Square, moves};

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    _White,
    _Black,
}

#[derive(Clone, Copy, Debug)]
pub enum Pieces {
    NoPiece(PieceColor),
    _Pawn(PieceColor),
    _Knight(PieceColor),
    _Bishop(PieceColor),
    _Rook(PieceColor),
    _Queen(PieceColor),
    _King(PieceColor),
}

impl Pieces {
    fn r#_move(&self, board: &[[Square; 8]; 8]) -> bool {
        match self {
            Pieces::NoPiece(_) => false,
            Pieces::_Pawn(color) => moves::_pawn(color, board),
            Pieces::_Knight(color) => moves::_knight(color, board),
            Pieces::_Bishop(color) => moves::_bishop(color, board),
            Pieces::_Rook(color) => moves::_rook(color, board),
            Pieces::_Queen(color) => moves::_queen(color, board),
            Pieces::_King(color) => moves::_king(color, board),
        }
    }
}

impl Default for Pieces {
    fn default() -> Pieces {
        Pieces::NoPiece(PieceColor::_White)
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
