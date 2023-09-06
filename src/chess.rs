use crate::board::Board;

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

impl Default for Chess {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_board_is_proper() {
        let chess: Chess = Chess::new();
        let chess_board = chess.board;
        assert_eq!(chess_board.board[0][0].name, "A1");
        assert_eq!(chess_board.board[0][7].name, "A8");
        assert_eq!(chess_board.board[1][1].name, "B2");
        assert_eq!(chess_board.board[2][2].name, "C3");
        assert_eq!(chess_board.board[3][3].name, "D4");
        assert_eq!(chess_board.board[4][4].name, "E5");
        assert_eq!(chess_board.board[5][5].name, "F6");
        assert_eq!(chess_board.board[6][6].name, "G7");
        assert_eq!(chess_board.board[7][7].name, "H8");
        assert_eq!(chess_board.board[4][2].name, "E3");
        assert_eq!(chess_board.board[7][0].name, "H1");
    }
}
