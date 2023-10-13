use crate::{chess::Chess, chessboard::square::Square};

use super::helpers::{
    move_is_down_and_left, move_is_down_and_right, move_is_up_and_left, move_is_up_and_right,
};

pub enum DiagonalMoveDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl DiagonalMoveDirection {
    pub fn new(start_sq: &Square, end_sq: &Square) -> Option<DiagonalMoveDirection> {
        if move_is_up_and_left(start_sq, end_sq) {
            Some(Self::UpLeft)
        } else if move_is_up_and_right(start_sq, end_sq) {
            return Some(Self::UpRight);
        } else if move_is_down_and_left(start_sq, end_sq) {
            return Some(Self::DownLeft);
        } else if move_is_down_and_right(start_sq, end_sq) {
            return Some(Self::DownRight);
        } else {
            None
        }
    }

    pub fn make_move(&self, start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
        match self {
            DiagonalMoveDirection::UpLeft => move_top_left(start_sq, end_sq, chess),
            DiagonalMoveDirection::UpRight => move_top_right(start_sq, end_sq, chess),
            DiagonalMoveDirection::DownLeft => move_down_left(start_sq, end_sq, chess),
            DiagonalMoveDirection::DownRight => move_down_right(start_sq, end_sq, chess),
        }
    }
}

fn move_top_left(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.file as usize - end_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize - i][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}

fn move_top_right(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = end_sq.file as usize - start_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize + i][start_sq.rank as usize + i].has_piece() {
            return false;
        }
    }
    true
}

fn move_down_left(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let distance = start_sq.file as usize - end_sq.file as usize;
    for i in 1..distance {
        if chess.board[start_sq.file as usize - i][start_sq.rank as usize - i].has_piece() {
            return false;
        }
    }
    true
}

fn move_down_right(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
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
    use crate::piece::{Piece, PieceColor};

    use super::*;
    #[test]
    fn diagonal_movement_functions_work() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();
        //no piece on D2
        chess.board[3][1].piece = Piece::None;

        let start_sq = *chess._get_square_from_str("c", "1");
        let end_sq = *chess._get_square_from_str("g", "5");

        assert!(move_top_right(&start_sq, &end_sq, &chess));

        //D2 is now blocked
        chess.board[3][1].piece = Piece::Queen(PieceColor::Black);
        assert!(!move_top_right(&start_sq, &end_sq, &chess));

        //b2 is not empty
        let end_sq = *chess._get_square_from_str("a", "3");
        assert!(!move_top_left(&start_sq, &end_sq, &chess));
        chess.board[1][1].piece = Piece::None;
        assert!(move_top_left(&start_sq, &end_sq, &chess));

        //Bishop on F2
        chess.board[5][1].piece = Piece::Bishop(PieceColor::Black);
        let start_sq = *chess._get_square_from_str("f", "2");
        let end_sq = *chess._get_square_from_str("a", "7");
        assert!(move_top_left(&start_sq, &end_sq, &chess));

        chess.board[4][2].piece = Piece::Bishop(PieceColor::Black);
        assert!(!move_top_left(&start_sq, &end_sq, &chess));
        chess.board[4][2].piece = Piece::None;
        chess.board[1][5].piece = Piece::Bishop(PieceColor::Black);
        assert!(!move_top_left(&start_sq, &end_sq, &chess));

        let start_sq = *chess._get_square_from_str("b", "6");
        let end_sq = *chess._get_square_from_str("f", "2");
        assert!(move_down_right(&start_sq, &end_sq, &chess));
        let end_sq = *chess._get_square_from_str("g", "1");
        assert!(!move_down_right(&start_sq, &end_sq, &chess));

        //Bishop on H8 and pawn on G7 blocks it
        chess.board[7][7].piece = Piece::Bishop(PieceColor::White);
        let start_sq = *chess._get_square_from_str("h", "8");
        let end_sq = *chess._get_square_from_str("a", "1");
        assert!(!move_down_left(&start_sq, &end_sq, &chess));
        let end_sq = *chess._get_square_from_str("c", "3");
        assert!(!move_down_left(&start_sq, &end_sq, &chess));

        chess.board[6][6].piece = Piece::None;

        assert!(move_down_left(&start_sq, &end_sq, &chess));
        let end_sq = *chess._get_square_from_str("a", "1");
        assert!(move_down_left(&start_sq, &end_sq, &chess));
    }
}
