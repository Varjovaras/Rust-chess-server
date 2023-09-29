use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::{diagonal_move_direction::_DiagonalMoveDirection, helpers::_is_diagonal};

pub fn _move_bishob(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if !_is_diagonal(start_sq, end_sq) {
        return false;
    }

    match _DiagonalMoveDirection::_new(start_sq, end_sq) {
        Some(move_direction) => move_direction._make_move(start_sq, end_sq, chess),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::{PieceColor, Pieces};

    use super::*;
    #[test]
    fn move_bishop_works() {
        let mut chess: Chess = Chess::new();
        //Bishop on D5
        chess.board[3][4].piece = Pieces::Bishop(PieceColor::White);

        let sq1 = *chess.get_square_from_str("d", "5");
        let sq2 = *chess.get_square_from_str("a", "2");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), true);
        let sq2 = *chess.get_square_from_str("a", "8");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), true);

        //non-diagonal moves
        let sq2 = *chess.get_square_from_str("a", "7");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);
        let sq2 = *chess.get_square_from_str("a", "6");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);
        let sq2 = *chess.get_square_from_str("a", "5");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);
        let sq2 = *chess.get_square_from_str("a", "4");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);
        let sq2 = *chess.get_square_from_str("a", "3");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);
        let sq2 = *chess.get_square_from_str("a", "1");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);

        let sq2 = *chess.get_square_from_str("g", "8");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), true);
        let sq2 = *chess.get_square_from_str("g", "7");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);
        let sq2 = *chess.get_square_from_str("h", "8");
        assert_eq!(_move_bishob(&sq1, &sq2, &chess), false);
    }
}
