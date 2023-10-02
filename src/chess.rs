use crate::{
    chessboard::{self, file::File, rank::Rank, square::Square, ChessBoard},
    piece::{PieceColor, Pieces},
};

type Move = (Square, Square, PieceColor);

#[derive(Debug)]
pub struct Chess {
    pub board: ChessBoard,
    pub turn_number: i32,
    pub latest_move: Option<Move>,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: chessboard::new(),
            turn_number: 0,
            latest_move: None,
        }
    }

    pub fn make_move(&mut self, start_sq: &mut Square, end_sq: &mut Square) {
        //only en passant affects chess_board removing the en passanted piece
        //rest of the effects on board happen in todo!()
        //maybe refactor pawn into its own move_piece function?

        //cannot capture own piece
        if end_sq.has_piece() && end_sq.piece.color() == start_sq.piece.color() {
            return;
        }

        if start_sq.piece.color() == &PieceColor::White && self.turn_number % 2 != 0
            || start_sq.piece.color() == &PieceColor::Black && self.turn_number % 2 != 1
        {
            return;
        }

        if !start_sq.piece.move_is_legal(start_sq, end_sq, self) {
            return;
        };

        self.update_board(*start_sq, *end_sq);
        self.turn_number += 1;
        self.latest_move = Some((*start_sq, *end_sq, *start_sq.piece.color()));
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
        &self.board[file][rank]
    }

    fn update_board(&mut self, mut start_sq: Square, mut end_sq: Square) {
        end_sq.piece = start_sq.piece.clone();
        start_sq.piece = Pieces::NoPiece();
        self.board[end_sq.file as usize][end_sq.rank as usize] = end_sq;
        self.board[start_sq.file as usize][start_sq.rank as usize] = start_sq;
    }

    pub fn _print_board_white(&self) {
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

        assert_eq!(chess.get_square_from_str("e", "2").piece, Pieces::NoPiece());
        assert_eq!(
            chess.get_square_from_str("e", "4").piece,
            Pieces::Pawn(PieceColor::White)
        );

        let mut start_sq = *chess.get_square_from_str("e", "4");
        let mut end_sq = *chess.get_square_from_str("e", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(
            chess.get_square_from_str("e", "4").piece,
            Pieces::Pawn(PieceColor::White)
        );
        assert_eq!(chess.get_square_from_str("e", "5").piece, Pieces::NoPiece());

        let mut start_sq = *chess.get_square_from_str("e", "7");
        let mut end_sq = *chess.get_square_from_str("e", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(
            chess.get_square_from_str("e", "5").piece,
            Pieces::Pawn(PieceColor::Black)
        );
        assert_eq!(chess.get_square_from_str("e", "7").piece, Pieces::NoPiece());

        let mut start_sq = *chess.get_square_from_str("e", "4");
        let mut end_sq = *chess.get_square_from_str("d", "5");
        chess.make_move(&mut start_sq, &mut end_sq);
        assert_eq!(chess.get_square_from_str("d", "5").piece, Pieces::NoPiece());
    }
}
