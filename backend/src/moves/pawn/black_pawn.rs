use crate::{
    chess::Chess,
    chessboard::{rank::Rank, square::Square},
    moves::move_helpers::helpers::{diagonally_one_square_apart, square_column_diff},
    piece::{Piece, PieceColor},
};

//only en passant affects board, thats why chess is mutable reference
pub fn move_black_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::First {
        return false;
    }
    if end_sq.rank > start_sq.rank {
        // println!("Black pawn is moving backwards");
        return false;
    }
    if start_sq.is_empty() {
        return false;
    }
    if start_sq.rank == Rank::Seventh {
        black_starting_sq_move(start_sq, end_sq, chess)
    } else if diagonally_one_square_apart(start_sq, end_sq) {
        return black_capture(start_sq, end_sq, chess);
    } else if start_sq.file != end_sq.file || square_column_diff(start_sq, end_sq) > 1 {
        return false;
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
        let column_diff = square_column_diff(start_sq, end_sq);
        match column_diff {
            1 => return one_square_forward(end_sq),
            2 => return two_squares_forward(start_sq, end_sq, chess),
            _ => return false,
        };
    }
}

fn one_square_forward(end_sq: &Square) -> bool {
    !end_sq.has_piece()
}

fn two_squares_forward(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let in_between_sq = chess.get_square(start_sq.file, Rank::from(start_sq.rank as u8 - 1));
    !(end_sq.has_piece() || in_between_sq.has_piece())
}

fn black_capture(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::Fourth
        && end_sq.rank == Rank::Third
        && latest_move_enables_black_en_passant(chess, start_sq, end_sq)
        && chess.latest_move.unwrap().0.piece == Piece::Pawn(PieceColor::White)
    {
        return black_en_passant(start_sq, end_sq, chess);
    }

    !end_sq.is_empty()
}

fn black_en_passant(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let mut last_move_sq = *chess.get_square(end_sq.file, start_sq.rank);
    if (last_move_sq.file as u8).abs_diff(end_sq.file as u8) != 1 {
        return false;
    }

    if last_move_sq.is_empty() || last_move_sq.piece.color() == &PieceColor::Black {
        return false;
    }
    if end_sq.has_piece() {
        return false;
    }

    last_move_sq.piece = Piece::default();
    true
}

pub fn latest_move_enables_black_en_passant(
    chess: &Chess,
    start_sq: &Square,
    end_sq: &Square,
) -> bool {
    match chess.latest_move {
        Some(latest_move) => {
            latest_move.0.rank == Rank::Second
                && latest_move.1.rank == Rank::Fourth
                && latest_move.0.piece == Piece::Pawn(latest_move.2)
                && latest_move.1.file == end_sq.file
                && latest_move.1.rank == start_sq.rank
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{chessboard::file::File, moves::pawn::white_pawn::move_white_pawn};

    use super::*;
    #[test]
    fn black_pawn_moves_from_starting_square() {
        let mut chess: Chess = Chess::_new();
        chess.starting_position();

        //Bishop on B7
        chess.board[1][6].piece = Piece::Bishop(PieceColor::Black);

        assert!(move_black_pawn(
            chess.get_square(File::E, Rank::Seventh),
            chess.get_square(File::E, Rank::Sixth),
            &chess
        ));
        assert!(move_black_pawn(
            chess.get_square(File::E, Rank::Seventh),
            chess.get_square(File::E, Rank::Fifth),
            &chess
        ));
        assert!(!move_black_pawn(
            chess.get_square(File::E, Rank::Seventh),
            chess.get_square(File::A, Rank::Sixth),
            &chess
        ));
        assert!(!move_black_pawn(
            chess.get_square(File::E, Rank::Seventh),
            chess.get_square(File::E, Rank::Fourth),
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
            chess.get_square(File::A, Rank::Seventh),
            chess.get_square(File::A, Rank::Fifth),
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
            chess.get_square(File::D, Rank::Sixth),
            chess.get_square(File::D, Rank::Fifth),
            &chess
        ));

        assert!(!move_black_pawn(
            chess.get_square(File::D, Rank::Sixth),
            chess.get_square(File::E, Rank::Sixth),
            &chess
        ));
    }

    #[test]
    fn black_en_passant_works() {
        let mut chess: Chess = Chess::_new();
        chess.starting_position();
        chess.make_move_from_str("e2", "e4");
        chess.make_move_from_str("c7", "c5");
        chess.make_move_from_str("e4", "e5");
        chess.make_move_from_str("d7", "d5");
        // assert!(latest_move_enables_white_en_passant(&chess,start_sq));
        assert!(move_white_pawn(
            chess.get_square(File::E, Rank::Fifth),
            chess.get_square(File::D, Rank::Sixth),
            &chess
        ));

        // //black pawn on e4
        // //white pawn on f4
        // chess.board[4][3].piece = Piece::Pawn(PieceColor::Black);
        // chess.board[5][3].piece = Piece::Pawn(PieceColor::White);

        // assert!(move_black_pawn(
        //     &chess._get_square_from_str("e", "4").clone(),
        //     &chess._get_square_from_str("f", "3").clone(),
        //     &chess
        // ));

        // assert!(!move_black_pawn(
        //     &chess._get_square_from_str("d", "4").clone(),
        //     &chess._get_square_from_str("f", "3").clone(),
        //     &chess
        // ));
        // assert!(!move_black_pawn(
        //     &chess._get_square_from_str("e", "4").clone(),
        //     &chess._get_square_from_str("f", "4").clone(),
        //     &chess
        // ));
        // assert!(!move_black_pawn(
        //     &chess._get_square_from_str("e", "4").clone(),
        //     &chess._get_square_from_str("f", "2").clone(),
        //     &chess
        // ));
        // assert!(!move_black_pawn(
        //     &chess._get_square_from_str("g", "4").clone(),
        //     &chess._get_square_from_str("f", "3").clone(),
        //     &chess
        // ));
    }
}
