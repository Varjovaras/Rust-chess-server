use crate::chessboard::square::Square;

pub fn move_knight(start_sq: &Square, end_sq: &Square) -> bool {
    let start_sq_file = start_sq.file as u8;
    let start_sq_rank = start_sq.rank as u8;
    let end_sq_file = end_sq.file as u8;
    let end_sq_rank = end_sq.rank as u8;

    start_sq_file.abs_diff(end_sq_file) == 1 && start_sq_rank.abs_diff(end_sq_rank) == 2
        || start_sq_file.abs_diff(end_sq_file) == 2 && start_sq_rank.abs_diff(end_sq_rank) == 1
}

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        moves::knight::move_knight,
        piece::{Piece, PieceColor},
    };

    const BLACKHORSE: Piece = Piece::Knight(PieceColor::Black);
    #[test]
    fn knight_move_works() {
        let mut chess = Chess::new();
        chess.starting_position();
        chess.board[4][4].piece = BLACKHORSE;
        let sq1 = chess.board[4][4];
        let sq2 = chess.board[6][3];
        assert!(move_knight(&sq1, &sq2));

        let sq2 = chess.board[6][5];
        assert!(move_knight(&sq1, &sq2));

        let sq2 = chess.board[5][6];
        assert!(move_knight(&sq1, &sq2));

        let sq2 = chess.board[5][2];
        assert!(move_knight(&sq1, &sq2));

        let sq2 = chess.board[3][6];
        assert!(move_knight(&sq1, &sq2));

        let sq2 = chess.board[3][2];
        assert!(move_knight(&sq1, &sq2));

        let sq2 = chess.board[2][5];
        assert!(move_knight(&sq1, &sq2));

        let sq2 = chess.board[2][3];
        assert!(move_knight(&sq1, &sq2));
    }
}
