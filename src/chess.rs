use crate::chessboard::{square::Square, Board};

#[derive(Debug)]
pub struct Chess {
    pub board: Board,
    pub turn_number: i32,
}

pub type ChessBoard = [[Square; 8]; 8];

/**
*
*
*tee chessboardista chess structin osa pasalusta


*/
impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: Board::new(),
            turn_number: 0,
        }
    }

    pub fn make_move(&self, _start_sq: &Square, _end_sq: &Square) {
        let move_legal = _start_sq
            .piece
            ._move(_start_sq, _end_sq, &self.board.get_board());

        if !move_legal {
            return;
        };
    }

    pub fn starting_position(&mut self) {
        self.board.starting_position();
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
