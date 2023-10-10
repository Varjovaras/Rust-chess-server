use crate::{
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
    piece::{Piece, PieceColor},
};

use super::move_helpers::helpers::{is_diagonal, is_horizontal, is_vertical};

pub fn move_king(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if move_is_castling(start_sq, end_sq, chess) {
        true
    } else if is_vertical(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8) == 1
    } else if is_horizontal(start_sq, end_sq) {
        (start_sq.file as u8).abs_diff(end_sq.file as u8) == 1
    } else if is_diagonal(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8)
            == (start_sq.file as u8).abs_diff(end_sq.file as u8)
    } else {
        false
    }
}

pub fn move_is_castling(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if !(start_sq.piece == Piece::King(PieceColor::White))
        && !(start_sq.piece == Piece::King(PieceColor::Black))
    {
        return false;
    };

    if !(start_sq.file == File::E
        && (start_sq.file as u8).abs_diff(end_sq.file as u8) == 2
        && (end_sq.file == File::G || end_sq.file == File::C)
        && (start_sq.rank == Rank::First || start_sq.rank == Rank::Eighth)
        && start_sq.rank == end_sq.rank)
    {
        return false;
    }

    let castling = chess.castling;

    match (start_sq.rank, end_sq.file) {
        (Rank::First, File::G) => {
            chess.board[5][0].piece == Piece::None
                && chess.board[6][0].piece == Piece::None
                && castling.white_king_side_castling
        }
        (Rank::First, File::C) => {
            chess.board[1][0].piece == Piece::None
                && chess.board[2][0].piece == Piece::None
                && chess.board[3][0].piece == Piece::None
                && castling.white_queen_side_castling
        }
        (Rank::Eighth, File::G) => {
            chess.board[5][7].piece == Piece::None
                && chess.board[6][7].piece == Piece::None
                && castling.black_king_side_castling
        }
        (Rank::Eighth, File::C) => {
            chess.board[1][7].piece == Piece::None
                && chess.board[2][7].piece == Piece::None
                && chess.board[3][7].piece == Piece::None
                && castling.black_queen_side_castling
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        moves::king::move_king,
        piece::{Piece, PieceColor},
    };

    // use super::*;
    const BLACKKING: Piece = Piece::King(PieceColor::Black);
    const NONE: Piece = Piece::None;

    #[test]
    fn king_move_works() {
        let mut chess = Chess::new();
        chess.starting_position();
        chess.board[4][4].piece = BLACKKING;
        let sq1 = chess.board[4][4];
        let sq2 = chess.board[4][5];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[4][3];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[5][5];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[5][3];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[5][4];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[3][3];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[3][4];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
        let sq2 = chess.board[3][5];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
    }

    #[test]
    fn castling_works() {
        let mut chess = Chess::new();
        chess.starting_position();

        let sq1 = chess.board[4][0];
        let sq2 = chess.board[6][0];
        assert_eq!(move_king(&sq1, &sq2, &chess), false);

        chess.board[5][0].piece = NONE;
        chess.board[6][0].piece = NONE;
        let sq1 = chess.board[4][0];
        let sq2 = chess.board[6][0];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);

        let sq2 = chess.board[7][0];
        assert_eq!(move_king(&sq1, &sq2, &chess), false);

        let sq2 = chess.board[2][0];
        assert_eq!(move_king(&sq1, &sq2, &chess), false);
        chess.board[1][0].piece = NONE;
        chess.board[2][0].piece = NONE;
        chess.board[3][0].piece = NONE;
        assert_eq!(move_king(&sq1, &sq2, &chess), true);

        let sq1 = chess.board[4][7];
        let sq2 = chess.board[6][7];
        assert_eq!(move_king(&sq1, &sq2, &chess), false);

        chess.board[5][7].piece = NONE;
        chess.board[6][7].piece = NONE;
        let sq1 = chess.board[4][7];
        let sq2 = chess.board[6][7];
        assert_eq!(move_king(&sq1, &sq2, &chess), true);

        let sq2 = chess.board[7][7];
        assert_eq!(move_king(&sq1, &sq2, &chess), false);

        let sq2 = chess.board[2][7];
        assert_eq!(move_king(&sq1, &sq2, &chess), false);
        chess.board[1][7].piece = NONE;
        chess.board[2][7].piece = NONE;
        chess.board[3][7].piece = NONE;
        assert_eq!(move_king(&sq1, &sq2, &chess), true);
    }
}
