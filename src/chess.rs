use crate::board::{self, square::Square, ChessBoard};

#[derive(Debug)]
pub struct Chess {
    pub board: ChessBoard,
    pub turn_number: i32,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: board::new(),
            turn_number: 0,
        }
    }

    pub fn make_move(&self, _start_sq: &Square, _end_sq: &Square) {
        let move_legal = _start_sq.piece._move(_start_sq, _end_sq, self);

        if !move_legal {
            ()
        };
    }

    pub fn starting_position(&mut self) {
        self.board = board::starting_position(&mut self.board);
        self.turn_number = 0;
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
