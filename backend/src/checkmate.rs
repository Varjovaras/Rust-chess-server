use crate::{
    chess::Chess, game_state::GameState, make_move::king_is_not_in_check_after_move,
    piece::PieceColor, possible_moves::possible_moves,
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
