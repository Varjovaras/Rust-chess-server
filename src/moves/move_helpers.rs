use crate::chessboard::square::Square;

pub fn diagonally_one_square_apart(first_sq: &Square, second_sq: &Square) -> bool {
    let (first_sq_file, first_sq_rank) = (first_sq.file as u8, first_sq.rank as u8);
    let (second_sq_file, second_sq_rank) = (second_sq.file as u8, second_sq.rank as u8);
    if first_sq_file.abs_diff(second_sq_file) != 1 {
        return false;
    }
    first_sq_rank.abs_diff(second_sq_rank) == 1
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn diagonally_one_square_apart_test() {
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
}
