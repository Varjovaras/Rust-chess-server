use crate::piece::Piece;

pub struct Pawn {
    // pub square: String,
}

impl Piece for Pawn {
    fn new() -> Self {
        Pawn {}
    }

    fn move_piece(&self) -> bool {
        true
    }
}
