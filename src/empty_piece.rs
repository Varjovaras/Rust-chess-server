use crate::piece::{Piece, PieceColor};

pub struct NoPiece {}

impl Piece for NoPiece {
    fn new(_color: PieceColor) -> Self {
        NoPiece {}
    }

    fn move_piece(&self) -> bool {
        false
    }

    fn color(&self) -> &PieceColor {
        &PieceColor::NoPiece
    }
}
