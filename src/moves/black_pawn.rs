use crate::{
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
    piece::{PieceColor, Pieces},
};

use super::move_helpers::{diagonally_one_square_apart, square_column_diff};

//only en passant affects board, thats why chess is mutable reference
pub fn move_black_pawn(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if start_sq.is_empty() {
        return false;
    }
    if start_sq.rank == Rank::Second {
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
        File::from(start_sq.file as u8),
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
    let mut last_move_sq = *chess.get_square(
        File::from(end_sq.file as u8),
        Rank::from(start_sq.rank as u8),
    );
    if last_move_sq.is_empty() || last_move_sq.piece.color() == &PieceColor::White {
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

// #[cfg(test)]
// mod tests {
//     use crate::chessboard::square::SquareColor;

//     use super::*;
//     #[test]
//     fn pawn_moves_from_starting_square() {
//         let mut chess: Chess = Chess::new();
//         chess.starting_position();

//         //Bishop on B2
//         chess.board[1][1].piece = Pieces::Bishop(PieceColor::Black);

//         assert_eq!(
//             move_white_pawn(
//                 chess.get_square(File::E, Rank::Second),
//                 chess.get_square(File::E, Rank::Third),
//                 &chess
//             ),
//             true
//         );
//         assert_eq!(
//             move_white_pawn(
//                 chess.get_square(File::E, Rank::Second),
//                 chess.get_square(File::E, Rank::Fourth),
//                 &chess
//             ),
//             true
//         );
//         assert_eq!(
//             move_white_pawn(
//                 chess.get_square(File::E, Rank::Second),
//                 chess.get_square(File::A, Rank::Third),
//                 &chess
//             ),
//             false
//         );
//         assert_eq!(
//             move_white_pawn(
//                 chess.get_square(File::E, Rank::Second),
//                 chess.get_square(File::E, Rank::Fifth),
//                 &chess
//             ),
//             false
//         );
//         println!(
//             "{:?}",
//             move_white_pawn(
//                 chess.get_square(File::A, Rank::Second),
//                 chess.get_square(File::A, Rank::Fourth),
//                 &chess
//             )
//         );
//         assert_eq!(
//             move_white_pawn(
//                 chess.get_square(File::A, Rank::Second),
//                 chess.get_square(File::A, Rank::Fourth),
//                 &chess
//             ),
//             true
//         );
//     }
//     #[test]
//     fn normal_pawn_moves() {
//         let mut chess: Chess = Chess::new();
//         chess.starting_position();

//         //Bishop on B2
//         chess.board[1][1].piece = Pieces::Bishop(PieceColor::Black);
//         chess.board[3][2].piece = Pieces::Pawn(PieceColor::White);

//         assert_eq!(
//             move_white_pawn(
//                 chess.get_square(File::D, Rank::Third),
//                 chess.get_square(File::D, Rank::Fourth),
//                 &chess
//             ),
//             true
//         );

//         assert_eq!(
//             move_white_pawn(
//                 chess.get_square(File::D, Rank::Third),
//                 chess.get_square(File::E, Rank::Third),
//                 &chess
//             ),
//             false
//         );
//     }

//     #[test]
//     fn white_en_passant_works() {
//         let mut chess: Chess = Chess::new();
//         chess.starting_position();
//         chess.latest_move = Some((
//             Square::new(
//                 5,
//                 6,
//                 SquareColor::default(),
//                 Pieces::Pawn(PieceColor::Black),
//             ),
//             Square::new(
//                 5,
//                 4,
//                 SquareColor::default(),
//                 Pieces::Pawn(PieceColor::Black),
//             ),
//             PieceColor::Black,
//         ));

//         chess.board[4][4].piece = Pieces::Pawn(PieceColor::White);
//         chess.board[5][4].piece = Pieces::Pawn(PieceColor::Black);

//         assert_eq!(
//             move_white_pawn(
//                 &chess.get_square_from_str("e", "5").clone(),
//                 &chess.get_square_from_str("f", "6").clone(),
//                 &chess
//             ),
//             true
//         );

//         assert_eq!(
//             move_white_pawn(
//                 &chess.get_square_from_str("d", "5").clone(),
//                 &chess.get_square_from_str("f", "6").clone(),
//                 &chess
//             ),
//             false
//         );
//         assert_eq!(
//             move_white_pawn(
//                 &chess.get_square_from_str("e", "5").clone(),
//                 &chess.get_square_from_str("f", "5").clone(),
//                 &chess
//             ),
//             false
//         );
//         assert_eq!(
//             move_white_pawn(
//                 &chess.get_square_from_str("e", "5").clone(),
//                 &chess.get_square_from_str("f", "7").clone(),
//                 &chess
//             ),
//             false
//         );
//         assert_eq!(
//             move_white_pawn(
//                 &chess.get_square_from_str("g", "5").clone(),
//                 &chess.get_square_from_str("f", "6").clone(),
//                 &chess
//             ),
//             false
//         );
//     }
// }
