use crate::{chess::Chess, chessboard::square::Square};

pub const ROOK_MOVES: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

use super::move_helpers::{
    helpers::{is_horizontal, is_vertical},
    rook_move::MoveDirection,
};

#[must_use]
pub fn move_piece(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if !is_vertical(start_sq, end_sq) && !is_horizontal(start_sq, end_sq) {
        return false;
    }

    MoveDirection::new(start_sq, end_sq).map_or_else(
        || false,
        |move_direction| move_direction.make_move(start_sq, end_sq, chess),
    )
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
        let mut chess = Chess::default();
        chess.starting_position();
        let sq1 = &chess.board[0][0];
        let sq2 = &chess.board[0][6];
        assert!(!move_piece(sq1, sq2, &chess));
        chess.board[0][1].piece = NONE;
        let sq1 = &chess.board[0][0];
        let sq2 = &chess.board[0][6];
        assert!(move_piece(sq1, sq2, &chess));

        chess.board[4][4].piece = BLACK_ROOK;
        let sq1 = &chess.board[4][4];
        let sq2 = &chess.board[0][6];
        assert!(!move_piece(sq1, sq2, &chess));
        let sq1 = &chess.board[4][4];
        let sq2 = &chess.board[4][5];
        assert!(move_piece(sq1, sq2, &chess));
        let sq1 = &chess.board[4][4];
        let sq2 = &chess.board[7][4];
        assert!(move_piece(sq1, sq2, &chess));
        let sq1 = &chess.board[4][4];
        let sq2 = &chess.board[0][4];
        assert!(move_piece(sq1, sq2, &chess));

        chess.board[1][4].piece = WHITE_ROOK;
        let sq1 = &chess.board[1][4];
        let sq2 = &chess.board[4][4];
        assert!(move_piece(sq1, sq2, &chess));

        let sq1 = &chess.board[1][4];
        let sq2 = &chess.board[1][1];
        assert!(move_piece(sq1, sq2, &chess));
    }
}
