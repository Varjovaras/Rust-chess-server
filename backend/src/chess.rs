use crate::{
    castling::Castling,
    chessboard::{self, file::File, rank::Rank, square::Square, starting_position, ChessBoard},
    game_state::GameState,
    make_move::make_move,
    piece::PieceColor,
    player::Player,
};

use serde::{Deserialize, Serialize};

pub type LatestMove = (Square, Square, PieceColor);
type SquareCoordinates = (File, usize);
pub type Move = (SquareCoordinates, SquareCoordinates);
pub type ListOfMoves = Vec<Move>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chess {
    pub board: ChessBoard,
    pub turn_number: i32,
    pub latest_move: Option<LatestMove>,
    pub castling: Castling,
    pub white_player: Player,
    pub black_player: Player,
    pub gamestate: GameState,
    pub fifty_move_rule: u8,
    pub list_of_moves: ListOfMoves,
}

impl Chess {
    pub fn _new() -> Self {
        Self {
            board: chessboard::new_board(),
            turn_number: 0,
            latest_move: None,
            castling: Castling::new(),
            white_player: Player::new(PieceColor::White),
            black_player: Player::new(PieceColor::Black),
            gamestate: GameState::InProgress,
            fifty_move_rule: 0,
            list_of_moves: Vec::new(),
        }
    }

    pub fn new_starting_position() -> Self {
        let mut chess = Self {
            board: chessboard::new_board(),
            turn_number: 0,
            latest_move: None,
            castling: Castling::new(),
            white_player: Player::new(PieceColor::White),
            black_player: Player::new(PieceColor::Black),
            gamestate: GameState::InProgress,
            fifty_move_rule: 0,
            list_of_moves: Vec::new(),
        };
        chess.starting_position();
        chess
    }

    pub fn _to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to convert to JSON")
    }

    pub fn _from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn make_move(&mut self, start_sq: Square, end_sq: Square) {
        make_move(self, start_sq, end_sq);
    }

    pub fn starting_position(&mut self) {
        self.board = chessboard::new_board();
        self.board = starting_position();
        self.turn_number = 0;
        self.white_player.victory = false;
        self.black_player.victory = false;
        self.white_player.in_check = false;
        self.black_player.in_check = false;
        self.gamestate = GameState::InProgress;
    }

    pub const fn get_square(&self, file: File, rank: Rank) -> Square {
        self.board[file as usize][rank as usize]
    }

    pub fn get_square_from_str(&mut self, file_str: &str, rank_str: &str) -> &Square {
        let file = File::_from_str_slice(file_str).as_usize();
        let rank = Rank::_from_str(rank_str).as_usize();
        assert!(
            !(file > 7 || rank > 7),
            "get_square_from_str failed for inputting too big file or rank"
        );
        &self.board[file][rank]
    }

    pub fn _print_board_white(&self) {
        let mut clone_board = self.board;
        clone_board.reverse();

        for i in (0..8).rev() {
            for j in (0..8).rev() {
                print!("{} ", clone_board[j][i]._piece_name());
            }
            println!(" ");
        }
    }

    pub fn _print_board_black(&self) {
        let mut clone_board = self.board;
        for square_vec in &mut clone_board {
            square_vec.reverse();
        }
        for row in &clone_board {
            for square in row {
                print!("{} ", square._square_name());
            }
            println!();
        }
    }

    pub fn make_move_from_str(&mut self, start_sq: &str, end_sq: &str) {
        let start_sq_chars: Vec<char> = start_sq.chars().collect();
        let end_sq_chars: Vec<char> = end_sq.chars().collect();
        let start_sq = *self.get_square_from_str(
            start_sq_chars[0].to_string().as_str(),
            start_sq_chars[1].to_string().as_str(),
        );

        let end_sq = *self.get_square_from_str(
            end_sq_chars[0].to_string().as_str(),
            end_sq_chars[1].to_string().as_str(),
        );

        self.make_move(start_sq, end_sq);
    }

    pub fn _print_moves(self) {
        self.list_of_moves.into_iter().for_each(|m| {
            println!("{m:?}");
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::Piece;

    use super::*;
    #[test]
    fn chess_initialization_works() {
        let _chess: Chess = Chess::_new();
    }

    #[test]
    fn make_move_works() {
        let mut chess: Chess = Chess::_new();
        chess.starting_position();
        let start_sq = *chess.get_square_from_str("e", "2");
        let end_sq = *chess.get_square_from_str("e", "4");
        chess.make_move(start_sq, end_sq);

        assert_eq!(chess.get_square_from_str("e", "2").piece, Piece::None);
        assert_eq!(
            chess.get_square_from_str("e", "4").piece,
            Piece::Pawn(PieceColor::White)
        );
        assert_eq!(chess.turn_number, 1);
        assert_eq!(
            chess.latest_move,
            Some((start_sq, end_sq, PieceColor::White))
        );

        let start_sq = *chess.get_square_from_str("e", "4");
        let end_sq = *chess.get_square_from_str("e", "5");
        chess.make_move(start_sq, end_sq);
        assert_eq!(
            chess.get_square_from_str("e", "4").piece,
            Piece::Pawn(PieceColor::White)
        );
        assert_eq!(chess.get_square_from_str("e", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 1);

        let start_sq = *chess.get_square_from_str("e", "7");
        let end_sq = *chess.get_square_from_str("e", "5");
        chess.make_move(start_sq, end_sq);
        assert_eq!(
            chess.get_square_from_str("e", "5").piece,
            Piece::Pawn(PieceColor::Black)
        );
        assert_eq!(chess.get_square_from_str("e", "7").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);

        let start_sq = *chess.get_square_from_str("e", "4");
        let end_sq = *chess.get_square_from_str("d", "5");
        chess.make_move(start_sq, end_sq);
        assert_eq!(chess.get_square_from_str("d", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);
    }
}
