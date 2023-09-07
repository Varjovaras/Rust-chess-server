use crate::{
    chessboard::square::Square,
    piece::{Piece, PieceColor},
};

pub struct NoPiece {}

impl Piece for NoPiece {
    fn new(_color: PieceColor) -> Self {
        NoPiece {}
    }

    fn move_piece(&self, _board: &[[Square; 8]; 8]) -> bool {
        false
    }

    fn color(&self) -> &PieceColor {
        &PieceColor::NoPiece
    }
}
