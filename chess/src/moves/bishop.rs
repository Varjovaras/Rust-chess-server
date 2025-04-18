use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::{bishop_move::DiagonalMoveDirection, helpers::is_diagonal};

pub const BISHOP_MOVES: [(i8, i8); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

#[must_use]
pub fn move_bishob(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if !is_diagonal(start_sq, end_sq)
        || end_sq.has_piece() && end_sq.piece.color() == start_sq.piece.color()
    {
        return false;
    }

    DiagonalMoveDirection::new(start_sq, end_sq).map_or_else(
        || false,
        |move_direction| move_direction.make_move(start_sq, end_sq, chess),
    )
}

#[cfg(test)]
mod tests {
    // use crate::piece::{Piece, PieceColor};

    // use super::*;
    // #[test]
    // fn move_bishop_works() {
    //     let mut chess: Chess = Chess::_new();
    //     //Bishop on D5
    //     chess.board[3][4].piece = Piece::Bishop(PieceColor::White);

    //     let sq1 = *chess.get_square_from_str("d", "5");

    //     let sq2 = *chess.get_square_from_str("a", "2");
    //     assert!(move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("a", "8");
    //     assert!(move_bishob(sq1, sq2, &chess));

    //     //non-diagonal moves
    //     let sq2 = *chess.get_square_from_str("a", "7");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("a", "6");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("a", "5");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("a", "4");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("a", "3");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("a", "1");
    //     assert!(!move_bishob(sq1, sq2, &chess));

    //     let sq2 = *chess.get_square_from_str("g", "8");
    //     assert!(move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("g", "7");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("h", "8");
    //     assert!(!move_bishob(sq1, sq2, &chess));

    //     let sq2 = *chess.get_square_from_str("h", "1");
    //     assert!(move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("h", "2");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    //     let sq2 = *chess.get_square_from_str("h", "3");
    //     assert!(!move_bishob(sq1, sq2, &chess));
    // }
}
