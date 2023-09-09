use crate::chessboard::{square::Square, Board};

#[derive(Debug)]
pub struct Chess {
    board: Board,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: Board::new(),
        }
    }
    pub fn _board(&self) -> &Board {
        &self.board
    }

    pub fn _make_move(&self, _start_sq: &Square, _end_sq: &Square) {
        // let _is_legal = self._is_move_legal(_start_sq, _end_sq);
    }

    // fn _is_move_legal(&self, _start_sq: Square, _end_sq: Square) -> bool {
    //     return _start_sq.piece.move_piece(_start_sq, self._board());
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_initialization_works() {
        let _chess: Chess = Chess::new();
    }
}
