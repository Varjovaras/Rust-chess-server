use core::panic;

enum Files {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Debug)]
struct Square {
    file: u8,
    row: u8,
    name: String,
}

impl Square {
    fn new(file: u8, row: u8) -> Square {
        let file_string: String = match file {
            0 => String::from("A"),
            1 => String::from("B"),
            2 => String::from("C"),
            3 => String::from("D"),
            4 => String::from("E"),
            5 => String::from("F"),
            6 => String::from("G"),
            7 => String::from("H"),
            _ => panic!("Invalid file"),
        };
        let row_string: String = match row {
            0 => String::from("1"),
            1 => String::from("2"),
            2 => String::from("3"),
            3 => String::from("4"),
            4 => String::from("5"),
            5 => String::from("6"),
            6 => String::from("7"),
            7 => String::from("8"),
            _ => panic!("Invalid row"),
        };
        Square {
            file,
            row,
            name: String::from(file_string + &row_string),
        }
    }
}

#[derive(Debug)]
struct Chess {
    board: Vec<Vec<Square>>,
}

impl Chess {
    fn new() -> Chess {
        let mut board: Vec<Vec<Square>> = Vec::new();

        for i in 0..8 {
            let mut row: Vec<Square> = Vec::new();
            for j in 0..8 {
                let sq = Square::new(i, j);
                row.push(sq);
            }
            board.push(row);
        }

        Chess { board }
    }
}

fn main() {
    let mut chess_board = Chess::new();
    println!("{:?}", chess_board.board)
}
