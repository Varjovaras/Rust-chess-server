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
        let chess_board = chess.board;
        assert_eq!(chess_board.board[0][0].square_name(), String::from("A1"));
        assert_eq!(chess_board.board[0][7].square_name(), String::from("A8"));
        assert_eq!(chess_board.board[1][1].square_name(), String::from("B2"));
        assert_eq!(chess_board.board[2][2].square_name(), String::from("C3"));
        assert_eq!(chess_board.board[3][3].square_name(), String::from("D4"));
        assert_eq!(chess_board.board[4][4].square_name(), String::from("E5"));
        assert_eq!(chess_board.board[5][5].square_name(), String::from("F6"));
        assert_eq!(chess_board.board[6][6].square_name(), String::from("G7"));
        assert_eq!(chess_board.board[7][7].square_name(), String::from("H8"));
        assert_eq!(chess_board.board[4][2].square_name(), String::from("E3"));
        assert_eq!(chess_board.board[7][0].square_name(), String::from("H1"));
        assert_eq!(chess_board.board[7][1].square_name(), String::from("H2"));
        assert_eq!(chess_board.board[6][1].square_name(), String::from("G2"));
    }
}
