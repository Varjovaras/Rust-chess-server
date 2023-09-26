use crate::{
    chessboard::{self, file::File, rank::Rank, square::Square, ChessBoard},
    piece::PieceColor,
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

    pub fn make_move(&mut self, start_sq: &Square, end_sq: &Square) {
        //only en passant affects chess_board removing the en passanted piece
        //rest of the effects on board happen in todo!()
        //maybe refactor pawn into its own move_piece function?

        //cannot capture own piece
        if end_sq.has_piece() && end_sq.piece.color() == start_sq.piece.color() {
            return;
        }

        let move_legal = start_sq.piece.move_piece(start_sq, end_sq, self);
        if !move_legal {
            return;
        };
        //
        //
        //make board changes here
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_initialization_works() {
        let _chess: Chess = Chess::new();
    }
}
