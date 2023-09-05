pub trait Piece {
    fn new(&self) -> Self;
    fn move_piece(&self) -> bool;
    fn square_name(&self) -> &'static str;
}
