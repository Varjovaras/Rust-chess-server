use crate::{chess::Chess, chessboard::square::Square};

pub const ROOK_MOVES: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

use super::move_helpers::{
    helpers::{is_horizontal, is_vertical},
    rook_move_helpers::RookMoveDirection,
};

pub fn move_rook(start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    if !is_vertical(start_sq, _end_sq) && !is_horizontal(start_sq, _end_sq) {
        return false;
    }

    RookMoveDirection::new(start_sq, _end_sq).map_or(false, |move_direction| {
        move_direction.make_move(start_sq, _end_sq, _chess)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{Piece, PieceColor};

    const NONE: Piece = Piece::None;
    const WHITE_ROOK: Piece = Piece::Rook(PieceColor::White);
    const BLACK_ROOK: Piece = Piece::Rook(PieceColor::Black);

    #[test]
    fn rook_move_works() {
        let mut chess = Chess::_new();
        chess.starting_position();
        let sq1 = chess.board[0][0];
        let sq2 = chess.board[0][6];
        assert!(!move_rook(&sq1, &sq2, &chess));
        chess.board[0][1].piece = NONE;
        assert!(move_rook(&sq1, &sq2, &chess));

        chess.board[4][4].piece = BLACK_ROOK;
        let sq1 = chess.board[4][4];
        assert!(!move_rook(&sq1, &sq2, &chess));
        let sq2 = chess.board[4][5];
        assert!(move_rook(&sq1, &sq2, &chess));
        let sq2 = chess.board[7][4];
        assert!(move_rook(&sq1, &sq2, &chess));
        let sq2 = chess.board[0][4];
        assert!(move_rook(&sq1, &sq2, &chess));

        chess.board[1][4].piece = WHITE_ROOK;
        let sq1 = chess.board[1][4];
        let sq2 = chess.board[4][4];
        assert!(move_rook(&sq1, &sq2, &chess));

        let sq2 = chess.board[1][1];
        assert!(move_rook(&sq1, &sq2, &chess));
    }
}
