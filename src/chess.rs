use core::panic;

use crate::{
    castling::Castling,
    check::king_is_in_check,
    checkmate::{self},
    chessboard::{
        self, file::File, get_black_king, get_squares_with_black_pieces,
        get_squares_with_white_pieces, get_white_king, rank::Rank, square::Square,
        starting_position, ChessBoard,
    },
    moves::{
        king::move_is_castling,
        move_helpers::helpers::{move_is_black_en_passant, move_is_white_en_passant},
        pawn::promote,
    },
    piece::{Piece, PieceColor},
};

type Move = (Square, Square, PieceColor);

#[derive(Debug, Clone)]
pub struct Chess {
    pub board: ChessBoard,
    pub turn_number: i32,
    pub latest_move: Option<Move>,
    pub castling: Castling,
    pub white_in_check: bool,
    pub black_in_check: bool,
    pub white_won: bool,
    pub black_won: bool,
    pub tie: bool,
    pub fifty_move_rule: u8,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: chessboard::new_board(),
            turn_number: 0,
            latest_move: None,
            castling: Castling::new(),
            white_in_check: false,
            black_in_check: false,
            white_won: false,
            black_won: false,
            tie: false,
            fifty_move_rule: 0,
        }
    }

    pub fn make_move(&mut self, start_sq: &mut Square, end_sq: &mut Square) {
        if self.white_won || self.black_won {
            return;
        }

        // if self.fifty_move_rule == 50 {
        //     self.tie = true;
        //     println!("tie by fifty move rule");
        //     return;
        // }

        if self.insufficient_material() {
            self.tie = true;
            self.print_board_white();
            println!("tie by insufficient material");
            panic!("tie by insufficient material");
        }

        let moving_piece_color = start_sq.piece.color();

        //wrong players turn
        if start_sq.piece.color() == &PieceColor::White && self.turn_number % 2 == 1
            || start_sq.piece.color() == &PieceColor::Black && self.turn_number % 2 == 0
        {
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

        //piece can make the move
        if !start_sq.piece.piece_move(start_sq, end_sq, self) {
            return;
        };

        //king is in check and the move doesnt remove check return
        if (moving_piece_color == &PieceColor::White && self.white_in_check
            || moving_piece_color == &PieceColor::Black && self.black_in_check)
            && !self.move_removes_check(start_sq, end_sq)
        {
            return;
        }

        if start_sq.piece == Piece::Pawn(PieceColor::White) && end_sq.rank == Rank::Eighth
            || start_sq.piece == Piece::Pawn(PieceColor::Black) && end_sq.rank == Rank::First
        {
            match promote(start_sq, end_sq, self) {
                Some(Piece::Pawn(_)) => return,
                Some(Piece::King(_)) => return,
                Some(Piece::None) => return,
                Some(promoted_piece) => {
                    if end_sq.has_piece() {
                        self.fifty_move_rule = 0;
                    } else {
                        self.fifty_move_rule += 1;
                    }
                    self.board[end_sq.file as usize][end_sq.rank as usize].piece = promoted_piece;
                    self.handle_check_after_move(start_sq);
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

        //remove castling if king or rook moves
        if start_sq.piece == Piece::King(PieceColor::White)
            || start_sq.piece == Piece::King(PieceColor::Black)
            || start_sq.piece == Piece::Rook(PieceColor::White)
            || start_sq.piece == Piece::Rook(PieceColor::Black)
        {
            if move_is_castling(start_sq, end_sq, self) {
                self.handle_castling(start_sq, end_sq);
                self.handle_check_after_move(start_sq);
                self.fifty_move_rule += 1;
                return;
            }
            self.remove_castling(start_sq);
        }

        self.update_board(start_sq, end_sq);
    }

    fn update_board(&mut self, start_sq: &Square, end_sq: &Square) {
        if end_sq.has_piece() {
            self.fifty_move_rule = 0;
        } else {
            self.fifty_move_rule += 1;
        }
        self.board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
        self.latest_move = Some((*start_sq, *end_sq, *start_sq.piece.color()));
        self.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
        self.turn_number += 1;
        self.handle_check_after_move(start_sq);
    }

    fn handle_check_after_move(&mut self, _start_sq: &Square) {
        self.white_in_check = king_is_in_check(&self.board, PieceColor::White);
        self.black_in_check = king_is_in_check(&self.board, PieceColor::Black);

        if self.white_in_check {
            self.black_won = checkmate::position_is_checkmate(self);
        } else if self.black_in_check {
            self.white_won = checkmate::position_is_checkmate(self);
        }
    }

    pub fn move_removes_check(&mut self, start_sq: &Square, end_sq: &Square) -> bool {
        let mut temp_board = self.board;
        if end_sq.has_piece() && end_sq.piece.color() == start_sq.piece.color() {
            return false;
        }

        if !start_sq.piece.piece_move(start_sq, end_sq, self) {
            return false;
        };

        if move_is_white_en_passant(start_sq, end_sq, self)
            || move_is_black_en_passant(start_sq, end_sq, self)
        {
            temp_board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None
        }

        temp_board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
        temp_board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
        !king_is_in_check(&temp_board, *start_sq.piece.color())
    }

    pub fn _make_move_from_str(&mut self, start_sq: &str, end_sq: &str) {
        let start_sq_chars: Vec<char> = start_sq.chars().collect();
        let end_sq_chars: Vec<char> = end_sq.chars().collect();
        let mut start_sq = *self._get_square_from_str(
            start_sq_chars[0].to_string().as_str(),
            start_sq_chars[1].to_string().as_str(),
        );

        let mut end_sq = *self._get_square_from_str(
            end_sq_chars[0].to_string().as_str(),
            end_sq_chars[1].to_string().as_str(),
        );

        self.make_move(&mut start_sq, &mut end_sq)
    }

    pub fn starting_position(&mut self) {
        self.board = chessboard::new_board();
        self.board = starting_position();
        self.turn_number = 0;
        self.white_won = false;
        self.black_won = false;
        self.white_in_check = false;
        self.black_in_check = false;
    }

    pub fn get_square(&self, file: File, rank: Rank) -> &Square {
        &self.board[file as usize][rank as usize]
    }

    pub fn _get_square_from_str(&mut self, file_str: &str, rank_str: &str) -> &Square {
        let file = File::_from_str_slice(file_str)._as_usize();
        let rank = Rank::_from_str(rank_str)._as_usize();
        if file > 7 || rank > 7 {
            panic!("get_square_from_str failed for inputting too big file or rank")
        }
        &self.board[file][rank]
    }

    pub fn print_board_white(&self) {
        let mut clone_board = self.board;
        clone_board.reverse();

        for i in (0..8).rev() {
            for j in (0..8).rev() {
                print!("{} ", clone_board[j][i].piece_name());
            }
            println!(" ");
        }
    }

    pub fn _print_board_black(&self) {
        let mut clone_board = self.board;
        for square_vec in &mut clone_board {
            square_vec.reverse();
        }
        clone_board.iter().for_each(|row| {
            row.iter().for_each(|square| {
                print!("{} ", square._square_name());
            });
            println!();
        });
    }

    fn handle_castling(&mut self, start_sq: &Square, end_sq: &Square) {
        match (start_sq.rank, end_sq.file) {
            (Rank::First, File::G) => {
                self.board[File::H as usize][Rank::First as usize].piece = Piece::None;
                self.board[File::F as usize][Rank::First as usize].piece =
                    Piece::Rook(PieceColor::White);
                self.board[File::G as usize][Rank::First as usize].piece =
                    Piece::King(PieceColor::White);
                self.castling.white_king_side_castling = false;
                self.castling.white_queen_side_castling = false;
            }
            (Rank::First, File::C) => {
                self.board[File::A as usize][Rank::First as usize].piece = Piece::None;
                self.board[File::D as usize][Rank::First as usize].piece =
                    Piece::Rook(PieceColor::White);
                self.board[File::C as usize][Rank::First as usize].piece =
                    Piece::King(PieceColor::White);
                self.castling.white_king_side_castling = false;
                self.castling.white_queen_side_castling = false;
            }
            (Rank::Eighth, File::G) => {
                self.board[File::H as usize][Rank::Eighth as usize].piece = Piece::None;
                self.board[File::F as usize][Rank::Eighth as usize].piece =
                    Piece::Rook(PieceColor::Black);
                self.board[File::G as usize][Rank::Eighth as usize].piece =
                    Piece::King(PieceColor::Black);
                self.castling.black_king_side_castling = false;
                self.castling.black_queen_side_castling = false;
            }
            (Rank::Eighth, File::C) => {
                self.board[File::A as usize][Rank::Eighth as usize].piece = Piece::None;
                self.board[File::D as usize][Rank::Eighth as usize].piece =
                    Piece::Rook(PieceColor::Black);
                self.board[File::C as usize][Rank::Eighth as usize].piece =
                    Piece::King(PieceColor::Black);
                self.castling.black_king_side_castling = false;
                self.castling.black_queen_side_castling = false;
            }
            _ => panic!("Trying to castle with wrong start and end square"),
        }
    }

    fn remove_castling(&mut self, start_sq: &Square) {
        match start_sq.piece {
            Piece::King(PieceColor::White) => {
                self.castling.white_king_side_castling = false;
                self.castling.white_queen_side_castling = false;
            }
            Piece::King(PieceColor::Black) => {
                self.castling.black_king_side_castling = false;
                self.castling.black_queen_side_castling = false;
            }
            Piece::Rook(PieceColor::White) => {
                if start_sq.file == File::A && start_sq.rank == Rank::First {
                    self.castling.white_queen_side_castling = false;
                } else if start_sq.file == File::H && start_sq.rank == Rank::First {
                    self.castling.white_king_side_castling = false;
                }
            }
            Piece::Rook(PieceColor::Black) => {
                if start_sq.file == File::A && start_sq.rank == Rank::Eighth {
                    self.castling.black_queen_side_castling = false;
                } else if start_sq.file == File::H && start_sq.rank == Rank::Eighth {
                    self.castling.black_king_side_castling = false;
                }
            }
            _ => (),
        }
    }

    pub fn insufficient_material(&self) -> bool {
        let mut white_knights: u8 = 0;
        let mut white_bishops: u8 = 0;
        let mut black_knights: u8 = 0;
        let mut black_bishops: u8 = 0;

        let white_piece_squares = get_squares_with_white_pieces(&self.board);
        let black_piece_squares = get_squares_with_black_pieces(&self.board);

        if get_white_king(&self.board).is_none() || get_black_king(&self.board).is_none() {
            return true;
        }

        if white_piece_squares.len() == 1 && black_piece_squares.len() == 1 {
            return true;
        }

        for square in white_piece_squares {
            match square.piece {
                Piece::None => panic!("White piece square has no piece"),
                Piece::Pawn(_) => return false,
                Piece::Knight(_) => white_knights += 1,
                Piece::Bishop(_) => white_bishops += 1,
                Piece::Rook(_) => return false,
                Piece::Queen(_) => return false,
                Piece::King(_) => {}
            }
        }

        for square in black_piece_squares {
            match square.piece {
                Piece::None => panic!("Black piece square has no piece"),
                Piece::Pawn(_) => return false,
                Piece::Knight(_) => black_knights += 1,
                Piece::Bishop(_) => black_bishops += 1,
                Piece::Rook(_) => return false,
                Piece::Queen(_) => return false,
                Piece::King(_) => {}
            }
        }

        if white_knights > 1 || black_knights > 1 || white_bishops > 1 || black_bishops > 1 {
            return false;
        }

        if white_knights == 0 && black_knights == 1 && white_bishops == 1 && black_bishops == 0 {
            return true;
        }

        if white_knights == 1 && black_knights == 0 && white_bishops == 0 && black_bishops == 1 {
            return true;
        }

        if white_knights == 1 && black_knights == 0 && white_bishops == 0 && black_bishops == 0 {
            return true;
        }

        if white_knights == 0 && black_knights == 1 && white_bishops == 0 && black_bishops == 0 {
            return true;
        }

        if white_knights == 0 && black_knights == 0 && white_bishops == 1 && black_bishops == 0 {
            return true;
        }
        if white_knights == 0 && black_knights == 0 && white_bishops == 0 && black_bishops == 1 {
            return true;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_initialization_works() {
        let _chess: Chess = Chess::new();
    }

    #[test]
    fn make_move_works() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();
        let mut start_sq = *chess._get_square_from_str("e", "2");
        let mut end_sq = *chess._get_square_from_str("e", "4");
        chess.make_move(&mut start_sq, &mut end_sq);

        assert_eq!(chess._get_square_from_str("e", "2").piece, Piece::None);
        assert_eq!(
            chess._get_square_from_str("e", "4").piece,
            Piece::Pawn(PieceColor::White)
        );
        assert_eq!(chess.turn_number, 1);
        assert_eq!(
            chess.latest_move,
            Some((start_sq, end_sq, PieceColor::White))
        );

        let mut start_sq = *chess._get_square_from_str("e", "4");
        let mut end_sq = *chess._get_square_from_str("e", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(
            chess._get_square_from_str("e", "4").piece,
            Piece::Pawn(PieceColor::White)
        );
        assert_eq!(chess._get_square_from_str("e", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 1);

        let mut start_sq = *chess._get_square_from_str("e", "7");
        let mut end_sq = *chess._get_square_from_str("e", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(
            chess._get_square_from_str("e", "5").piece,
            Piece::Pawn(PieceColor::Black)
        );
        assert_eq!(chess._get_square_from_str("e", "7").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);

        let mut start_sq = *chess._get_square_from_str("e", "4");
        let mut end_sq = *chess._get_square_from_str("d", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(chess._get_square_from_str("d", "5").piece, Piece::None);
        assert_eq!(chess.turn_number, 2);
    }
}
