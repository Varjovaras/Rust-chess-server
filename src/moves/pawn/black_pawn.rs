use crate::{
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
    moves::move_helpers::{diagonally_one_square_apart, square_column_diff},
    piece::{PieceColor, Pieces},
};

//only en passant affects board, thats why chess is mutable reference
pub fn move_black_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if end_sq.rank > start_sq.rank {
        panic!("Black pawn is moving backwards");
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
    let in_between_sq = chess.get_square(
        File::from(start_sq.file),
        Rank::from(start_sq.rank as u8 - 1),
    );
    !(end_sq.has_piece() || in_between_sq.has_piece())
}

fn black_capture(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.rank == Rank::Fourth
        && end_sq.rank == Rank::Third
        && latest_move_enables_black_en_passant(chess)
    {
        return black_en_passant(start_sq, end_sq, chess);
    }

    !end_sq.is_empty()
}

fn black_en_passant(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    let mut last_move_sq = *chess.get_square(File::from(end_sq.file), Rank::from(start_sq.rank));
    if last_move_sq.is_empty() || last_move_sq.piece.color() == &PieceColor::Black {
        return false;
    }
    if end_sq.has_piece() {
        return false;
    }

    last_move_sq.piece = Pieces::default();
    true
}

fn latest_move_enables_black_en_passant(chess: &Chess) -> bool {
    match chess.latest_move {
        Some(latest_move) => {
            if latest_move.0.rank == Rank::Second
                && latest_move.1.rank == Rank::Fourth
                && latest_move.0.piece == Pieces::Pawn(latest_move.2)
            {
                println!("Move is en passant");
                true
            } else {
                println!("Move is not en passant");
                false
            }
        }
        None => {
            println!("Move is not en passant");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chessboard::square::SquareColor;

    use super::*;
    #[test]
    fn black_pawn_moves_from_starting_square() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();

        //Bishop on B7
        chess.board[1][6].piece = Pieces::Bishop(PieceColor::Black);

        assert_eq!(
            move_black_pawn(
                chess.get_square(File::E, Rank::Seventh),
                chess.get_square(File::E, Rank::Sixth),
                &chess
            ),
            true
        );
        assert_eq!(
            move_black_pawn(
                chess.get_square(File::E, Rank::Seventh),
                chess.get_square(File::E, Rank::Fifth),
                &chess
            ),
            true
        );
        assert_eq!(
            move_black_pawn(
                chess.get_square(File::E, Rank::Seventh),
                chess.get_square(File::A, Rank::Sixth),
                &chess
            ),
            false
        );
        assert_eq!(
            move_black_pawn(
                chess.get_square(File::E, Rank::Seventh),
                chess.get_square(File::E, Rank::Fourth),
                &chess
            ),
            false
        );
        //     println!(
        //         "{:?}",
        //         move_white_pawn(
        //             chess.get_square(File::A, Rank::Second),
        //             chess.get_square(File::A, Rank::Fourth),
        //             &chess
        //         )
        //     );
        assert_eq!(
            move_black_pawn(
                chess.get_square(File::A, Rank::Seventh),
                chess.get_square(File::A, Rank::Fifth),
                &chess
            ),
            true
        );
    }
    #[test]
    fn normal_pawn_moves() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();

        //Bishop on B2, black pawn on D6
        chess.board[1][6].piece = Pieces::Bishop(PieceColor::White);
        chess.board[3][5].piece = Pieces::Pawn(PieceColor::Black);

        assert_eq!(
            move_black_pawn(
                chess.get_square(File::D, Rank::Sixth),
                chess.get_square(File::D, Rank::Fifth),
                &chess
            ),
            true
        );

        assert_eq!(
            move_black_pawn(
                chess.get_square(File::D, Rank::Sixth),
                chess.get_square(File::E, Rank::Sixth),
                &chess
            ),
            false
        );
    }

    #[test]
    fn black_en_passant_works() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();
        chess.latest_move = Some((
            Square::new(
                5,
                1,
                SquareColor::default(),
                Pieces::Pawn(PieceColor::White),
            ),
            Square::new(
                5,
                3,
                SquareColor::default(),
                Pieces::Pawn(PieceColor::White),
            ),
            PieceColor::White,
        ));

        //black pawn on e4
        //white pawn on f4
        chess.board[4][3].piece = Pieces::Pawn(PieceColor::Black);
        chess.board[5][3].piece = Pieces::Pawn(PieceColor::White);

        assert_eq!(
            move_black_pawn(
                &chess.get_square_from_str("e", "4").clone(),
                &chess.get_square_from_str("f", "3").clone(),
                &chess
            ),
            true
        );

        assert_eq!(
            move_black_pawn(
                &chess.get_square_from_str("d", "4").clone(),
                &chess.get_square_from_str("f", "3").clone(),
                &chess
            ),
            false
        );
        assert_eq!(
            move_black_pawn(
                &chess.get_square_from_str("e", "4").clone(),
                &chess.get_square_from_str("f", "4").clone(),
                &chess
            ),
            false
        );
        assert_eq!(
            move_black_pawn(
                &chess.get_square_from_str("e", "4").clone(),
                &chess.get_square_from_str("f", "2").clone(),
                &chess
            ),
            false
        );
        assert_eq!(
            move_black_pawn(
                &chess.get_square_from_str("g", "4").clone(),
                &chess.get_square_from_str("f", "3").clone(),
                &chess
            ),
            false
        );
    }
}
