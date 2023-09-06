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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn chess_board_is_proper() {
        let chess: Chess = Chess::new();
        let _chess_board = chess.board;
        // assert_eq!(chess_board.board[0][0].file_name, F);
        // assert_eq!(chess_board.board[0][7].rank_name, Rank::Eighth);
        // assert_eq!(chess_board.board[1][1].file_name, File::B);
        // assert_eq!(chess_board.board[2][2].rank_name, Rank::Third);
        // assert_eq!(chess_board.board[3][3].rank_name, Rank::Fourth);
        // assert_eq!(chess_board.board[4][4].name, "E5");
        // assert_eq!(chess_board.board[5][5].name, "F6");
        // assert_eq!(chess_board.board[6][6].name, "G7");
        // assert_eq!(chess_board.board[7][7].name, "H8");
        // assert_eq!(chess_board.board[4][2].name, "E3");
        // assert_eq!(chess_board.board[7][0].name, "H1");
    }
}
