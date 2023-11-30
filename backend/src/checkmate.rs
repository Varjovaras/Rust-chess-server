use crate::{
    chess::Chess,
    chessboard::{
        get_squares_with_black_pieces, get_squares_with_white_pieces, rank::Rank, square::Square,
    },
    game_state::GameState,
    make_move::king_is_not_in_check_after_move,
    moves::{bishop::BISHOP_MOVES, knight::KNIGHT_MOVES, rook::ROOK_MOVES},
    piece::{Piece, PieceColor},
};

const WHITE: PieceColor = PieceColor::White;
const BLACK: PieceColor = PieceColor::Black;

type SquareCoordinates = (usize, usize);
pub type MoveFromCoordinates = (SquareCoordinates, SquareCoordinates);

pub fn position_is_checkmate(chess: &mut Chess) -> bool {
    let moves: Vec<MoveFromCoordinates> = if chess.white_player.in_check() {
        possible_moves(chess, WHITE)
    } else if chess.black_player.in_check() {
        possible_moves(chess, BLACK)
    } else {
        return false;
    };

    let is_checkmate = moves.iter().all(|possible_move| {
        let start_sq = chess.board[possible_move.0 .0][possible_move.0 .1];
        let end_sq = chess.board[possible_move.1 .0][possible_move.1 .1];
        !king_is_not_in_check_after_move(chess, &start_sq, &end_sq)
    });

    if is_checkmate {
        chess.gamestate = if chess.white_player.in_check() {
            chess.black_player.victory = true;
            GameState::BlackVictory
        } else {
            chess.white_player.victory = true;
            GameState::WhiteVictory
        };
    }

    is_checkmate
}

pub fn possible_moves(chess: &Chess, color: PieceColor) -> Vec<MoveFromCoordinates> {
    let chessboard = chess.board;
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    let pieces = match color {
        PieceColor::White => get_squares_with_white_pieces(&chessboard),
        PieceColor::Black => get_squares_with_black_pieces(&chessboard),
        _ => panic!("Invalid color"),
    };

    for sq in pieces.iter() {
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

fn pawn_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let file = sq.file as usize;
    let rank = sq.rank as usize;
    let mut possible_moves = Vec::new();

    match *sq.piece.color() {
        PieceColor::White => {
            if rank == 2 {
                possible_moves.push(((file, rank), (file, Rank::Fourth as usize)));
                possible_moves.push(((file, rank), (file, Rank::Third as usize)));
            } else {
                possible_moves.push(((file, rank), (file, rank + 1)));
            }
            if file < 7 {
                possible_moves.push(((file, rank), (file + 1, rank + 1)));
            }
            if file > 0 {
                possible_moves.push(((file, rank), (file - 1, rank + 1)));
            }
        }
        PieceColor::Black => {
            if rank == 7 {
                possible_moves.push(((file, rank), (file, Rank::Fifth as usize)));
                possible_moves.push(((file, rank), (file, Rank::Sixth as usize)));
            } else {
                possible_moves.push(((file, rank), (file, rank - 1)));
            }
            if file < 7 {
                possible_moves.push(((file, rank), (file + 1, rank - 1)));
            }
            if file > 0 {
                possible_moves.push(((file, rank), (file - 1, rank - 1)));
            }
        }
        _ => {}
    }
    possible_moves
}

fn knight_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    KNIGHT_MOVES
        .iter()
        .filter_map(|knight_move| {
            let file = sq.file as isize + knight_move.0;
            let rank = sq.rank as isize + knight_move.1;
            if (0..=7).contains(&file) && (0..=7).contains(&rank) {
                Some((
                    (sq.file as usize, sq.rank as usize),
                    (file as usize, rank as usize),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn bishop_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    BISHOP_MOVES
        .iter()
        .flat_map(|bishop_move| {
            let mut moves = Vec::new();
            let mut file = sq.file as isize + bishop_move.0;
            let mut rank = sq.rank as isize + bishop_move.1;
            while (0..=7).contains(&file) && (0..=7).contains(&rank) {
                moves.push((
                    (sq.file as usize, sq.rank as usize),
                    (file as usize, rank as usize),
                ));
                file += bishop_move.0;
                rank += bishop_move.1;
            }
            moves
        })
        .collect()
}

fn rook_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    ROOK_MOVES
        .iter()
        .flat_map(|rook_move| {
            let mut moves = Vec::new();
            let mut file = sq.file as isize + rook_move.0;
            let mut rank = sq.rank as isize + rook_move.1;
            while (0..=7).contains(&file) && (0..=7).contains(&rank) {
                moves.push((
                    (sq.file as usize, sq.rank as usize),
                    (file as usize, rank as usize),
                ));
                file += rook_move.0;
                rank += rook_move.1;
            }
            moves
        })
        .collect()
}

fn king_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let king_moves: [(isize, isize); 8] = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    king_moves
        .iter()
        .filter_map(|king_move| {
            let file = sq.file as isize + king_move.0;
            let rank = sq.rank as isize + king_move.1;
            if (0..=7).contains(&file) && (0..=7).contains(&rank) {
                Some((
                    (sq.file as usize, sq.rank as usize),
                    (file as usize, rank as usize),
                ))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::chess::Chess;

    #[test]
    fn checkmate_works() {
        let mut chess = Chess::_new();
        chess.starting_position();
        chess.make_move_from_str("f2", "f3");
        chess.make_move_from_str("e7", "e5");
        chess.make_move_from_str("g2", "g4");
        assert!(!chess.white_player.in_check);
        assert!(!chess.black_player.in_check);
        chess.make_move_from_str("d8", "h4");
        assert!(!chess.black_player.in_check);
        assert!(chess.white_player.in_check);
        assert!(chess.black_player.victory);
        chess._print_board_white();

        chess.starting_position();
        chess.make_move_from_str("e2", "e4");
        chess.make_move_from_str("e2", "e4");
        chess.make_move_from_str("e7", "e5");
        chess.make_move_from_str("d1", "h5");
        chess.make_move_from_str("b8", "c6");
        chess.make_move_from_str("h5", "e5");
        chess._print_board_white();
        assert!(chess.black_player.in_check);
        chess.make_move_from_str("c6", "e7");
        assert!(!chess.black_player.in_check);

        chess.make_move_from_str("f1", "c4");
        chess.make_move_from_str("a7", "a6");
        chess.make_move_from_str("e5", "f4");
        chess.make_move_from_str("a6", "a5");
        chess.make_move_from_str("c4", "f7");
        assert!(chess.black_player.in_check);
        assert!(chess.white_player.victory);
    }
}
