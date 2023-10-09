use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::{
    helpers::{is_horizontal, is_vertical},
    rook_move_helpers::RookMoveDirection,
};

pub fn move_rook(start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    if !is_vertical(start_sq, _end_sq) && !is_horizontal(start_sq, _end_sq) {
        println!("blyat");

        return false;
    }

    match RookMoveDirection::new(start_sq, _end_sq) {
        Some(move_direction) => move_direction.make_move(start_sq, _end_sq, _chess),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{PieceColor, Pieces};

    const NONE: Pieces = Pieces::None;
    const WHITEROOK: Pieces = Pieces::Rook(PieceColor::White);
    const BLACKROOK: Pieces = Pieces::Rook(PieceColor::Black);

    #[test]
    fn rook_move_works() {
        let mut chess = Chess::new();
        chess.starting_position();
        let sq1 = chess.board[0][0];
        let sq2 = chess.board[0][5];
        assert_eq!(move_rook(&sq1, &sq2, &chess), false);
        chess.board[0][1].piece = NONE;
        assert_eq!(move_rook(&sq1, &sq2, &chess), true);

        chess.board[4][4].piece = BLACKROOK;
        let sq1 = chess.board[4][4];
        assert_eq!(move_rook(&sq1, &sq2, &chess), false);
        let sq2 = chess.board[4][5];
        assert_eq!(move_rook(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[7][4];
        assert_eq!(move_rook(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[0][4];
        assert_eq!(move_rook(&sq1, &sq2, &chess), true);

        chess.board[1][4].piece = WHITEROOK;
        let sq1 = chess.board[1][4];
        let sq2 = chess.board[4][4];
        assert_eq!(move_rook(&sq1, &sq2, &chess), true);
    }
}
