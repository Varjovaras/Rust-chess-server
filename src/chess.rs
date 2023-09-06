use crate::chessboard::Board;

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
    pub fn _board(&self) -> &Board {
        &self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_initialization_works() {
        let chess: Chess = Chess::new();
    }
}
