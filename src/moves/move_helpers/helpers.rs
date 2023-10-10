use crate::{
    chess::Chess,
    chessboard::{rank::Rank, square::Square},
    moves::pawn::{
        black_pawn::latest_move_enables_black_en_passant,
        white_pawn::latest_move_enables_white_en_passant,
    },
    piece::{Piece, PieceColor},
};

pub fn diagonally_one_square_apart(first_sq: &Square, second_sq: &Square) -> bool {
    let (first_sq_file, first_sq_rank) = (first_sq.file as u8, first_sq.rank as u8);
    let (second_sq_file, second_sq_rank) = (second_sq.file as u8, second_sq.rank as u8);

    first_sq_rank.abs_diff(second_sq_rank) == 1 && first_sq_file.abs_diff(second_sq_file) == 1
}

pub fn _squares_on_same_row(first_sq: &Square, second_sq: &Square) -> bool {
    let (first_sq_file, first_sq_rank) = (first_sq.file as u8, first_sq.rank as u8);
    let (second_sq_file, second_sq_rank) = (second_sq.file as u8, second_sq.rank as u8);

    first_sq_rank.abs_diff(second_sq_rank) == 0 && first_sq_file.abs_diff(second_sq_file) == 1
}

pub fn _square_row_diff(first_sq: &Square, second_sq: &Square) -> u8 {
    let first_sq_file = first_sq.file as u8;
    let second_sq_file = second_sq.file as u8;
    first_sq_file.abs_diff(second_sq_file)
}

pub fn square_column_diff(first_sq: &Square, second_sq: &Square) -> u8 {
    let first_sq_rank = first_sq.rank as u8;
    let second_sq_rank = second_sq.rank as u8;

    first_sq_rank.abs_diff(second_sq_rank)
}

pub fn is_diagonal(first_sq: &Square, second_sq: &Square) -> bool {
    (first_sq.rank as u8).abs_diff(second_sq.rank as u8)
        == (first_sq.file as u8).abs_diff(second_sq.file as u8)
}

pub fn is_vertical(first_sq: &Square, second_sq: &Square) -> bool {
    first_sq.rank != second_sq.rank && first_sq.file == second_sq.file
}

pub fn is_horizontal(first_sq: &Square, second_sq: &Square) -> bool {
    first_sq.rank == second_sq.rank && first_sq.file != second_sq.file
}

pub fn move_is_up_and_left(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file > end_sq.file && start_sq.rank < end_sq.rank
}

pub fn move_is_up_and_right(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file < end_sq.file && start_sq.rank < end_sq.rank
}

pub fn move_is_down_and_left(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file > end_sq.file && start_sq.rank > end_sq.rank
}

pub fn move_is_down_and_right(start_sq: &Square, end_sq: &Square) -> bool {
    start_sq.file < end_sq.file && start_sq.rank > end_sq.rank
}

pub fn move_is_white_en_passant(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    start_sq.piece == Piece::Pawn(PieceColor::White)
        && diagonally_one_square_apart(start_sq, end_sq)
        && start_sq.rank == Rank::Fifth
        && end_sq.rank == Rank::Sixth
        && latest_move_enables_white_en_passant(chess)
        && !end_sq.has_piece()
}

pub fn move_is_black_en_passant(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    start_sq.piece == Piece::Pawn(PieceColor::Black)
        && diagonally_one_square_apart(start_sq, end_sq)
        && start_sq.rank == Rank::Fourth
        && end_sq.rank == Rank::Third
        && latest_move_enables_black_en_passant(chess)
        && !end_sq.has_piece()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn diagonally_one_square_apart_tests() {
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(1, 1);
        assert!(diagonally_one_square_apart(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(2, 1);
        assert!(!diagonally_one_square_apart(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(6, 6);
        let sq_2 = Square::_new_without_piece(5, 5);
        assert!(diagonally_one_square_apart(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(2, 3);
        let sq_2 = Square::_new_without_piece(3, 3);
        assert!(!diagonally_one_square_apart(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(5, 5);
        let sq_2 = Square::_new_without_piece(6, 4);
        assert!(diagonally_one_square_apart(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(5, 5);
        let sq_2 = Square::_new_without_piece(4, 6);
        assert!(diagonally_one_square_apart(&sq_1, &sq_2));
    }

    #[test]
    fn squares_on_same_row_tests() {
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(1, 0);
        assert!(_squares_on_same_row(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(1, 1);
        assert!(!_squares_on_same_row(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(5, 6);
        let sq_2 = Square::_new_without_piece(5, 5);
        assert!(!_squares_on_same_row(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(2, 3);
        let sq_2 = Square::_new_without_piece(3, 3);
        assert!(_squares_on_same_row(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(5, 5);
        let sq_2 = Square::_new_without_piece(6, 4);
        assert!(!_squares_on_same_row(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(5, 5);
        let sq_2 = Square::_new_without_piece(4, 5);
        assert!(_squares_on_same_row(&sq_1, &sq_2));
    }

    #[test]
    fn square_row_diff_tests() {
        //ranks dont matter
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(1, 0);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 1);
        let sq_2 = Square::_new_without_piece(1, 1);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 1);
        let sq_2 = Square::_new_without_piece(2, 1);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 2);
        let sq_2 = Square::_new_without_piece(3, 2);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 3);
        let sq_2 = Square::_new_without_piece(3, 4);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 3);
        let sq_2 = Square::_new_without_piece(4, 5);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 4);
        let sq_2 = Square::_new_without_piece(5, 7);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 5);
        let sq_2 = Square::_new_without_piece(6, 6);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 6);
        let sq_2 = Square::_new_without_piece(7, 5);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 7);
        let sq_2 = Square::_new_without_piece(7, 2);
        assert_eq!(_square_row_diff(&sq_1, &sq_2), 7);
    }

    #[test]
    fn square_column_diff_tests() {
        //files dont matter
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(0, 1);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 1);
        let sq_2 = Square::_new_without_piece(1, 2);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 2);
        let sq_2 = Square::_new_without_piece(1, 3);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 3);
        let sq_2 = Square::_new_without_piece(2, 4);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 4);
        let sq_2 = Square::_new_without_piece(3, 5);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 5);
        let sq_2 = Square::_new_without_piece(4, 6);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 6);
        let sq_2 = Square::_new_without_piece(5, 7);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 7);
    }

    #[test]
    fn is_diagonal_works() {
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(0, 1);
        assert!(!is_diagonal(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(1, 1);
        assert!(is_diagonal(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(0, 0);
        let sq_2 = Square::_new_without_piece(3, 3);
        assert!(is_diagonal(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(1, 0);
        let sq_2 = Square::_new_without_piece(4, 3);
        assert!(is_diagonal(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(0, 1);
        let sq_2 = Square::_new_without_piece(4, 3);
        assert!(!is_diagonal(&sq_1, &sq_2));
        let sq_1 = Square::_new_without_piece(0, 7);
        let sq_2 = Square::_new_without_piece(7, 0);
        assert!(is_diagonal(&sq_1, &sq_2));
    }
}
