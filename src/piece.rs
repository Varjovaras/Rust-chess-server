pub enum PieceColor {
    White,
    Black,
    NoPiece,
}

pub trait Piece {
    fn new(color: PieceColor) -> Self;
    fn move_piece(&self) -> bool;
    fn color(&self) -> &PieceColor;
    // fn square_name(&self) -> &'static str;
}
