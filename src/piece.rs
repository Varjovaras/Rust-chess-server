pub enum PieceColor {
    _White,
    _Black,
    NoPiece,
}

pub trait Piece {
    fn new(color: PieceColor) -> Self;
    fn move_piece(&self) -> bool;
    fn color(&self) -> &PieceColor;
    // fn square_name(&self) -> &'static str;
}
