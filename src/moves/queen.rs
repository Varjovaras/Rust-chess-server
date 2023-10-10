use crate::{chess::Chess, chessboard::square::Square};

use super::{
    bishop,
    move_helpers::helpers::{is_diagonal, is_horizontal, is_vertical},
    rook,
};

pub fn move_queen(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if is_diagonal(start_sq, end_sq) {
        bishop::move_bishob(start_sq, end_sq, chess)
    } else if is_horizontal(start_sq, end_sq) || is_vertical(start_sq, end_sq) {
        rook::move_rook(start_sq, end_sq, chess)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        moves::queen::move_queen,
        piece::{Piece, PieceColor},
    };

    const NONE: Piece = Piece::None;
    const WHITEQUEEN: Piece = Piece::Queen(PieceColor::White);

    #[test]
    fn queen_move_works() {
        let mut chess = Chess::new();
        chess.starting_position();
        chess.board[4][4].piece = WHITEQUEEN;
        let sq1 = chess.board[4][4];
        let sq2 = chess.board[0][0];
        assert!(!move_queen(&sq1, &sq2, &chess));

        let sq2 = chess.board[1][1];
        assert!(move_queen(&sq1, &sq2, &chess));

        chess.board[1][1].piece = NONE;
        let sq2 = chess.board[0][0];
        assert!(move_queen(&sq1, &sq2, &chess));

        let sq2 = chess.board[6][6];
        assert!(move_queen(&sq1, &sq2, &chess));

        let sq2 = chess.board[1][6];
        assert!(!move_queen(&sq1, &sq2, &chess));
        let sq2 = chess.board[2][6];
        assert!(move_queen(&sq1, &sq2, &chess));

        let sq2 = chess.board[7][1];
        assert!(move_queen(&sq1, &sq2, &chess));
    }
}
