use crate::{
    chess::Chess,
    chessboard::{rank::Rank, square::Square},
    moves::move_helpers::helpers::{diagonally_one_square_apart, square_column_diff},
    piece::{Piece, PieceColor},
};

//only en passant affects board, that's why chess is mutable reference
#[must_use] pub fn move_black_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::First || end_sq.rank > start_sq.rank || start_sq.is_empty() {
        false
    } else if start_sq.rank == Rank::Seventh {
        black_starting_sq_move(start_sq, end_sq, chess)
    } else if diagonally_one_square_apart(start_sq, end_sq) {
        black_capture(start_sq, end_sq, chess)
    } else if start_sq.file != end_sq.file || square_column_diff(start_sq, end_sq) > 1 {
        false
    } else {
        one_square_forward(end_sq)
    }
}
fn black_starting_sq_move(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if diagonally_one_square_apart(start_sq, end_sq) {
        black_capture(start_sq, end_sq, chess)
    } else if start_sq.file != end_sq.file {
        return false;
    } else {
        match square_column_diff(start_sq, end_sq) {
            1 => one_square_forward(end_sq),
            2 => two_squares_forward(start_sq, end_sq, chess),
            _ => false,
        }
    }
}

fn one_square_forward(end_sq: &Square) -> bool {
    !end_sq.has_piece()
}

fn two_squares_forward(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let in_between_sq = chess.get_square(
        start_sq.file,
        Rank::try_from(start_sq.rank as u8 - 1).expect("Invalid rank"),
    );
    !(end_sq.has_piece() || in_between_sq.has_piece())
}

fn black_capture(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::Fourth
        && end_sq.rank == Rank::Third
        && latest_move_enables_black_en_passant(chess, start_sq, end_sq)
    {
        return black_en_passant(start_sq, end_sq, chess);
    }

    !end_sq.is_empty()
}

fn black_en_passant(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let mut last_move_sq = chess.get_square(end_sq.file, start_sq.rank);
    if last_move_sq.is_empty() || last_move_sq.piece.color() == PieceColor::Black {
        return false;
    }
    if end_sq.has_piece() {
        return false;
    }

    last_move_sq.piece = Piece::default();
    true
}

#[must_use] pub fn latest_move_enables_black_en_passant(
    chess: &Chess,
    start_sq: &Square,
    end_sq: &Square,
) -> bool {
    chess.clone().latest_move.map_or(false, |latest_move| {
        latest_move.0.rank == Rank::Second
            && latest_move.1.rank == Rank::Fourth
            && latest_move.0.piece == Piece::Pawn(PieceColor::White)
            && latest_move.1.file == end_sq.file
            && latest_move.1.rank as usize == start_sq.rank as usize
    })
}

#[cfg(test)]
mod tests {
    use crate::chessboard::file::File;

    use super::*;
    #[test]
    fn black_pawn_moves_from_starting_square() {
        let mut chess: Chess = Chess::_new();
        chess.starting_position();

        //Bishop on B7
        chess.board[1][6].piece = Piece::Bishop(PieceColor::Black);

        assert!(move_black_pawn(
            &chess.get_square(File::E, Rank::Seventh),
            &chess.get_square(File::E, Rank::Sixth),
            &chess
        ));
        assert!(move_black_pawn(
            &chess.get_square(File::E, Rank::Seventh),
            &chess.get_square(File::E, Rank::Fifth),
            &chess
        ));
        assert!(!move_black_pawn(
            &chess.get_square(File::E, Rank::Seventh),
            &chess.get_square(File::A, Rank::Sixth),
            &chess
        ));
        assert!(!move_black_pawn(
            &chess.get_square(File::E, Rank::Seventh),
            &chess.get_square(File::E, Rank::Fourth),
            &chess
        ));
        //     println!(
        //         "{:?}",
        //         move_white_pawn(
        //             chess.get_square(File::A, Rank::Second),
        //             chess.get_square(File::A, Rank::Fourth),
        //             &chess
        //         )
        //     );
        assert!(move_black_pawn(
            &chess.get_square(File::A, Rank::Seventh),
            &chess.get_square(File::A, Rank::Fifth),
            &chess
        ));
    }
    #[test]
    fn normal_pawn_moves() {
        let mut chess: Chess = Chess::_new();
        chess.starting_position();

        //Bishop on B2, black pawn on D6
        chess.board[1][6].piece = Piece::Bishop(PieceColor::White);
        chess.board[3][5].piece = Piece::Pawn(PieceColor::Black);

        assert!(move_black_pawn(
            &chess.get_square(File::D, Rank::Sixth),
            &chess.get_square(File::D, Rank::Fifth),
            &chess
        ));

        assert!(!move_black_pawn(
            &chess.get_square(File::D, Rank::Sixth),
            &chess.get_square(File::E, Rank::Sixth),
            &chess
        ));
    }

    #[test]
    fn black_en_passant_works() {
        let mut chess: Chess = Chess::_new();
        chess.starting_position();
        chess.make_move_from_str("e2", "e4");
        chess.make_move_from_str("d7", "d5");
        chess.make_move_from_str("a2", "a3");
        chess.make_move_from_str("d5", "e4");
        chess.make_move_from_str("f2", "f4");

        assert!(move_black_pawn(
            &chess.get_square(File::E, Rank::Fourth),
            &chess.get_square(File::F, Rank::Third),
            &chess
        ));
    }
}
