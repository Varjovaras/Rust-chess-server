use crate::{
    chess::Chess,
    chessboard::{
        get_squares_with_black_pieces, get_squares_with_white_pieces, rank::Rank, square::Square,
    },
    game_state::GameState,
    moves::{bishop::BISHOP_MOVES, knight::KNIGHT_MOVES, rook::ROOK_MOVES},
    piece::{Piece, PieceColor},
};

const WHITE: PieceColor = PieceColor::White;
const BLACK: PieceColor = PieceColor::Black;

type SquareCoordinates = (usize, usize);
pub type MoveFromCoordinates = (SquareCoordinates, SquareCoordinates);

pub fn position_is_checkmate(chess: &mut Chess) -> bool {
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    if chess.white_player.in_check() {
        possible_moves.append(&mut white_possible_moves(chess));
    } else if chess.black_player.in_check() {
        possible_moves.append(&mut black_possible_moves(chess));
    } else {
        return false;
    };

    if possible_moves.is_empty() {
        return false;
    }

    // println!("Possible moves: {:?}", possible_moves);

    for possible_move in possible_moves.iter() {
        let start_sq = chess.board[possible_move.0 .0][possible_move.0 .1];
        let end_sq = chess.board[possible_move.1 .0][possible_move.1 .1];

        if chess.king_is_in_check_after_move(&start_sq, &end_sq) {
            return false;
        }
    }
    chess.gamestate = if chess.white_player.in_check() {
        chess.black_player.victory = true;
        GameState::BlackVictory
    } else {
        chess.white_player.victory = true;
        GameState::WhiteVictory
    };
    true
}

pub fn _position_is_checkmate_test(chess: &mut Chess) -> bool {
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    if chess.white_player.in_check() {
        possible_moves.append(&mut white_possible_moves(chess));
    } else if chess.black_player.in_check() {
        possible_moves.append(&mut black_possible_moves(chess));
    } else {
        return false;
    };

    if possible_moves.is_empty() {
        return false;
    }

    // println!("Possible moves: {:?}", possible_moves);

    for possible_move in possible_moves.iter() {
        let start_sq = chess.board[possible_move.0 .0][possible_move.0 .1];
        let end_sq = chess.board[possible_move.1 .0][possible_move.1 .1];

        if chess.king_is_in_check_after_move(&start_sq, &end_sq) {
            println!("Move {:?} removes check", possible_move);
            println!("start_sq: {:?}", start_sq);
            println!("end_sq: {:?}", end_sq);
            return false;
        }
    }
    chess.gamestate = if chess.white_player.in_check() {
        chess.black_player.victory = true;
        GameState::BlackVictory
    } else {
        chess.white_player.victory = true;
        GameState::WhiteVictory
    };
    true
}

fn black_possible_moves(chess: &Chess) -> Vec<MoveFromCoordinates> {
    let chessboard = chess.board;
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    let black_pieces = get_squares_with_black_pieces(&chessboard);

    for sq in black_pieces.iter() {
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

pub fn white_possible_moves(chess: &Chess) -> Vec<MoveFromCoordinates> {
    let chessboard = chess.board;
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    let white_pieces = get_squares_with_white_pieces(&chessboard);

    for sq in white_pieces.iter() {
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
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    let file = sq.file as usize;
    let rank = sq.rank as usize;
    if sq.piece.color() == &WHITE {
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

    if sq.piece.color() == &BLACK {
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
    possible_moves
}

fn knight_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();

    let knight_moves = KNIGHT_MOVES;

    for knight_move in knight_moves.iter() {
        let file = sq.file as isize + knight_move.0;
        let rank = sq.rank as isize + knight_move.1;
        if (0..=7).contains(&file) && (0..=7).contains(&rank) {
            possible_moves.push((
                (sq.file as usize, sq.rank as usize),
                (file as usize, rank as usize),
            ));
        }
    }

    possible_moves
}

fn bishop_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    for bishop_move in BISHOP_MOVES.iter() {
        let mut file = sq.file as isize + bishop_move.0;
        let mut rank = sq.rank as isize + bishop_move.1;
        while (0..=7).contains(&file) && (0..=7).contains(&rank) {
            possible_moves.push((
                (sq.file as usize, sq.rank as usize),
                (file as usize, rank as usize),
            ));
            file += bishop_move.0;
            rank += bishop_move.1;
        }
    }
    possible_moves
}

fn rook_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
    for rook_move in ROOK_MOVES.iter() {
        let mut file = sq.file as isize + rook_move.0;
        let mut rank = sq.rank as isize + rook_move.1;
        while (0..=7).contains(&file) && (0..=7).contains(&rank) {
            possible_moves.push((
                (sq.file as usize, sq.rank as usize),
                (file as usize, rank as usize),
            ));
            file += rook_move.0;
            rank += rook_move.1;
        }
    }
    possible_moves
}

fn king_possible_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let mut possible_moves: Vec<MoveFromCoordinates> = Vec::new();
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

    for king_move in king_moves.iter() {
        let file = sq.file as isize + king_move.0;
        let rank = sq.rank as isize + king_move.1;
        if (0..=7).contains(&file) && (0..=7).contains(&rank) {
            possible_moves.push((
                (sq.file as usize, sq.rank as usize),
                (file as usize, rank as usize),
            ));
        }
    }

    possible_moves
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
        chess.print_board_white();

        chess.starting_position();
        chess.make_move_from_str("e2", "e4");
        chess.make_move_from_str("e2", "e4");
        chess.make_move_from_str("e7", "e5");
        chess.make_move_from_str("d1", "h5");
        chess.make_move_from_str("b8", "c6");
        chess.make_move_from_str("h5", "e5");
        chess.print_board_white();
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

//println!("chess.white_won = {}", chess.white_won);
