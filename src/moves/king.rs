use crate::{chess::Chess, chessboard::square::Square, piece::PieceColor};

pub fn _move_king(
    _color: &PieceColor,
    _start_sq: &Square,
    _end_sq: &Square,
    _chess: &Chess,
) -> bool {
    true
}

pub mod check {
    use crate::{
        chessboard::{get_white_king, ChessBoard},
        piece::{PieceColor, Pieces},
    };

    pub fn is_white_king_in_check(chess_board: &ChessBoard) -> bool {
        let white_king_sq = get_white_king(chess_board);
        let white_king_sq_file = white_king_sq.file as usize;
        let white_king_sq_rank = white_king_sq.rank as usize;
        white_king_pawn_check(white_king_sq_file, white_king_sq_rank, chess_board)
            || white_king_bishop_test(white_king_sq_file, white_king_sq_rank, chess_board)
    }

    fn white_king_pawn_check(
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

    fn white_king_bishop_test(
        white_king_sq_file: usize,
        white_king_sq_rank: usize,
        chess_board: &ChessBoard,
    ) -> bool {
        // let file_directions: [isize; 4] = [-1, 1, 1, -1];
        // let rank_directions: [isize; 4] = [1, 1, -1, -1];
        let color = PieceColor::Black;
        //top-left
        for i in 1..8 {
            if white_king_sq_file <= 0
                || white_king_sq_file - i <= 0
                || white_king_sq_rank >= 7
                || white_king_sq_rank + i >= 7
            {
                break;
            }
            let sq = chess_board[white_king_sq_file - i][white_king_sq_rank + i];
            if sq.has_piece() && sq.piece != Pieces::Bishop(color)
                || sq.piece != Pieces::Queen(color)
            {
                break;
            } else {
                return true;
            }
        }
        //top-right
        for i in 1..8 {
            if white_king_sq_file >= 7
                || white_king_sq_file + i >= 7
                || white_king_sq_rank >= 7
                || white_king_sq_rank + i >= 7
            {
                break;
            }
            let sq = chess_board[white_king_sq_file + i][white_king_sq_rank + i];
            if sq.has_piece() && sq.piece == Pieces::Bishop(color)
                || sq.piece == Pieces::Queen(color)
            {
                return true;
            } else {
                break;
            }
        }

        //bottom-right
        for i in 1..8 {
            if white_king_sq_file >= 7
                || white_king_sq_file + i >= 7
                || white_king_sq_rank <= 0
                || white_king_sq_rank - i <= 0
            {
                break;
            }
            let sq = chess_board[white_king_sq_file + i][white_king_sq_rank - i];
            if sq.has_piece() && sq.piece == Pieces::Bishop(color)
                || sq.piece == Pieces::Queen(color)
            {
                return true;
            } else {
                break;
            }
        }

        //bottom-left
        for i in 1..8 {
            if white_king_sq_file <= 0
                || white_king_sq_file - i <= 0
                || white_king_sq_rank <= 0
                || white_king_sq_rank + i <= 0
            {
                break;
            }
            let sq = chess_board[white_king_sq_file - i][white_king_sq_rank - i];
            if sq.has_piece() && sq.piece == Pieces::Bishop(color)
                || sq.piece == Pieces::Queen(color)
            {
                return true;
            } else {
                break;
            }
        }
        true
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            chess::Chess,
            moves::king::check::white_king_bishop_test,
            piece::{PieceColor, Pieces},
        };

        use super::white_king_pawn_check;
        const BLACK: PieceColor = PieceColor::Black;
        const WHITE: PieceColor = PieceColor::White;

        #[test]
        fn white_king_in_check_by_pawn() {
            let mut chess = Chess::new();
            chess.board[7][7].piece = Pieces::Bishop(BLACK);
            let king_file: usize = 0;
            let king_rank: usize = 0;
            let chess_board = &mut chess.board;
            let ches = chess.clone();
            ches._print_board_white();
            assert_eq!(
                white_king_pawn_check(king_file, king_rank, chess_board),
                false
            );
            assert_eq!(
                white_king_bishop_test(king_file, king_rank, chess_board),
                true
            );
            chess_board[6][6].piece = Pieces::Knight(WHITE);
            assert_eq!(
                white_king_bishop_test(king_file, king_rank, chess_board),
                false
            );
        }
    }
}
