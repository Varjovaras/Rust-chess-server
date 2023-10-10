use crate::{
    castling::Castling,
    check::king_is_in_check,
    chessboard::{self, file::File, rank::Rank, square::Square, ChessBoard},
    moves::{
        king::move_is_castling,
        move_helpers::helpers::{move_is_black_en_passant, move_is_white_en_passant},
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
    pub game_over: bool,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: chessboard::new(),
            turn_number: 0,
            latest_move: None,
            castling: Castling::new(),
            white_in_check: false,
            black_in_check: false,
            game_over: false,
        }
    }

    pub fn make_move(&mut self, start_sq: &mut Square, end_sq: &mut Square) {
        let moving_piece_color = start_sq.piece.color();

        //piece can make the move
        if !start_sq.piece.piece_move(start_sq, end_sq, self) {
            return;
        };

        //cannot capture own piece
        if end_sq.has_piece() && end_sq.piece.color() == moving_piece_color {
            return;
        }

        //king is in check and the move doesnt remove check
        if (moving_piece_color == &PieceColor::White && self.white_in_check
            || moving_piece_color == &PieceColor::Black && self.black_in_check)
            && !self.move_removes_check(start_sq, end_sq)
        {
            return;
        }

        //wrong players turn
        if start_sq.piece.color() == &PieceColor::White && self.turn_number % 2 == 1
            || start_sq.piece.color() == &PieceColor::Black && self.turn_number % 2 == 0
        {
            return;
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
                return;
            }

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

        self.update_board(start_sq, end_sq);
    }

    fn update_board(&mut self, start_sq: &Square, end_sq: &Square) {
        self.board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
        self.latest_move = Some((*start_sq, *end_sq, *start_sq.piece.color()));
        self.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
        self.turn_number += 1;

        if start_sq.piece.color() == &PieceColor::White {
            self.black_in_check = king_is_in_check(&self.board, PieceColor::Black);
        } else {
            self.white_in_check = king_is_in_check(&self.board, PieceColor::White);
        }
    }

    pub fn move_removes_check(&self, start_sq: &Square, end_sq: &Square) -> bool {
        let mut temp_board = self.board;

        if move_is_white_en_passant(start_sq, end_sq, self)
            || move_is_black_en_passant(start_sq, end_sq, self)
        {
            temp_board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None
        }

        temp_board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
        temp_board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;

        !king_is_in_check(&temp_board, *start_sq.piece.color())
    }

    pub fn make_move_from_str(&mut self, start_sq: &str, end_sq: &str) {
        let start_sq_chars: Vec<char> = start_sq.chars().collect();
        let end_sq_chars: Vec<char> = end_sq.chars().collect();
        let mut start_sq = *self.get_square_from_str(
            start_sq_chars[0].to_string().as_str(),
            start_sq_chars[1].to_string().as_str(),
        );

        let mut end_sq = *self.get_square_from_str(
            end_sq_chars[0].to_string().as_str(),
            end_sq_chars[1].to_string().as_str(),
        );

        self.make_move(&mut start_sq, &mut end_sq)
    }

    pub fn starting_position(&mut self) {
        self.board = chessboard::starting_position(&mut self.board);
        self.turn_number = 0;
    }

    pub fn get_square(&self, file: File, rank: Rank) -> &Square {
        &self.board[file as usize][rank as usize]
    }

    pub fn get_square_from_str(&mut self, file_str: &str, rank_str: &str) -> &Square {
        let file = File::from_str_slice(file_str).as_usize();
        let rank = Rank::from_str(rank_str).as_usize();
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
                print!("{:?} ", clone_board[j][i].piece);
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
