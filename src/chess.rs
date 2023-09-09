use crate::chessboard::{square::Square, Board};

#[derive(Debug)]
pub struct Chess {
    pub board: Board,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: Board::new(),
        }
    }

    pub fn _make_move(&self, _start_sq: &Square, _end_sq: &Square) {
        let _move_legal = _start_sq
            .piece
            ._move(_start_sq, _end_sq, &self.board.get_board());

        if !_move_legal {
            return;
        };
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
