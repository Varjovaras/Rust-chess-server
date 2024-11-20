use std::str::FromStr;

use crate::{
    chessboard::{
        self, add_possible_moves_to_squares, file::File, rank::Rank, square::Square,
        starting_position, ChessBoard,
    },
    game_state::GameState,
    make_chess_move::make_chess_move,
    piece::{tuple_to_promoted_piece, Piece, PieceColor},
    pieces_eaten::PiecesEaten,
    player::Player,
};

use serde::{Deserialize, Serialize};

pub type LatestMove = (Square, Square, PieceColor);
type SquareCoordinates = (File, Rank);

/**
* (0, x) is always None,
* (1, x) is always Queen
* (2, x) is always Rook
* (3, x) is always Knight
* (4, x) is always Bishop
* (x, 0) is White
* (x, 1) is Black
*/
pub type PromotedPiece = (i32, i32);

pub type Move = (SquareCoordinates, SquareCoordinates, PromotedPiece);
pub type ListOfMoves = Vec<Move>;

/**
 * first player is white, second is black
 */
pub type Players = (Player, Player);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chess {
    pub board: ChessBoard,
    pub turn_number: i32,
    pub latest_move: Option<LatestMove>,
    // pub castling: Castling,
    pub players: (Player, Player),
    pub gamestate: GameState,
    pub fifty_move_rule: u8,
    pub list_of_moves: ListOfMoves,
    pub pieces_eaten: PiecesEaten,
}

impl Chess {
    #[must_use]
    fn new() -> Self {
        Self {
            board: chessboard::new_board(),
            turn_number: 0,
            latest_move: None,
            players: (
                Player::new(PieceColor::White),
                Player::new(PieceColor::Black),
            ),
            gamestate: GameState::InProgress,
            fifty_move_rule: 0,
            list_of_moves: Vec::new(),
            pieces_eaten: PiecesEaten::new(),
        }
    }

    #[must_use]
    pub fn new_starting_position() -> Self {
        let mut chess = Self {
            board: chessboard::new_board(),
            turn_number: 0,
            latest_move: None,
            players: (
                Player::new(PieceColor::White),
                Player::new(PieceColor::Black),
            ),
            gamestate: GameState::InProgress,
            fifty_move_rule: 0,
            list_of_moves: Vec::new(),
            pieces_eaten: PiecesEaten::new(),
        };
        chess.starting_position();
        add_possible_moves_to_squares(&mut chess);
        chess
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to convert to JSON")
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn starting_position(&mut self) {
        self.board = chessboard::new_board();
        self.board = starting_position();
        self.turn_number = 0;
        self.players.0.victory = false;
        self.players.1.victory = false;
        self.players.0.in_check = false;
        self.players.1.in_check = false;
        self.gamestate = GameState::InProgress;
    }

    #[must_use]
    pub fn get_square(&self, file: File, rank: Rank) -> Square {
        self.board[file as usize][rank as usize].clone()
    }

    #[must_use]
    pub fn get_white_player(self) -> Player {
        self.players.0
    }

    #[must_use]
    pub fn get_black_player(self) -> Player {
        self.players.1
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_square_from_str(&self, file_str: &str, rank_str: &str) -> &Square {
        let file = File::from_str_slice(file_str).as_usize();
        let rank = Rank::from_str(rank_str)
            .expect("Invalid rank string provided")
            .as_usize();
        assert!(
            !(file > 7 || rank > 7),
            "get_square_from_str failed for inputting too big file or rank"
        );
        &self.board[file][rank]
    }

    pub fn print_white_board_to_terminal(&self) {
        let mut clone_board = self.board.clone();
        clone_board.reverse();

        for i in (0..8).rev() {
            for j in (0..8).rev() {
                print!("{} ", clone_board[j][i].piece_name());
            }
            println!(" ");
        }
    }

    pub fn print_black_board_to_terminal(&self) {
        let mut clone_board = self.board.clone();
        for square_vec in &mut clone_board {
            square_vec.reverse();
        }
        for row in &clone_board {
            for square in row {
                print!("{} ", square.square_name());
            }
            println!();
        }
    }

    pub fn make_move(&mut self, start_sq: &Square, end_sq: &Square, promoted_piece: PromotedPiece) {
        let promoted_piece = tuple_to_promoted_piece(promoted_piece);
        make_chess_move(self, start_sq, end_sq, promoted_piece);
    }

    pub fn make_move_from_str(
        &mut self,
        start_sq: &str,
        end_sq: &str,
        possible_promoted_piece: Option<&str>,
    ) {
        if start_sq.is_empty() || end_sq.is_empty() {
            println!("Startsq or endsq was empty");
            return;
        }

        let promoted_piece = possible_promoted_piece.and_then(|piece_str| {
            let piece = Piece::from(Some(piece_str));
            if piece == Piece::None {
                None
            } else {
                Some(piece)
            }
        });

        let start_sq_chars: Vec<char> = start_sq.chars().collect();
        let end_sq_chars: Vec<char> = end_sq.chars().collect();

        let start_sq = self
            .get_square_from_str(
                start_sq_chars[0].to_string().as_str(),
                start_sq_chars[1].to_string().as_str(),
            )
            .clone();

        let end_sq = self
            .get_square_from_str(
                end_sq_chars[0].to_string().as_str(),
                end_sq_chars[1].to_string().as_str(),
            )
            .clone();

        make_chess_move(self, &start_sq, &end_sq, promoted_piece);
    }

    pub fn print_moves(self) {
        self.list_of_moves.into_iter().for_each(|m| {
            println!("{m:?}");
        });
    }
}

impl Default for Chess {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::piece::Piece;

    use super::*;
    #[test]
    fn chess_initialization_works() {
        let _chess: Chess = Chess::new();
    }

    #[test]
    fn make_move_works() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();
        let start_sq = chess.get_square_from_str("e", "2").clone();
        let end_sq = chess.get_square_from_str("e", "4").clone();
        make_chess_move(&mut chess, &start_sq, &end_sq, None);

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

        let start_sq = chess.get_square_from_str("e", "4").clone();
        let end_sq = chess.get_square_from_str("e", "5").clone();
        make_chess_move(&mut chess, &start_sq, &end_sq, None);
        assert_eq!(
            chess.get_square_from_str("e", "4").piece,
            Piece::Pawn(PieceColor::White)
        );
        assert_eq!(chess.get_square_from_str("e", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 1);

        let start_sq = chess.get_square_from_str("e", "7").clone();
        let end_sq = chess.get_square_from_str("e", "5").clone();
        make_chess_move(&mut chess, &start_sq, &end_sq, None);

        assert_eq!(
            chess.get_square_from_str("e", "5").piece,
            Piece::Pawn(PieceColor::Black)
        );
        assert_eq!(chess.get_square_from_str("e", "7").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);

        let start_sq = chess.get_square_from_str("e", "4").clone();
        let end_sq = chess.get_square_from_str("d", "5").clone();
        make_chess_move(&mut chess, &start_sq, &end_sq, None);
        assert_eq!(chess.get_square_from_str("d", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);
    }
}
