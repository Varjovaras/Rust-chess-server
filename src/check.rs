use crate::{
    chessboard::{ChessBoard, _get_black_king, _get_white_king},
    piece::{PieceColor, Pieces},
};

pub fn _is_king_in_check(chess_board: &ChessBoard, king_color: PieceColor) -> bool {
    let king_sq = if king_color == PieceColor::White {
        _get_white_king(chess_board)
    } else {
        _get_black_king(chess_board)
    };

    let king_file = king_sq.file as usize;
    let king_rank = king_sq.rank as usize;

    let pawn_check = if king_color == PieceColor::White {
        _check_by_black_pawn(king_file, king_rank, chess_board)
    } else {
        _check_by_white_pawn(king_file, king_rank, chess_board)
    };
    pawn_check
        || _bishop_check(king_file, king_rank, PieceColor::White, chess_board)
        || _knight_check(king_file, king_rank, PieceColor::White, chess_board)
        || rook_check(king_file, king_rank, PieceColor::White, chess_board)
}

fn rook_check(
    king_file: usize,
    king_rank: usize,
    white: PieceColor,
    chess_board: &ChessBoard,
) -> bool {
    let opponent_color: PieceColor = if white == PieceColor::White {
        PieceColor::Black
    } else {
        PieceColor::White
    };
    let rook_moves: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (file, rank) in rook_moves.iter() {
        let mut test_file: isize = king_file as isize + file;
        let mut test_rank: isize = king_rank as isize + rank;

        while (0..=7).contains(&test_file) && (0..=7).contains(&test_rank) {
            let sq = chess_board[test_file as usize][test_rank as usize];

            if sq.has_piece() {
                if sq.piece == Pieces::Rook(opponent_color)
                    || sq.piece == Pieces::Queen(opponent_color)
                {
                    return true;
                } else {
                    break;
                }
            }
            test_file += file;
            test_rank += rank;
        }
    }

    false
}

fn _check_by_white_pawn(king_file: usize, king_rank: usize, board: &ChessBoard) -> bool {
    if king_rank == 0 {
        return false;
    }

    if king_file == 0 {
        board[king_file + 1][king_rank - 1].piece == Pieces::Pawn(PieceColor::White)
    } else if king_file >= 7 {
        board[king_file - 1][king_rank - 1].piece == Pieces::Pawn(PieceColor::White)
    } else {
        board[king_file + 1][king_rank - 1].piece == Pieces::Pawn(PieceColor::White)
            || board[king_file - 1][king_rank - 1].piece == Pieces::Pawn(PieceColor::White)
    }
}

fn _check_by_black_pawn(king_file: usize, king_rank: usize, chess_board: &ChessBoard) -> bool {
    if king_rank >= 7 {
        return false;
    }

    if king_file == 0 {
        chess_board[king_file + 1][king_rank + 1].piece == Pieces::Pawn(PieceColor::Black)
    } else if king_file >= 7 {
        chess_board[king_file - 1][king_rank + 1].piece == Pieces::Pawn(PieceColor::Black)
    } else {
        chess_board[king_file + 1][king_rank + 1].piece == Pieces::Pawn(PieceColor::Black)
            || chess_board[king_file - 1][king_rank + 1].piece == Pieces::Pawn(PieceColor::Black)
    }
}

fn _bishop_check(
    king_file: usize,
    king_rank: usize,
    king_color: PieceColor,
    chess_board: &ChessBoard,
) -> bool {
    let opponent_color: PieceColor = if king_color == PieceColor::White {
        PieceColor::Black
    } else {
        PieceColor::White
    };

    let bishop_moves: [(isize, isize); 4] = [(-1, 1), (1, 1), (1, -1), (-1, -1)];

    for (file, rank) in bishop_moves.iter() {
        let mut test_file: isize = king_file as isize + file;
        let mut test_rank: isize = king_rank as isize + rank;

        while (0..=7).contains(&test_file) && (0..=7).contains(&test_rank) {
            let sq = chess_board[test_file as usize][test_rank as usize];

            if sq.has_piece() {
                if sq.piece == Pieces::Bishop(opponent_color)
                    || sq.piece == Pieces::Queen(opponent_color)
                {
                    return true;
                } else {
                    break;
                }
            }
            test_file += file;
            test_rank += rank;
        }
    }

    false
}

fn _knight_check(
    king_file: usize,
    king_rank: usize,
    king_color: PieceColor,
    chess_board: &ChessBoard,
) -> bool {
    let opponent_color: PieceColor = if king_color == PieceColor::White {
        PieceColor::Black
    } else {
        PieceColor::White
    };
    let knight_moves: [(isize, isize); 8] = [
        (-2, 1),
        (-2, -1),
        (-1, 2),
        (-1, -2),
        (1, 2),
        (1, -2),
        (2, 1),
        (2, -1),
    ];
    for (file, rank) in knight_moves.iter() {
        if king_file as isize + file < 0
            || king_file as isize + file > 7
            || king_rank as isize + rank < 0
            || king_rank as isize + rank > 7
        {
            continue;
        }
        let sq =
            chess_board[(king_file as isize + file) as usize][(king_rank as isize + rank) as usize];
        if sq.has_piece() {
            if sq.piece == Pieces::Knight(opponent_color) {
                return true;
            } else {
                continue;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{
        check::{_bishop_check, _check_by_black_pawn, _check_by_white_pawn, rook_check},
        chess::Chess,
        chessboard::ChessBoard,
        piece::{PieceColor, Pieces},
    };

    const BLACK: PieceColor = PieceColor::Black;
    const WHITE: PieceColor = PieceColor::White;

    #[test]
    fn test_black_pawn_check() {
        let mut chess = Chess::new();
        let king_file: usize = 0;
        let king_rank: usize = 0;
        assert_eq!(
            _check_by_black_pawn(king_file, king_rank, &chess.board),
            false
        );
        chess.board[1][0].piece = Pieces::Pawn(BLACK);
        assert_eq!(
            _check_by_black_pawn(king_file, king_rank, &chess.board),
            false
        );
        chess.board[0][1].piece = Pieces::Pawn(BLACK);
        assert_eq!(
            _check_by_black_pawn(king_file, king_rank, &chess.board),
            false
        );

        chess.board[1][1].piece = Pieces::Pawn(BLACK);

        assert_eq!(
            _check_by_black_pawn(king_file, king_rank, &chess.board),
            true
        );
        chess.board[1][1].piece = Pieces::Pawn(PieceColor::Black);
        chess.board[1][1].piece = Pieces::Pawn(PieceColor::Black);
        chess.board[6][1].piece = Pieces::Pawn(PieceColor::Black);
        chess.board[7][1].piece = Pieces::Pawn(PieceColor::Black);

        assert_eq!(_check_by_black_pawn(5, 5, &chess.board), false);
        chess.board[4][6].piece = Pieces::Pawn(PieceColor::Black);
        assert_eq!(_check_by_black_pawn(5, 5, &chess.board), true);
    }

    #[test]
    fn test_check_by_white_pawn() {
        let chess = Chess::new();
        let mut board = chess.board;
        board[5][3].piece = Pieces::Pawn(PieceColor::White);
        board[7][7].piece = Pieces::Pawn(PieceColor::White);

        assert_eq!(_check_by_white_pawn(0, 2, &board), false);
        board[1][1].piece = Pieces::Pawn(PieceColor::White);
        assert_eq!(_check_by_white_pawn(0, 2, &board), true);
        assert_eq!(_check_by_white_pawn(6, 4, &board), true);
        assert_eq!(_check_by_white_pawn(8, 8, &board), true);
    }

    #[test]
    fn bishop_check_tests() {
        let mut chess = Chess::new();
        chess.board[6][6].piece = Pieces::Bishop(BLACK);
        let king_file: usize = 0;
        let king_rank: usize = 0;

        assert_eq!(
            _bishop_check(king_file, king_rank, WHITE, &chess.board),
            true
        );
        assert_eq!(
            _bishop_check(king_file, king_rank, BLACK, &chess.board),
            false
        );
        chess.board[5][5].piece = Pieces::Knight(WHITE);
        assert_eq!(
            _bishop_check(king_file, king_rank, WHITE, &chess.board),
            false
        );

        let king_file: usize = 7;
        let king_rank: usize = 0;
        assert_eq!(
            _bishop_check(king_file, king_rank, WHITE, &chess.board),
            false
        );
        chess.board[0][7].piece = Pieces::Bishop(BLACK);
        assert_eq!(
            _bishop_check(king_file, king_rank, WHITE, &chess.board),
            true
        );
        assert_eq!(
            _bishop_check(king_file, king_rank, BLACK, &chess.board),
            false
        );
    }

    #[test]
    fn test_rook_check() {
        let chess = Chess::new();
        let mut board = chess.board;
        board[1][1].piece = Pieces::Rook(PieceColor::White);

        assert_eq!(rook_check(1, 4, PieceColor::Black, &board), true);
        board[1][2].piece = Pieces::Rook(PieceColor::Black);
        assert_eq!(rook_check(1, 4, PieceColor::Black, &board), false);

        assert_eq!(rook_check(7, 1, PieceColor::Black, &board), true);
        assert_eq!(rook_check(6, 6, PieceColor::Black, &board), false);

        board[6][1].piece = Pieces::Rook(PieceColor::White);

        assert_eq!(rook_check(6, 6, PieceColor::Black, &board), true);
    }
}
