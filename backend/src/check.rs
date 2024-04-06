use crate::{
    chessboard::{get_black_king, get_white_king, ChessBoard},
    moves::bishop::BISHOP_MOVES,
    moves::{knight::KNIGHT_MOVES, rook::ROOK_MOVES},
    piece::{Piece, PieceColor},
};

pub fn is_king_in_check_state(chess_board: &ChessBoard, king_color: PieceColor) -> bool {
    let king_sq = match king_color {
        PieceColor::White => {
            get_white_king(chess_board).unwrap_or_else(|| panic!("White king not found"))
        }
        PieceColor::Black => {
            get_black_king(chess_board).unwrap_or_else(|| panic!("Black king not found"))
        }
        PieceColor::None => panic!("King color is neither white nor black"),
    };

    let king_file = king_sq.file as i8;
    let king_rank = king_sq.rank as i8;

    let pawn_check = match king_color {
        PieceColor::White => check_by_black_pawn(king_file, king_rank, chess_board),
        PieceColor::Black => check_by_white_pawn(king_file, king_rank, chess_board),
        PieceColor::None => panic!("King color is neither white nor black"),
    };

    pawn_check
        || bishop_or_queen_check(king_file, king_rank, king_color, chess_board)
        || knight_check(king_file, king_rank, king_color, chess_board)
        || rook_or_queen_check(king_file, king_rank, king_color, chess_board)
}

fn get_opponent_color(color: PieceColor) -> PieceColor {
    match color {
        PieceColor::White => PieceColor::Black,
        PieceColor::Black => PieceColor::White,
        PieceColor::None => panic!("Color is none"),
    }
}

fn is_piece_rook_or_queen(piece: Piece, opponent_color: PieceColor) -> bool {
    match piece {
        Piece::Rook(color) | Piece::Queen(color) => color == opponent_color,
        _ => false,
    }
}

fn is_piece_bishop_or_queen(piece: Piece, opponent_color: PieceColor) -> bool {
    match piece {
        Piece::Bishop(color) | Piece::Queen(color) => color == opponent_color,
        _ => false,
    }
}

/**
 * also tests vertical and horizontal queen movement
 */
fn rook_or_queen_check(
    king_file: i8,
    king_rank: i8,
    king_color: PieceColor,
    chess_board: &ChessBoard,
) -> bool {
    let opponent_color = get_opponent_color(king_color);

    ROOK_MOVES.iter().any(|(file, rank)| {
        let mut test_file: i8 = king_file + *file;
        let mut test_rank: i8 = king_rank + *rank;

        while (0..=7).contains(&test_file) && (0..=7).contains(&test_rank) {
            #[allow(clippy::cast_sign_loss)]
            let sq = &chess_board[test_file as usize][test_rank as usize];

            if sq.has_piece() && is_piece_rook_or_queen(sq.piece, opponent_color) {
                return true;
            } else if sq.has_piece() {
                break;
            }

            test_file += *file;
            test_rank += *rank;
        }

        false
    })
}

fn check_by_white_pawn(king_file: i8, king_rank: i8, board: &ChessBoard) -> bool {
    if king_rank > 0 {
        #[allow(clippy::cast_sign_loss)]
        let right_attack = king_file < 7
            && board[king_file as usize + 1][king_rank as usize - 1].piece
                == Piece::Pawn(PieceColor::White);
        #[allow(clippy::cast_sign_loss)]
        let left_attack = king_file > 0
            && board[king_file as usize - 1][king_rank as usize - 1].piece
                == Piece::Pawn(PieceColor::White);
        if right_attack || left_attack {
            // println!("in check by white pawn");
        }
        right_attack || left_attack
    } else {
        false
    }
}

fn check_by_black_pawn(king_file: i8, king_rank: i8, chess_board: &ChessBoard) -> bool {
    if king_rank < 7 {
        #[allow(clippy::cast_sign_loss)]
        let right_attack = king_file > 0
            && chess_board[king_file as usize - 1][king_rank as usize + 1].piece
                == Piece::Pawn(PieceColor::Black);
        #[allow(clippy::cast_sign_loss)]
        let left_attack = king_file < 7
            && chess_board[king_file as usize + 1][king_rank as usize + 1].piece
                == Piece::Pawn(PieceColor::Black);
        if right_attack || left_attack {
            // println!("in check by black pawn");
        }
        right_attack || left_attack
    } else {
        false
    }
}
/**
 * also tests diagonal queen movement
 */
fn bishop_or_queen_check(
    king_file: i8,
    king_rank: i8,
    king_color: PieceColor,
    chess_board: &ChessBoard,
) -> bool {
    let opponent_color = get_opponent_color(king_color);

    BISHOP_MOVES.iter().any(|(file, rank)| {
        let mut test_file = king_file + file;
        let mut test_rank = king_rank + rank;

        while (0..=7).contains(&test_file) && (0..=7).contains(&test_rank) {
            #[allow(clippy::cast_sign_loss)]
            let sq = &chess_board[test_file as usize][test_rank as usize];

            if sq.has_piece() && is_piece_bishop_or_queen(sq.piece, opponent_color) {
                // println!("in check by {:?}", sq.piece.name());
                return true;
            } else if sq.has_piece() {
                break;
            }

            test_file += file;
            test_rank += rank;
        }

        false
    })
}

fn knight_check(
    king_file: i8,
    king_rank: i8,
    king_color: PieceColor,
    chess_board: &ChessBoard,
) -> bool {
    let opponent_color = get_opponent_color(king_color);

    KNIGHT_MOVES.iter().any(|(file, rank)| {
        let test_file = king_file + file;
        let test_rank = king_rank + rank;

        if (0..=7).contains(&test_file) && (0..=7).contains(&test_rank) {
            #[allow(clippy::cast_sign_loss)]
            let sq = &chess_board[test_file as usize][test_rank as usize];
            if sq.has_piece() && sq.piece == Piece::Knight(opponent_color) {
                // println!("in check by {:?}", sq.piece.name());
            }
            sq.has_piece() && sq.piece == Piece::Knight(opponent_color)
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        check::{
            bishop_or_queen_check, check_by_black_pawn, check_by_white_pawn, rook_or_queen_check,
        },
        chess::Chess,
        piece::{Piece, PieceColor},
    };

    const BLACK: PieceColor = PieceColor::Black;
    const WHITE: PieceColor = PieceColor::White;

    #[test]
    fn test_black_pawn_check() {
        let mut chess = Chess::_new();
        let king_file = 0;
        let king_rank = 0;
        assert!(!check_by_black_pawn(king_file, king_rank, &chess.board));
        chess.board[1][0].piece = Piece::Pawn(BLACK);
        assert!(!check_by_black_pawn(king_file, king_rank, &chess.board));
        chess.board[0][1].piece = Piece::Pawn(BLACK);
        assert!(!check_by_black_pawn(king_file, king_rank, &chess.board));

        chess.board[1][1].piece = Piece::Pawn(BLACK);

        assert!(check_by_black_pawn(king_file, king_rank, &chess.board));
        chess.board[1][1].piece = Piece::Pawn(PieceColor::Black);
        chess.board[1][1].piece = Piece::Pawn(PieceColor::Black);
        chess.board[6][1].piece = Piece::Pawn(PieceColor::Black);
        chess.board[7][1].piece = Piece::Pawn(PieceColor::Black);

        assert!(!check_by_black_pawn(5, 5, &chess.board));
        chess.board[4][6].piece = Piece::Pawn(PieceColor::Black);
        assert!(check_by_black_pawn(5, 5, &chess.board));
    }

    #[test]
    fn test_check_by_white_pawn() {
        let chess = Chess::_new();
        let mut board = chess.board;
        board[5][3].piece = Piece::Pawn(PieceColor::White);
        board[7][7].piece = Piece::Pawn(PieceColor::White);

        assert!(!check_by_white_pawn(0, 2, &board));
        board[1][1].piece = Piece::Pawn(PieceColor::White);
        assert!(check_by_white_pawn(0, 2, &board));
        assert!(check_by_white_pawn(6, 4, &board));
        assert!(check_by_white_pawn(8, 8, &board));
    }

    #[test]
    fn bishop_check_tests() {
        let mut chess = Chess::_new();
        chess.board[6][6].piece = Piece::Bishop(BLACK);
        let king_file = 0;
        let king_rank = 0;

        assert!(bishop_or_queen_check(
            king_file,
            king_rank,
            WHITE,
            &chess.board
        ));
        assert!(!bishop_or_queen_check(
            king_file,
            king_rank,
            BLACK,
            &chess.board
        ));
        chess.board[5][5].piece = Piece::Knight(WHITE);
        assert!(!bishop_or_queen_check(
            king_file,
            king_rank,
            WHITE,
            &chess.board
        ));

        let king_file = 7;
        let king_rank = 0;
        assert!(!bishop_or_queen_check(
            king_file,
            king_rank,
            WHITE,
            &chess.board
        ));
        chess.board[0][7].piece = Piece::Bishop(BLACK);
        assert!(bishop_or_queen_check(
            king_file,
            king_rank,
            WHITE,
            &chess.board
        ));
        assert!(!bishop_or_queen_check(
            king_file,
            king_rank,
            BLACK,
            &chess.board
        ));
    }

    #[test]
    fn test_rook_check() {
        let chess = Chess::_new();
        let mut board = chess.board;
        board[1][1].piece = Piece::Rook(PieceColor::White);

        assert!(rook_or_queen_check(1, 4, PieceColor::Black, &board));
        board[1][2].piece = Piece::Rook(PieceColor::Black);
        assert!(!rook_or_queen_check(1, 4, PieceColor::Black, &board));

        assert!(rook_or_queen_check(7, 1, PieceColor::Black, &board));
        assert!(!rook_or_queen_check(6, 6, PieceColor::Black, &board));

        board[6][1].piece = Piece::Rook(PieceColor::White);

        assert!(rook_or_queen_check(6, 6, PieceColor::Black, &board));
    }
}
