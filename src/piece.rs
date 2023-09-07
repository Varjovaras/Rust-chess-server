mod no_piece;
mod pawn;

use crate::chessboard::square::Square;

pub enum PieceColor {
    _White,
    _Black,
    NoPiece,
}

pub trait Piece {
    fn new(color: PieceColor) -> Self;
    fn move_piece(&self, board: &[[Square; 8]; 8]) -> bool;
    fn color(&self) -> &PieceColor;
    // fn square_name(&self) -> &'static str;
}
