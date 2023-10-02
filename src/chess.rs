use crate::{
    chessboard::{self, file::File, rank::Rank, square::Square, ChessBoard},
    piece::{PieceColor, Pieces},
};

#[derive(Debug)]
pub struct Chess {
    pub board: ChessBoard,
    pub turn_number: i32,
    pub latest_move: Option<(Square, Square, PieceColor)>,
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

        if !start_sq.piece.move_piece(start_sq, end_sq, self) {
            return;
        };

        self.update_board(*start_sq, *end_sq);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_initialization_works() {
        let _chess: Chess = Chess::new();
    }
}
