use crate::square::Square;

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<Square>>,
}

impl Board {
    pub fn new() -> Board {
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
        Board { board }
    }

    pub fn print_squares(&self) {
        self.board.iter().for_each(|row| {
            row.iter().for_each(|square| {
                print!("{} ", square.get_name());
            });
        })
    }
}
