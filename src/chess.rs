use crate::square::Square;

#[derive(Debug)]
pub struct Chess {
    pub board: Vec<Vec<Square>>,
}

impl Chess {
    pub fn new() -> Chess {
        let mut board: Vec<Vec<Square>> = Vec::new();

        for i in 0..8 {
            let mut row: Vec<Square> = Vec::new();
            for j in 0..8 {
                let sq = Square::new(i, j);
                row.push(sq);
            }
            board.push(row);
        }

        if board.len() != 8 {
            panic!("Invalid board");
        }
        for board_row in board.iter() {
            if board_row.len() != 8 {
                panic!("Invalid board");
            }
        }

        Chess { board }
    }

    pub fn print_squares(&self) {
        self.board.iter().for_each(|row| {
            row.iter().for_each(|square| {
                print!("{} ", square.get_name());
            });
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chess_board_is_proper() {
        let chess_board: Chess = Chess::new();
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
