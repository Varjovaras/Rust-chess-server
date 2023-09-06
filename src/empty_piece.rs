use crate::piece::Piece;

pub struct EmptyPiece {}

impl Piece for EmptyPiece {
    fn new() -> Self {
        EmptyPiece {}
    }

    fn move_piece(&self) -> bool {
        false
    }
}
