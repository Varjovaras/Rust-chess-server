use crate::piece::{Piece, PieceColor};

pub struct Pawn {
    pub color: PieceColor,
}

impl Piece for Pawn {
    fn new(color: PieceColor) -> Self {
        Pawn { color }
    }

    fn move_piece(&self) -> bool {
        true
    }

    fn color(&self) -> &PieceColor {
        &self.color
    }
}
