use crate::piece::Piece;

pub struct empty_piece {}

impl Piece for empty_piece {
    fn new() -> Self {
        empty_piece {}
    }

    fn move_piece(&self) -> bool {
        false
    }
}
