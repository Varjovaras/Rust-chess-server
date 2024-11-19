use crate::{
    chess::Chess,
    chessboard::{rank::Rank, square::Square},
    moves::move_helpers::helpers::{diagonally_one_square_apart, square_column_diff},
    piece::{Piece, PieceColor},
};

#[must_use]
pub fn move_white_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::Eighth || start_sq.rank > end_sq.rank || start_sq.is_empty() {
        false
    } else if start_sq.rank == Rank::Second {
        white_starting_sq_move(start_sq, end_sq, chess)
    } else if diagonally_one_square_apart(start_sq, end_sq) {
        white_capture(start_sq, end_sq, chess)
    } else if start_sq.file != end_sq.file || square_column_diff(start_sq, end_sq) > 1 {
        false
    } else {
        one_square_forward(end_sq)
    }
}

fn white_starting_sq_move(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if diagonally_one_square_apart(start_sq, end_sq) {
        white_capture(start_sq, end_sq, chess)
    } else if start_sq.file != end_sq.file {
        return false;
    } else {
        let column_diff = square_column_diff(start_sq, end_sq);
        return match column_diff {
            1 => one_square_forward(end_sq),
            2 => two_squares_forward(start_sq, end_sq, chess),
            _ => false,
        };
    }
}

fn one_square_forward(end_sq: &Square) -> bool {
    !end_sq.has_piece()
}

fn two_squares_forward(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let in_between_sq = chess.get_square(
        start_sq.file,
        Rank::try_from(start_sq.rank as u8 + 1).expect("Invalid rank"),
    );
    !(end_sq.has_piece() || in_between_sq.has_piece())
}

fn white_capture(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if end_sq.piece.color() == PieceColor::White {
        return false;
    }

    if start_sq.rank == Rank::Fifth
        && end_sq.rank == Rank::Sixth
        && latest_move_enables_white_en_passant(chess, start_sq, end_sq)
    {
        return white_en_passant(start_sq, end_sq, chess);
    }

    !end_sq.is_empty()
}

fn white_en_passant(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let mut last_move_sq = chess.get_square(end_sq.file, start_sq.rank);
    if last_move_sq.is_empty() || last_move_sq.piece.color() == PieceColor::White {
        return false;
    }
    if end_sq.has_piece() {
        return false;
    }

    last_move_sq.piece = Piece::default();
    true
}

#[must_use]
pub fn latest_move_enables_white_en_passant(
    chess: &Chess,
    start_sq: &Square,
    end_sq: &Square,
) -> bool {
    chess.clone().latest_move.map_or(false, |latest_move| {
        latest_move.0.rank == Rank::Seventh
            && latest_move.1.rank == Rank::Fifth
            && latest_move.0.piece == Piece::Pawn(latest_move.2)
            && latest_move.1.file == end_sq.file
            && latest_move.1.rank == start_sq.rank
    })
}

#[cfg(test)]
mod tests {
    use crate::chessboard::file::File;

    use super::*;
    #[test]
    fn white_pawn_moves_from_starting_square() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();

        //Bishop on B2
        chess.board[1][1].piece = Piece::Bishop(PieceColor::Black);

        assert!(move_white_pawn(
            &chess.get_square(File::E, Rank::Second),
            &chess.get_square(File::E, Rank::Third),
            &chess
        ));
        assert!(move_white_pawn(
            &chess.get_square(File::E, Rank::Second),
            &chess.get_square(File::E, Rank::Fourth),
            &chess
        ));
        assert!(!move_white_pawn(
            &chess.get_square(File::E, Rank::Second),
            &chess.get_square(File::A, Rank::Third),
            &chess
        ));
        assert!(!move_white_pawn(
            &chess.get_square(File::E, Rank::Second),
            &chess.get_square(File::E, Rank::Fifth),
            &chess
        ));

        assert!(move_white_pawn(
            &chess.get_square(File::A, Rank::Second),
            &chess.get_square(File::A, Rank::Fourth),
            &chess
        ));
    }
    #[test]
    fn white_normal_pawn_moves() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();

        //Bishop on B2
        chess.board[1][1].piece = Piece::Bishop(PieceColor::Black);
        chess.board[3][2].piece = Piece::Pawn(PieceColor::White);

        assert!(move_white_pawn(
            &chess.get_square(File::D, Rank::Third),
            &chess.get_square(File::D, Rank::Fourth),
            &chess
        ));

        assert!(!move_white_pawn(
            &chess.get_square(File::D, Rank::Third),
            &chess.get_square(File::E, Rank::Third),
            &chess
        ));
    }

    #[test]
    fn white_en_passant_works() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();
        chess.make_move_from_str("e2", "e4", None);
        chess.make_move_from_str("c7", "c5", None);
        chess.make_move_from_str("e4", "e5", None);
        chess.make_move_from_str("d7", "d5", None);
        assert!(latest_move_enables_white_en_passant(
            &chess,
            &chess.get_square(File::E, Rank::Fifth),
            &chess.get_square(File::D, Rank::Sixth),
        ));
        assert!(!latest_move_enables_white_en_passant(
            &chess,
            &chess.get_square(File::E, Rank::Fifth),
            &chess.get_square(File::F, Rank::Sixth),
        ));

        assert!(move_white_pawn(
            &chess.get_square(File::E, Rank::Fifth),
            &chess.get_square(File::D, Rank::Sixth),
            &chess
        ));
    }

    #[test]
    fn white_promotion_works_with_queen() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();
        chess.make_move_from_str("e2", "e4", None);
        chess.make_move_from_str("d7", "d5", None);
        chess.make_move_from_str("e4", "d5", None);
        chess.make_move_from_str("e7", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("a7", "a5", None);
        chess.make_move_from_str("e6", "f7", None);
        chess.make_move_from_str("e8", "d7", None);
        chess.make_move_from_str("f7", "g8", Some("QUEEN"));
        chess.print_white_board_to_terminal();
        assert!(chess.get_square(File::G, Rank::Eighth).piece == Piece::Queen(PieceColor::White));
    }
    #[test]
    fn white_promotion_works_with_knight() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();
        chess.make_move_from_str("e2", "e4", None);
        chess.make_move_from_str("d7", "d5", None);
        chess.make_move_from_str("e4", "d5", None);
        chess.make_move_from_str("e7", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("a7", "a5", None);
        chess.make_move_from_str("e6", "f7", None);
        chess.make_move_from_str("e8", "d7", None);
        chess.make_move_from_str("f7", "g8", Some("KNIGHT"));
        assert!(chess.get_square(File::G, Rank::Eighth).piece == Piece::Knight(PieceColor::White));
    }
    #[test]
    fn white_promotion_works_with_bishop() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();
        chess.make_move_from_str("e2", "e4", None);
        chess.make_move_from_str("d7", "d5", None);
        chess.make_move_from_str("e4", "d5", None);
        chess.make_move_from_str("e7", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("a7", "a5", None);
        chess.make_move_from_str("e6", "f7", None);
        chess.make_move_from_str("e8", "d7", None);
        chess.make_move_from_str("f7", "g8", Some("BISHOP"));
        assert!(chess.get_square(File::G, Rank::Eighth).piece == Piece::Bishop(PieceColor::White));
    }

    #[test]
    fn white_promotion_works_with_rook() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();
        chess.make_move_from_str("e2", "e4", None);
        chess.make_move_from_str("d7", "d5", None);
        chess.make_move_from_str("e4", "d5", None);
        chess.make_move_from_str("e7", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("d5", "e6", None);
        chess.make_move_from_str("a7", "a5", None);
        chess.make_move_from_str("e6", "f7", None);
        chess.make_move_from_str("e8", "d7", None);
        chess.make_move_from_str("f7", "g8", Some("ROOK"));
        assert!(chess.get_square(File::G, Rank::Eighth).piece == Piece::Rook(PieceColor::White));
    }
}
