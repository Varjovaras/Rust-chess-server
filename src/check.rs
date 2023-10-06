use crate::{
    chessboard::{ChessBoard, _get_white_king},
    piece::{PieceColor, Pieces},
};

pub fn _is_white_king_in_check(chess_board: &ChessBoard) -> bool {
    let white_king_sq = _get_white_king(chess_board);
    let white_king_sq_file = white_king_sq.file as usize;
    let white_king_sq_rank = white_king_sq.rank as usize;
    _white_pawn_check(white_king_sq_file, white_king_sq_rank, chess_board)
        || _bishop_check(
            white_king_sq_file,
            white_king_sq_rank,
            PieceColor::White,
            chess_board,
        )
}

fn _white_pawn_check(
    white_king_sq_file: usize,
    white_king_sq_rank: usize,
    chess_board: &ChessBoard,
) -> bool {
    if white_king_sq_rank >= 7 {
        return false;
    }
    if white_king_sq_file == 0 {
        chess_board[white_king_sq_file + 1][white_king_sq_rank + 1].piece
            == Pieces::Pawn(PieceColor::Black)
    } else if white_king_sq_file >= 7 {
        chess_board[white_king_sq_file - 1][white_king_sq_rank + 1].piece
            == Pieces::Pawn(PieceColor::Black)
    } else {
        chess_board[white_king_sq_file + 1][white_king_sq_rank + 1].piece
            == Pieces::Pawn(PieceColor::Black)
            || chess_board[white_king_sq_file - 1][white_king_sq_rank + 1].piece
                == Pieces::Pawn(PieceColor::Black)
    }
}

fn _bishop_check(
    white_king_sq_file: usize,
    white_king_sq_rank: usize,
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
        let mut test_file: isize = white_king_sq_file as isize + file;
        let mut test_rank: isize = white_king_sq_rank as isize + rank;

        while test_file >= 0 && test_file <= 7 && test_rank >= 0 && test_rank <= 7 {
            let sq = chess_board[test_file as usize][test_rank as usize];
            println!("{:?}", sq);

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

fn knight_check(
    white_king_sq_file: usize,
    white_king_sq_rank: usize,
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
        if white_king_sq_file as isize + file < 0
            || white_king_sq_file as isize + file > 7
            || white_king_sq_rank as isize + rank < 0
            || white_king_sq_rank as isize + rank > 7
        {
            continue;
        }
        let sq = chess_board[(white_king_sq_file as isize + file) as usize]
            [(white_king_sq_rank as isize + rank) as usize];
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
        check::{_bishop_check, _white_pawn_check},
        chess::Chess,
        piece::{PieceColor, Pieces},
    };

    const BLACK: PieceColor = PieceColor::Black;
    const WHITE: PieceColor = PieceColor::White;

    #[test]
    fn white_king_in_check_by_pawn() {
        let mut chess = Chess::new();
        chess.board[7][7].piece = Pieces::Bishop(BLACK);
        let king_file: usize = 0;
        let king_rank: usize = 0;
        assert_eq!(_white_pawn_check(king_file, king_rank, &chess.board), false);
        chess.board[1][0].piece = Pieces::Pawn(BLACK);
        assert_eq!(_white_pawn_check(king_file, king_rank, &chess.board), false);
        chess.board[0][1].piece = Pieces::Pawn(BLACK);
        assert_eq!(_white_pawn_check(king_file, king_rank, &chess.board), false);

        chess.board[1][1].piece = Pieces::Pawn(BLACK);

        assert_eq!(_white_pawn_check(king_file, king_rank, &chess.board), true);
    }
    #[test]
    fn white_king_in_check_by_bishop() {
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
}
