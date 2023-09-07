use self::square::Square;

pub mod file;
pub mod rank;
pub mod square;

#[derive(Debug)]
/// board[i][j]
/// i stands for the rank and
/// j stands for the file.
/// for example board[0][1] is B1
/// board[1][0] is A2
/// and board[4][2] is E3
pub struct Board {
    board: [[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let mut board: [[Square; 8]; 8] = [[Square::default(); 8]; 8];

        for rank in 0..8 {
            let mut row: [Square; 8] = [Square::default(); 8];

            for file in 0..8 {
                let sq = Square::new(file, rank);
                row[file as usize] = sq;
            }
            board[rank as usize] = row;
        }
        Board { board }
    }

    pub fn get_board(&self) -> &[[Square; 8]; 8] {
        &self.board
    }

    // pub fn get_square(&self, file: u8, rank: u8) -> &Square {
    //     // let square = self.board[];
    //     square
    // }

    pub fn print_board_white(&self) {
        let mut clone_board = self.get_board().clone();
        clone_board.reverse();
        clone_board.iter().for_each(|row| {
            row.iter().for_each(|square| {
                print!("{} ", square.square_name());
            });
            println!();
        });
    }

    pub fn print_board_black(&self) {
        let mut clone_board = self.get_board().clone();
        for square_vec in &mut clone_board {
            square_vec.reverse();
        }
        clone_board.iter().for_each(|row| {
            row.iter().for_each(|square| {
                print!("{} ", square.square_name());
            });
            println!();
        });
    }
}

mod tests {
    #[test]
    fn chess_board_is_proper() {
        use crate::Chess;
        let chess: Chess = Chess::new();
        let chess_board = chess.board;
        assert_eq!(
            chess_board.get_board()[0][0].square_name(),
            String::from("A1")
        );
        assert_eq!(
            chess_board.get_board()[0][7].square_name(),
            String::from("H1")
        );
        assert_eq!(
            chess_board.get_board()[1][1].square_name(),
            String::from("B2")
        );
        assert_eq!(
            chess_board.get_board()[2][2].square_name(),
            String::from("C3")
        );
        assert_eq!(
            chess_board.get_board()[3][3].square_name(),
            String::from("D4")
        );
        assert_eq!(
            chess_board.get_board()[4][4].square_name(),
            String::from("E5")
        );
        assert_eq!(
            chess_board.get_board()[5][5].square_name(),
            String::from("F6")
        );
        assert_eq!(
            chess_board.get_board()[6][6].square_name(),
            String::from("G7")
        );
        assert_eq!(
            chess_board.get_board()[7][7].square_name(),
            String::from("H8")
        );
        assert_eq!(
            chess_board.get_board()[4][2].square_name(),
            String::from("C5")
        );
        assert_eq!(
            chess_board.get_board()[7][0].square_name(),
            String::from("A8")
        );

        assert_eq!(
            chess_board.get_board()[7][1].square_name(),
            String::from("B8")
        );
        assert_eq!(
            chess_board.get_board()[6][1].square_name(),
            String::from("B7")
        );
        assert_eq!(
            chess_board.get_board()[1][0].square_name(),
            String::from("A2")
        );
        assert_eq!(
            chess_board.get_board()[0][1].square_name(),
            String::from("B1")
        );
    }
}
