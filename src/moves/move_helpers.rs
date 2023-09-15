use crate::chessboard::square::Square;

pub fn diagonally_one_square_apart(first_sq: &Square, second_sq: &Square) -> bool {
    let (first_sq_file, first_sq_rank) = (first_sq.file as u8, first_sq.rank as u8);
    let (second_sq_file, second_sq_rank) = (second_sq.file as u8, second_sq.rank as u8);

    first_sq_rank.abs_diff(second_sq_rank) == 1 && first_sq_file.abs_diff(second_sq_file) == 1
}

pub fn squares_on_same_row(first_sq: &Square, second_sq: &Square) -> bool {
    let (first_sq_file, first_sq_rank) = (first_sq.file as u8, first_sq.rank as u8);
    let (second_sq_file, second_sq_rank) = (second_sq.file as u8, second_sq.rank as u8);

    first_sq_rank.abs_diff(second_sq_rank) == 0 && first_sq_file.abs_diff(second_sq_file) == 1
}

pub fn square_row_diff(first_sq: &Square, second_sq: &Square) -> u8 {
    let first_sq_file = first_sq.file as u8;
    let second_sq_file = second_sq.file as u8;
    first_sq_file.abs_diff(second_sq_file)
}

pub fn square_column_diff(first_sq: &Square, second_sq: &Square) -> u8 {
    let first_sq_rank = first_sq.rank as u8;
    let second_sq_rank = second_sq.rank as u8;

    first_sq_rank.abs_diff(second_sq_rank)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn diagonally_one_square_apart_tests() {
        let sq_1 = Square::new_without_piece(0, 0);
        let sq_2 = Square::new_without_piece(1, 1);
        assert_eq!(diagonally_one_square_apart(&sq_1, &sq_2), true);
        let sq_1 = Square::new_without_piece(0, 0);
        let sq_2 = Square::new_without_piece(2, 1);
        assert_eq!(diagonally_one_square_apart(&sq_1, &sq_2), false);
        let sq_1 = Square::new_without_piece(6, 6);
        let sq_2 = Square::new_without_piece(5, 5);
        assert_eq!(diagonally_one_square_apart(&sq_1, &sq_2), true);
        let sq_1 = Square::new_without_piece(2, 3);
        let sq_2 = Square::new_without_piece(3, 3);
        assert_eq!(diagonally_one_square_apart(&sq_1, &sq_2), false);
        let sq_1 = Square::new_without_piece(5, 5);
        let sq_2 = Square::new_without_piece(6, 4);
        assert_eq!(diagonally_one_square_apart(&sq_1, &sq_2), true);
        let sq_1 = Square::new_without_piece(5, 5);
        let sq_2 = Square::new_without_piece(4, 6);
        assert_eq!(diagonally_one_square_apart(&sq_1, &sq_2), true);
    }

    #[test]
    fn squares_on_same_row_tests() {
        let sq_1 = Square::new_without_piece(0, 0);
        let sq_2 = Square::new_without_piece(1, 0);
        assert_eq!(squares_on_same_row(&sq_1, &sq_2), true);
        let sq_1 = Square::new_without_piece(0, 0);
        let sq_2 = Square::new_without_piece(1, 1);
        assert_eq!(squares_on_same_row(&sq_1, &sq_2), false);
        let sq_1 = Square::new_without_piece(5, 6);
        let sq_2 = Square::new_without_piece(5, 5);
        assert_eq!(squares_on_same_row(&sq_1, &sq_2), false);
        let sq_1 = Square::new_without_piece(2, 3);
        let sq_2 = Square::new_without_piece(3, 3);
        assert_eq!(squares_on_same_row(&sq_1, &sq_2), true);
        let sq_1 = Square::new_without_piece(5, 5);
        let sq_2 = Square::new_without_piece(6, 4);
        assert_eq!(squares_on_same_row(&sq_1, &sq_2), false);
        let sq_1 = Square::new_without_piece(5, 5);
        let sq_2 = Square::new_without_piece(4, 5);
        assert_eq!(squares_on_same_row(&sq_1, &sq_2), true);
    }

    #[test]
    fn square_row_diff_tests() {
        //ranks dont matter
        let sq_1 = Square::new_without_piece(0, 0);
        let sq_2 = Square::new_without_piece(1, 0);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 1);
        let sq_2 = Square::new_without_piece(1, 1);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 1);
        let sq_2 = Square::new_without_piece(2, 1);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 2);
        let sq_2 = Square::new_without_piece(3, 2);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 3);
        let sq_2 = Square::new_without_piece(3, 4);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 3);
        let sq_2 = Square::new_without_piece(4, 5);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 4);
        let sq_2 = Square::new_without_piece(5, 7);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 5);
        let sq_2 = Square::new_without_piece(6, 6);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 6);
        let sq_2 = Square::new_without_piece(7, 5);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 7);
        let sq_2 = Square::new_without_piece(7, 2);
        assert_eq!(square_row_diff(&sq_1, &sq_2), 7);
    }

    #[test]
    fn square_column_diff_tests() {
        //files dont matter
        let sq_1 = Square::new_without_piece(0, 0);
        let sq_2 = Square::new_without_piece(0, 1);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 1);
        let sq_2 = Square::new_without_piece(1, 2);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 2);
        let sq_2 = Square::new_without_piece(1, 3);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 3);
        let sq_2 = Square::new_without_piece(2, 4);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 4);
        let sq_2 = Square::new_without_piece(3, 5);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 5);
        let sq_2 = Square::new_without_piece(4, 6);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 6);
        let sq_2 = Square::new_without_piece(5, 7);
        assert_eq!(square_column_diff(&sq_1, &sq_2), 7);
    }
}
