use crate::square::Square;

#[derive(Debug)]
pub struct Board {
    //Inner vector is rank, outer vector file
    pub board: [[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let mut board: [[Square; 8]; 8] = [[Square::default(); 8]; 8];
        for i in 0..8 {
            let mut row: [Square; 8] = [Square::default(); 8];
            for j in 0..8 {
                let sq = Square::new(i, j);
                row[j as usize] = sq;
            }
            board[i as usize] = row;
        }
        Board { board }
    }

    // pub fn get_square(&self, file: u8, rank: u8) -> &Square {
    //     // let square = self.board[];
    //     square
    // }

    // pub fn print_squares(&self) {
    //     self.board.iter().for_each(|row| {
    //         row.iter().for_each(|square| {
    //             print!("{} ", square.get_name());
    //         });
    //     });
    //     println!("");
    // }
}
