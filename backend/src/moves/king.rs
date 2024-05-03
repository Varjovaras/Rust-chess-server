use crate::{
    check::is_king_in_check_state,
    chess::Chess,
    chessboard::{
        file::File, get_adjacent_squares, get_black_king, get_white_king, rank::Rank,
        square::Square, ChessBoard,
    },
    piece::{Piece, PieceColor},
};

use super::move_helpers::helpers::{is_diagonal, is_horizontal, is_vertical};

pub fn move_piece(start_sq: &Square, end_sq: &Square, chess: &Chess) -> bool {
    if square_is_bordered_by_other_king(&chess.board, start_sq, end_sq) {
        false
    } else if move_is_castling(start_sq, end_sq, chess) {
        true
    } else if is_vertical(start_sq, end_sq) || is_horizontal(start_sq, end_sq) {
        (start_sq.rank as u8).abs_diff(end_sq.rank as u8) == 1
            || (start_sq.file as u8).abs_diff(end_sq.file as u8) == 1
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
                && castling.white.king
                && not_checked_while_castling(5, 0, &chess.board)
        }
        (Rank::First, File::C) => {
            chess.board[1][0].piece == Piece::None
                && chess.board[2][0].piece == Piece::None
                && chess.board[3][0].piece == Piece::None
                && castling.white.queen
                && not_checked_while_castling(3, 0, &chess.board)
        }
        (Rank::Eighth, File::G) => {
            chess.board[5][7].piece == Piece::None
                && chess.board[6][7].piece == Piece::None
                && castling.black.king
                && not_checked_while_castling(5, 7, &chess.board)
        }
        (Rank::Eighth, File::C) => {
            chess.board[1][7].piece == Piece::None
                && chess.board[2][7].piece == Piece::None
                && chess.board[3][7].piece == Piece::None
                && castling.black.queen
                && not_checked_while_castling(3, 7, &chess.board)
        }
        _ => false,
    }
}

fn not_checked_while_castling(
    in_between_king_sq_file: usize,
    in_between_king_sq_rank: usize,
    chess_board: &ChessBoard,
) -> bool {
    let color = if in_between_king_sq_rank == 0 {
        PieceColor::White
    } else {
        PieceColor::Black
    };
    let mut temp_board = chess_board.clone();
    temp_board[4][in_between_king_sq_rank].piece = Piece::None;
    temp_board[in_between_king_sq_file][in_between_king_sq_rank].piece = Piece::King(color);
    !is_king_in_check_state(&temp_board, color)
}

fn square_is_bordered_by_other_king(
    chessboard: &ChessBoard,
    start_sq: &Square,
    end_sq: &Square,
) -> bool {
    let king_color = start_sq.piece.color();
    let enemy_king_sq = match king_color {
        PieceColor::White => get_black_king(chessboard).expect("Black king not found"),
        PieceColor::Black => get_white_king(chessboard).expect("White king not found"),
        PieceColor::None => panic!("King color is neither white nor black"),
    };

    let bordered_squares = get_adjacent_squares(end_sq, chessboard);
    bordered_squares.contains(enemy_king_sq)
}

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        moves::king::move_piece,
        piece::{Piece, PieceColor},
    };

    // use super::*;
    const BLACKKING: Piece = Piece::King(PieceColor::Black);
    const NONE: Piece = Piece::None;

    #[test]
    fn king_move_works() {
        let mut chess = Chess::_new();
        chess.starting_position();
        chess.board[4][4].piece = BLACKKING;
        let sq1 = chess.board[4][4].clone();
        let sq2 = chess.board[4][5].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
        let sq2 = chess.board[4][3].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
        let sq2 = chess.board[5][5].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
        let sq2 = chess.board[5][3].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
        let sq2 = chess.board[5][4].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
        let sq2 = chess.board[3][3].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
        let sq2 = chess.board[3][4].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
        let sq2 = chess.board[3][5].clone();
        assert!(move_piece(&sq1, &sq2, &chess));
    }

    #[test]
    fn castling_works() {
        let mut chess = Chess::_new();
        chess.starting_position();

        let sq1 = chess.board[4][0].clone();
        let sq2 = chess.board[6][0].clone();
        assert!(!move_piece(&sq1, &sq2, &chess));

        chess.board[5][0].piece = NONE;
        chess.board[6][0].piece = NONE;
        let sq1 = chess.board[4][0].clone();
        let sq2 = chess.board[6][0].clone();
        chess._print_board_white();
        assert!(move_piece(&sq1, &sq2, &chess));

        let sq2 = chess.board[7][0].clone();
        assert!(!move_piece(&sq1, &sq2, &chess));

        let sq2 = chess.board[2][0].clone();
        assert!(!move_piece(&sq1, &sq2, &chess));
        chess.board[1][0].piece = NONE;
        chess.board[2][0].piece = NONE;
        chess.board[3][0].piece = NONE;
        chess._print_board_white();
        assert!(move_piece(&sq1, &sq2, &chess));

        let sq1 = chess.board[4][7].clone();
        let sq2 = chess.board[6][7].clone();
        assert!(!move_piece(&sq1, &sq2, &chess));

        chess.board[5][7].piece = NONE;
        chess.board[6][7].piece = NONE;
        let sq1 = chess.board[4][7].clone();
        let sq2 = chess.board[6][7].clone();
        assert!(move_piece(&sq1, &sq2, &chess));

        let sq2 = chess.board[7][7].clone();
        assert!(!move_piece(&sq1, &sq2, &chess));

        let sq2 = chess.board[2][7].clone();
        assert!(!move_piece(&sq1, &sq2, &chess));
        chess.board[1][7].piece = NONE;
        chess.board[2][7].piece = NONE;
        chess.board[3][7].piece = NONE;
        assert!(move_piece(&sq1, &sq2, &chess));
    }
}
