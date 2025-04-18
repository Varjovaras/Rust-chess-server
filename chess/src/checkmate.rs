use crate::{
    chess::Chess,
    chessboard::{
        get_squares_with_black_pieces, get_squares_with_white_pieces, rank::Rank, square::Square,
    },
    game_state::GameState,
    make_chess_move::king_is_not_in_check_after_move,
    moves::{bishop::BISHOP_MOVES, knight::KNIGHT_MOVES, rook::ROOK_MOVES},
    piece::{Piece, PieceColor},
};

const DEFAULT_NO_PROMOTION_TUPLE: (usize, usize) = (0, 0);

const WHITE: PieceColor = PieceColor::White;
const BLACK: PieceColor = PieceColor::Black;

type SquareCoordinates = (usize, usize);
pub type MoveFromCoordinates = (SquareCoordinates, SquareCoordinates, (usize, usize));

pub fn is_checkmate_position(chess: &mut Chess) -> bool {
    let moves: Vec<MoveFromCoordinates> = if chess.clone().get_white_player().in_check() {
        possible_legal_moves(chess, WHITE)
    } else if chess.clone().get_black_player().in_check() {
        possible_legal_moves(chess, BLACK)
    } else {
        return false;
    };

    let is_checkmate = moves.iter().all(|possible_move| {
        let start_sq = &chess.board[possible_move.0 .0][possible_move.0 .1];
        let end_sq = &chess.board[possible_move.1 .0][possible_move.1 .1];
        !king_is_not_in_check_after_move(&*chess, start_sq, end_sq)
    });

    if is_checkmate {
        chess.gamestate = if chess.clone().get_white_player().in_check() {
            chess.players.1.victory = true;
            GameState::BlackVictory
        } else {
            chess.players.0.victory = true;
            GameState::WhiteVictory
        };
    }

    is_checkmate
}
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn possible_legal_moves(chess: &Chess, color: PieceColor) -> Vec<MoveFromCoordinates> {
    let chessboard = chess.board.clone();
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    let pieces = match color {
        PieceColor::White => get_squares_with_white_pieces(&chessboard),
        PieceColor::Black => get_squares_with_black_pieces(&chessboard),
        PieceColor::None => panic!("Invalid color"),
    };

    for sq in &pieces {
        match sq.piece {
            Piece::None => {}
            Piece::Pawn(_) => possible_moves.append(&mut pawn_possible_moves(sq)),
            Piece::Knight(_) => possible_moves.append(&mut knight_possible_moves(sq)),
            Piece::Bishop(_) => possible_moves.append(&mut bishop_possible_moves(sq)),
            Piece::Rook(_) => possible_moves.append(&mut rook_possible_moves(sq)),
            Piece::Queen(_) => {
                possible_moves.append(&mut bishop_possible_moves(sq));
                possible_moves.append(&mut rook_possible_moves(sq));
            }
            Piece::King(_) => possible_moves.append(&mut king_possible_moves(sq)),
        }
    }
    possible_moves
}

#[must_use]
#[allow(clippy::type_complexity)]
pub fn pawn_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let file = sq.file as usize;
    let rank = sq.rank as usize;
    let mut possible_moves: Vec<((usize, usize), (usize, usize), (usize, usize))> = Vec::new();

    if rank == 0 || rank == 8 {
        return possible_moves;
    }

    match sq.piece.color() {
        PieceColor::White => {
            possible_moves = white_pawn_possible_moves_helper(file, rank);
        }
        PieceColor::Black => possible_moves = black_pawn_possible_moves_helper(file, rank),
        PieceColor::None => {}
    }
    possible_moves
}

#[must_use]
pub fn knight_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    KNIGHT_MOVES
        .iter()
        .filter_map(|knight_move| {
            let file = sq.file as i8 + knight_move.0;
            let rank = sq.rank as i8 + knight_move.1;
            if (0..=7).contains(&file) && (0..=7).contains(&rank) {
                Some((
                    (sq.file as usize, sq.rank as usize),
                    #[allow(clippy::cast_sign_loss)]
                    (file as usize, rank as usize),
                    DEFAULT_NO_PROMOTION_TUPLE,
                ))
            } else {
                None
            }
        })
        .collect()
}

#[must_use]
pub fn bishop_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    BISHOP_MOVES
        .iter()
        .flat_map(|bishop_move| {
            let mut moves = Vec::new();
            let mut file = sq.file as i8 + bishop_move.0;
            let mut rank = sq.rank as i8 + bishop_move.1;
            while (0..=7).contains(&file) && (0..=7).contains(&rank) {
                moves.push((
                    (sq.file as usize, sq.rank as usize),
                    #[allow(clippy::cast_sign_loss)]
                    (file as usize, rank as usize),
                    DEFAULT_NO_PROMOTION_TUPLE,
                ));
                file += bishop_move.0;
                rank += bishop_move.1;
            }
            moves
        })
        .collect()
}

#[must_use]
pub fn rook_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    ROOK_MOVES
        .iter()
        .flat_map(|rook_move| {
            let mut moves = Vec::new();
            let mut file = sq.file as i8 + rook_move.0;
            let mut rank = sq.rank as i8 + rook_move.1;
            while (0..=7).contains(&file) && (0..=7).contains(&rank) {
                moves.push((
                    (sq.file as usize, sq.rank as usize),
                    #[allow(clippy::cast_sign_loss)]
                    (file as usize, rank as usize),
                    DEFAULT_NO_PROMOTION_TUPLE,
                ));
                file += rook_move.0;
                rank += rook_move.1;
            }
            moves
        })
        .collect()
}

#[must_use]
pub fn king_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let king_moves: [(i8, i8); 10] = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (2, 0),
        (-2, 0),
    ];

    #[allow(clippy::cast_sign_loss)]
    king_moves
        .iter()
        .filter_map(|king_move| {
            let file = sq.file as i8 + king_move.0;
            let rank = sq.rank as i8 + king_move.1;
            if (0..=7).contains(&file) && (0..=7).contains(&rank) {
                Some((
                    (sq.file as usize, sq.rank as usize),
                    (file as usize, rank as usize),
                    DEFAULT_NO_PROMOTION_TUPLE,
                ))
            } else {
                None
            }
        })
        .collect()
}

const WHITE_PROMOTIONS: [(usize, usize); 4] = [(1, 0), (2, 0), (3, 0), (4, 0)];
const BLACK_PROMOTIONS: [(usize, usize); 4] = [(1, 1), (2, 1), (3, 1), (4, 1)];

#[allow(clippy::type_complexity)]
fn white_pawn_possible_moves_helper(
    file: usize,
    rank: usize,
) -> Vec<((usize, usize), (usize, usize), (usize, usize))> {
    let mut possible_moves = Vec::new();
    //promotion
    if rank == 6 {
        for white_promotion_piece in WHITE_PROMOTIONS {
            possible_moves.push(((file, rank), (file, rank + 1), white_promotion_piece));
            if file < 7 {
                possible_moves.push(((file, rank), (file + 1, rank + 1), white_promotion_piece));
            }
            if file > 0 {
                possible_moves.push(((file, rank), (file - 1, rank + 1), white_promotion_piece));
            }
        }
        return possible_moves;
    }
    //starting square
    if rank == 1 {
        possible_moves.push((
            (file, rank),
            (file, Rank::Third.as_usize()),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
        possible_moves.push((
            (file, rank),
            (file, Rank::Fourth.as_usize()),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
    } else {
        possible_moves.push(((file, rank), (file, rank + 1), DEFAULT_NO_PROMOTION_TUPLE));
    }

    //captures to left
    if file < 7 {
        possible_moves.push((
            (file, rank),
            (file + 1, rank + 1),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
    }
    //captures to right
    if file > 0 {
        possible_moves.push((
            (file, rank),
            (file - 1, rank + 1),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
    }
    possible_moves
}

#[allow(clippy::type_complexity)]
fn black_pawn_possible_moves_helper(
    file: usize,
    rank: usize,
) -> Vec<((usize, usize), (usize, usize), (usize, usize))> {
    let mut possible_moves = Vec::new();
    //promotion
    if rank == 1 {
        for black_promotion_piece in BLACK_PROMOTIONS {
            if file < 7 {
                possible_moves.push(((file, rank), (file + 1, rank - 1), black_promotion_piece));
            }
            if file > 0 {
                possible_moves.push(((file, rank), (file - 1, rank - 1), black_promotion_piece));
            }
            possible_moves.push(((file, rank), (file, rank - 1), black_promotion_piece));
        }
        return possible_moves;
    }
    if rank == 6 {
        possible_moves.push((
            (file, rank),
            (file, Rank::Fifth.as_usize()),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
        possible_moves.push((
            (file, rank),
            (file, Rank::Sixth.as_usize()),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
    } else {
        possible_moves.push(((file, rank), (file, rank - 1), DEFAULT_NO_PROMOTION_TUPLE));
    }

    if file < 7 {
        possible_moves.push((
            (file, rank),
            (file + 1, rank - 1),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
    }
    if file > 0 {
        possible_moves.push((
            (file, rank),
            (file - 1, rank - 1),
            DEFAULT_NO_PROMOTION_TUPLE,
        ));
    }

    possible_moves
}

#[cfg(test)]
mod tests {
    use crate::chess::Chess;

    #[test]
    fn checkmate_works() {
        let mut chess = Chess::default();

        let promoted_piece = None;
        chess.starting_position();
        chess.make_move_from_str("f2", "f3", promoted_piece);
        chess.make_move_from_str("e7", "e5", promoted_piece);
        chess.make_move_from_str("g2", "g4", promoted_piece);
        assert!(!chess.players.0.in_check);
        assert!(!chess.players.1.in_check);
        chess.make_move_from_str("d8", "h4", promoted_piece);
        assert!(!chess.players.1.in_check);
        assert!(chess.players.0.in_check);
        assert!(chess.players.1.victory);
        chess.print_white_board_to_terminal();

        chess.starting_position();
        chess.make_move_from_str("e2", "e4", promoted_piece);
        chess.make_move_from_str("e2", "e4", promoted_piece);
        chess.make_move_from_str("e7", "e5", promoted_piece);
        chess.make_move_from_str("d1", "h5", promoted_piece);
        chess.make_move_from_str("b8", "c6", promoted_piece);
        chess.make_move_from_str("h5", "e5", promoted_piece);
        chess.print_white_board_to_terminal();
        assert!(chess.players.1.in_check);
        chess.make_move_from_str("c6", "e7", promoted_piece);
        assert!(!chess.players.1.in_check);

        chess.make_move_from_str("f1", "c4", promoted_piece);
        chess.make_move_from_str("a7", "a6", promoted_piece);
        chess.make_move_from_str("e5", "f4", promoted_piece);
        chess.make_move_from_str("a6", "a5", promoted_piece);
        chess.make_move_from_str("c4", "f7", promoted_piece);
        assert!(chess.players.1.in_check);
        assert!(chess.players.0.victory);
    }
}
