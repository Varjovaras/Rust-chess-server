use crate::{
    chess::Chess,
    chessboard::square::Square,
    moves::move_helpers::{
        _move_is_down_and_left, _move_is_down_and_right, _move_is_up_and_left,
        _move_is_up_and_right,
    },
};

pub enum _BishopMoveDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl _BishopMoveDirection {
    pub fn _new(start_sq: &Square, end_sq: &Square) -> Option<_BishopMoveDirection> {
        if _move_is_up_and_left(start_sq, end_sq) {
            Some(Self::UpLeft)
        } else if _move_is_up_and_right(start_sq, end_sq) {
            return Some(Self::UpRight);
        } else if _move_is_down_and_left(start_sq, end_sq) {
            return Some(Self::DownLeft);
        } else if _move_is_down_and_right(start_sq, end_sq) {
            return Some(Self::DownRight);
        } else {
            None
        }
    }

    pub fn _make_move(&self, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
        match self {
            _BishopMoveDirection::UpLeft => _move_top_left(start_sq, end_sq, chess),
            _BishopMoveDirection::UpRight => _move_top_right(start_sq, end_sq, chess),
            _BishopMoveDirection::DownLeft => _move_down_left(start_sq, end_sq, chess),
            _BishopMoveDirection::DownRight => _move_down_right(start_sq, end_sq, chess),
        }
    }
}

fn _move_top_left(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.file as usize - end_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize - i][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}

fn _move_top_right(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.file as usize - start_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize + i][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}

fn _move_down_left(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.file as usize - end_sq.file as usize;
    for i in 1..distance {
        println!(
            "{:?}",
            chess.board[start_sq.file as usize - i][start_sq.rank as usize - i].piece
        );
        if chess.board[start_sq.file as usize - i][start_sq.rank as usize - i].has_piece() {
            return false;
        }
    }
    true
}

fn _move_down_right(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.file as usize - start_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize + i][start_sq.rank as usize - i].has_piece() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::piece::{PieceColor, Pieces};

    use super::*;
    #[test]
    fn bishop_move_functions_work() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();
        //no piece on D2
        chess.board[3][1].piece = Pieces::NoPiece();

        let start_sq = *chess.get_square_from_str("c", "1");
        let end_sq = *chess.get_square_from_str("g", "5");

        assert_eq!(_move_top_right(&start_sq, &end_sq, &chess), true);

        //D2 is now blocked
        chess.board[3][1].piece = Pieces::Queen(PieceColor::Black);
        assert_eq!(_move_top_right(&start_sq, &end_sq, &chess), false);

        //b2 is not empty
        let end_sq = *chess.get_square_from_str("a", "3");
        assert_eq!(_move_top_left(&start_sq, &end_sq, &chess), false);
        chess.board[1][1].piece = Pieces::NoPiece();
        assert_eq!(_move_top_left(&start_sq, &end_sq, &chess), true);

        //Bishop on F2
        chess.board[5][1].piece = Pieces::Bishop(PieceColor::Black);
        let start_sq = *chess.get_square_from_str("f", "2");
        let end_sq = *chess.get_square_from_str("a", "7");
        assert_eq!(_move_top_left(&start_sq, &end_sq, &chess), true);

        chess.board[4][2].piece = Pieces::Bishop(PieceColor::Black);
        assert_eq!(_move_top_left(&start_sq, &end_sq, &chess), false);
        chess.board[4][2].piece = Pieces::NoPiece();
        chess.board[1][5].piece = Pieces::Bishop(PieceColor::Black);
        assert_eq!(_move_top_left(&start_sq, &end_sq, &chess), false);

        let start_sq = *chess.get_square_from_str("b", "6");
        let end_sq = *chess.get_square_from_str("f", "2");
        assert_eq!(_move_down_right(&start_sq, &end_sq, &chess), true);
        let end_sq = *chess.get_square_from_str("g", "1");
        assert_eq!(_move_down_right(&start_sq, &end_sq, &chess), false);

        //Bishop on H8 and pawn on G7 blocks it
        chess.board[7][7].piece = Pieces::Bishop(PieceColor::White);
        let start_sq = *chess.get_square_from_str("h", "8");
        let end_sq = *chess.get_square_from_str("a", "1");
        assert_eq!(_move_down_left(&start_sq, &end_sq, &chess), false);
        let end_sq = *chess.get_square_from_str("c", "3");
        assert_eq!(_move_down_left(&start_sq, &end_sq, &chess), false);

        chess.board[6][6].piece = Pieces::NoPiece();

        assert_eq!(_move_down_left(&start_sq, &end_sq, &chess), true);
        let end_sq = *chess.get_square_from_str("a", "1");
        assert_eq!(_move_down_left(&start_sq, &end_sq, &chess), true);
    }
}
