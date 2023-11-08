use crate::{
    castling::Castling,
    check::king_is_in_check,
    checkmate::{self},
    chessboard::{self, file::File, rank::Rank, square::Square, starting_position, ChessBoard},
    game_state::{insufficient_material, GameState},
    moves::{
        king::move_is_castling,
        move_helpers::helpers::{move_is_black_en_passant, move_is_white_en_passant},
        pawn::promote,
    },
    piece::{Piece, PieceColor},
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
    pub fn _new() -> Chess {
        Chess {
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

    pub fn new_starting_position() -> Chess {
        let mut chess = Chess {
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

    //refactor this mess, works as of now
    pub fn make_move(&mut self, start_sq: &mut Square, end_sq: &mut Square) {
        let moving_piece_color = start_sq.piece.color();

        if !self.move_is_allowed(moving_piece_color) {
            return;
        }

        if end_sq.piece == Piece::King(PieceColor::White)
            || end_sq.piece == Piece::King(PieceColor::Black)
        {
            return;
        }

        //cannot capture own piece
        if end_sq.has_piece() && end_sq.piece.color() == moving_piece_color {
            return;
        }

        //check if piece can make the move
        if !start_sq.piece.piece_move(start_sq, end_sq, self) {
            return;
        };

        //king is in check and the move doesnt remove check return
        if (moving_piece_color == &PieceColor::White && self.white_player.in_check()
            || moving_piece_color == &PieceColor::Black && self.black_player.in_check())
            && !self.king_is_not_in_check_after_move(start_sq, end_sq)
        {
            return;
        }

        //check if move puts you into check
        if !self.king_is_not_in_check_after_move(start_sq, end_sq) {
            return;
        }

        if start_sq.piece == Piece::Pawn(PieceColor::White)
            || start_sq.piece == Piece::Pawn(PieceColor::Black)
        {
            self.fifty_move_rule = 0;
        }

        if start_sq.piece == Piece::Pawn(PieceColor::White) && end_sq.rank == Rank::Eighth
            || start_sq.piece == Piece::Pawn(PieceColor::Black) && end_sq.rank == Rank::First
        {
            match promote(start_sq, end_sq, self) {
                Some(Piece::Pawn(_)) => return,
                Some(Piece::King(_)) => return,
                Some(Piece::None) => return,
                Some(promoted_piece) => {
                    self.board[end_sq.file as usize][end_sq.rank as usize].piece = promoted_piece;
                    self.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
                    self.handle_check_after_move(start_sq);
                    self.latest_move = Some((*start_sq, *end_sq, *start_sq.piece.color()));
                    self.turn_number += 1;
                    self.list_of_moves.push((
                        (start_sq.file, start_sq.rank.as_usize()),
                        (end_sq.file, end_sq.rank.as_usize()),
                    ));
                    if end_sq.has_piece()
                        || start_sq.piece == Piece::Pawn(PieceColor::White)
                        || start_sq.piece == Piece::Pawn(PieceColor::Black)
                    {
                        self.fifty_move_rule = 0;
                    } else {
                        self.fifty_move_rule += 1;
                    }
                    if self.fifty_move_rule >= 50 {
                        self.gamestate = GameState::Stalemate;
                    }
                    return;
                }
                None => return,
            }
        }

        //remove en-passanted piece
        if move_is_white_en_passant(start_sq, end_sq, self)
            || move_is_black_en_passant(start_sq, end_sq, self)
        {
            self.board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
        }

        if start_sq.piece == Piece::King(PieceColor::White)
            || start_sq.piece == Piece::King(PieceColor::Black)
            || start_sq.piece == Piece::Rook(PieceColor::White)
            || start_sq.piece == Piece::Rook(PieceColor::Black)
        {
            self.handle_rook_and_king_move(start_sq, end_sq);
        }

        self.update_board(start_sq, end_sq);
    }

    fn handle_rook_and_king_move(&mut self, start_sq: &Square, end_sq: &Square) {
        //remove castling if king or rook moves
        if move_is_castling(start_sq, end_sq, self) {
            self.handle_castling(start_sq, end_sq);
            self.handle_check_after_move(start_sq);
            return;
        }
        self.remove_castling(start_sq);
    }

    fn move_is_allowed(&mut self, start_sq_piece_color: &PieceColor) -> bool {
        if self.gamestate != GameState::InProgress {
            return false;
        }

        if self.white_player.victory || self.black_player.victory {
            return false;
        }

        if self.fifty_move_rule >= 50 {
            self.gamestate = GameState::Stalemate;
            return false;
        }

        if insufficient_material(self) {
            self.gamestate = GameState::Stalemate;
            return false;
        }

        //wrong players turn
        if start_sq_piece_color == &PieceColor::White && self.turn_number % 2 == 1
            || start_sq_piece_color == &PieceColor::Black && self.turn_number % 2 == 0
        {
            return false;
        }

        true
    }

    fn update_board(&mut self, start_sq: &Square, end_sq: &Square) {
        if end_sq.has_piece()
            || start_sq.piece == Piece::Pawn(PieceColor::White)
            || start_sq.piece == Piece::Pawn(PieceColor::Black)
        {
            self.fifty_move_rule = 0;
        } else {
            self.fifty_move_rule += 1;
        }

        if self.fifty_move_rule >= 50 {
            self.gamestate = GameState::Stalemate;
        }

        //move start_sq piece to end_sq
        self.board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
        self.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;

        self.latest_move = Some((*start_sq, *end_sq, *start_sq.piece.color()));
        self.turn_number += 1;
        self.handle_check_after_move(start_sq);
        self.list_of_moves.push((
            (start_sq.file, start_sq.rank.as_usize()),
            (end_sq.file, end_sq.rank.as_usize()),
        ));
    }

    fn handle_check_after_move(&mut self, _start_sq: &Square) {
        self.white_player.in_check = king_is_in_check(&self.board, PieceColor::White, self);
        self.black_player.in_check = king_is_in_check(&self.board, PieceColor::Black, self);

        if self.white_player.in_check {
            self.black_player.victory = checkmate::position_is_checkmate(self);
        } else if self.black_player.in_check {
            self.white_player.victory = checkmate::position_is_checkmate(self);
        }

        if self.white_player.victory {
            self.gamestate = GameState::WhiteVictory;
        } else if self.black_player.victory {
            self.gamestate = GameState::BlackVictory;
        }
    }

    pub fn king_is_not_in_check_after_move(&mut self, start_sq: &Square, end_sq: &Square) -> bool {
        if end_sq.has_piece() && end_sq.piece.color() == start_sq.piece.color() {
            return false;
        }

        if !start_sq.piece.piece_move(start_sq, end_sq, self) {
            return false;
        };

        let mut temp_board = self.board;
        if move_is_white_en_passant(start_sq, end_sq, self)
            || move_is_black_en_passant(start_sq, end_sq, self)
        {
            temp_board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None
        } else {
            temp_board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
            temp_board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
        }

        !king_is_in_check(&temp_board, *start_sq.piece.color(), self)
    }

    pub fn make_move_from_str(&mut self, start_sq: &str, end_sq: &str) {
        let mut start_sq = *self.get_square_from_str(&start_sq[0..1], &start_sq[1..2]);
        let mut end_sq = *self.get_square_from_str(&end_sq[0..1], &end_sq[1..2]);
        self.make_move(&mut start_sq, &mut end_sq)
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

    pub fn get_square(&self, file: File, rank: Rank) -> &Square {
        &self.board[file as usize][rank as usize]
    }

    pub fn get_square_from_str(&mut self, file_str: &str, rank_str: &str) -> &Square {
        let file = File::_from_str_slice(file_str).as_usize();
        let rank = Rank::_from_str(rank_str).as_usize();
        if file > 7 || rank > 7 {
            panic!("get_square_from_str failed for inputting too big file or rank")
        }
        &self.board[file][rank]
    }

    pub fn print_board_white(&self) {
        for i in (0..8).rev() {
            for j in (0..8).rev() {
                print!("{} ", self.board[j][i].piece_name());
            }
            println!();
        }
    }

    pub fn _print_board_black(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{} ", self.board[j][i].piece_name());
            }
            println!();
        }
    }

    fn handle_castling(&mut self, start_sq: &Square, end_sq: &Square) {
        let (rook_file, rook_piece) = match (start_sq.rank, end_sq.file) {
            (Rank::First, File::G) => (File::F, Piece::Rook(PieceColor::White)),
            (Rank::First, File::C) => (File::D, Piece::Rook(PieceColor::White)),
            (Rank::Eighth, File::G) => (File::F, Piece::Rook(PieceColor::Black)),
            (Rank::Eighth, File::C) => (File::D, Piece::Rook(PieceColor::Black)),
            _ => panic!("Trying to castle with wrong start and end square"),
        };

        self.move_rook_for_castling(start_sq, rook_file, rook_piece);
        self.update_castling_rights(start_sq);
    }

    fn move_rook_for_castling(&mut self, start_sq: &Square, rook_file: File, rook_piece: Piece) {
        self.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
        self.board[rook_file as usize][start_sq.rank as usize].piece = rook_piece;
    }

    fn update_castling_rights(&mut self, start_sq: &Square) {
        match start_sq.rank {
            Rank::First => {
                self.castling.white_king_side_castling = false;
                self.castling.white_queen_side_castling = false;
            }
            Rank::Eighth => {
                self.castling.black_king_side_castling = false;
                self.castling.black_queen_side_castling = false;
            }
            _ => {}
        }
    }

    fn remove_castling(&mut self, start_sq: &Square) {
        match (start_sq.piece, start_sq.file, start_sq.rank) {
            (Piece::King(PieceColor::White), _, _) => {
                self.castling.white_king_side_castling = false;
                self.castling.white_queen_side_castling = false;
            }
            (Piece::King(PieceColor::Black), _, _) => {
                self.castling.black_king_side_castling = false;
                self.castling.black_queen_side_castling = false;
            }
            (Piece::Rook(PieceColor::White), File::A, Rank::First) => {
                self.castling.white_queen_side_castling = false;
            }
            (Piece::Rook(PieceColor::White), File::H, Rank::First) => {
                self.castling.white_king_side_castling = false;
            }
            (Piece::Rook(PieceColor::Black), File::A, Rank::Eighth) => {
                self.castling.black_queen_side_castling = false;
            }
            (Piece::Rook(PieceColor::Black), File::H, Rank::Eighth) => {
                self.castling.black_king_side_castling = false;
            }
            _ => (),
        }
    }

    pub fn print_moves(self) {
        self.list_of_moves.into_iter().for_each(|m| {
            println!("{:?}", m);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_initialization_works() {
        let _chess: Chess = Chess::_new();
    }

    #[test]
    fn make_move_works() {
        let mut chess: Chess = Chess::_new();
        chess.starting_position();
        let mut start_sq = *chess.get_square_from_str("e", "2");
        let mut end_sq = *chess.get_square_from_str("e", "4");
        chess.make_move(&mut start_sq, &mut end_sq);

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

        let mut start_sq = *chess.get_square_from_str("e", "4");
        let mut end_sq = *chess.get_square_from_str("e", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(
            chess.get_square_from_str("e", "4").piece,
            Piece::Pawn(PieceColor::White)
        );
        assert_eq!(chess.get_square_from_str("e", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 1);

        let mut start_sq = *chess.get_square_from_str("e", "7");
        let mut end_sq = *chess.get_square_from_str("e", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(
            chess.get_square_from_str("e", "5").piece,
            Piece::Pawn(PieceColor::Black)
        );
        assert_eq!(chess.get_square_from_str("e", "7").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);

        let mut start_sq = *chess.get_square_from_str("e", "4");
        let mut end_sq = *chess.get_square_from_str("d", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(chess.get_square_from_str("d", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);
    }
}
