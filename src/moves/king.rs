use crate::{chess::Chess, chessboard::square::Square};

use super::move_helpers::helpers::{_is_diagonal, _is_horizontal, _is_vertical};

pub fn _move_king(_start_sq: &Square, _end_sq: &Square, _chess: &Chess) -> bool {
    if _is_vertical(_start_sq, _end_sq) {
        (_start_sq.rank as u8).abs_diff(_end_sq.rank as u8) == 1
    } else if _is_horizontal(_start_sq, _end_sq) {
        (_start_sq.file as u8).abs_diff(_end_sq.file as u8) == 1
    } else if _is_diagonal(_start_sq, _end_sq) {
        (_start_sq.rank as u8).abs_diff(_end_sq.rank as u8)
            == (_start_sq.file as u8).abs_diff(_end_sq.file as u8)
    } else {
        return false;
    }
}

pub mod check {
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
        // let file_directions: [isize; 4] = [-1, 1, 1, -1];
        // let rank_directions: [isize; 4] = [1, 1, -1, -1];

        let opponent_color: PieceColor = if king_color == PieceColor::White {
            PieceColor::Black
        } else {
            PieceColor::White
        };
        //top-left
        for i in 1..8 {
            if white_king_sq_file == 0 || white_king_sq_rank >= 8 {
                break;
            }
            let sq = chess_board[white_king_sq_file - i][white_king_sq_rank + i];

            if sq.has_piece() {
                if sq.piece == Pieces::Bishop(opponent_color)
                    || sq.piece == Pieces::Queen(opponent_color)
                {
                    return true;
                } else {
                    break;
                }
            }
            if white_king_sq_file - i == 0 || white_king_sq_rank + i >= 7 {
                break;
            }
        }
        //top-right
        for i in 1..8 {
            if white_king_sq_file >= 7 || white_king_sq_rank >= 7 {
                break;
            }

            let sq = chess_board[white_king_sq_file + i][white_king_sq_rank + i];

            if sq.has_piece() {
                if sq.piece == Pieces::Bishop(opponent_color)
                    || sq.piece == Pieces::Queen(opponent_color)
                {
                    return true;
                } else {
                    break;
                }
            }

            if white_king_sq_file + i >= 7 || white_king_sq_rank + i >= 7 {
                break;
            }
        }

        //bottom-right
        for i in 1..8 {
            if white_king_sq_file >= 7 || white_king_sq_rank == 0 {
                break;
            }

            let sq = chess_board[white_king_sq_file + i][white_king_sq_rank - i];
            if sq.has_piece() {
                if sq.piece == Pieces::Bishop(opponent_color)
                    || sq.piece == Pieces::Queen(opponent_color)
                {
                    return true;
                } else {
                    break;
                }
            }
            if white_king_sq_file + i >= 7 || white_king_sq_rank - i == 0 {
                break;
            }
        }

        //bottom-left
        for i in 1..8 {
            if white_king_sq_file == 0 || white_king_sq_rank == 0 {
                break;
            }
            let sq = chess_board[white_king_sq_file - i][white_king_sq_rank - i];
            if sq.has_piece() {
                if sq.piece == Pieces::Bishop(opponent_color)
                    || sq.piece == Pieces::Queen(opponent_color)
                {
                    return true;
                } else {
                    break;
                }
            }
            if white_king_sq_file - i == 0 || white_king_sq_rank - i == 0 {
                break;
            }
        }

        false
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            chess::Chess,
            moves::king::check::_bishop_check,
            piece::{PieceColor, Pieces},
        };

        use super::_white_pawn_check;
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
}
