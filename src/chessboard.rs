use crate::piece::{
    PieceColor::Black,
    PieceColor::White,
    Pieces::Bishop,
    Pieces::King,
    Pieces::Pawn,
    Pieces::Queen,
    Pieces::Rook,
    Pieces::{self, Knight},
};

use self::square::{Square, SquareColor};

pub mod file;
pub mod rank;
pub mod square;

pub type ChessBoard = [[Square; 8]; 8];

#[derive(Debug)]
/// 8*8 chessboard.
/// i stands for the file and
/// j stands for the rank.
/// file is the column (top to bottom) and rank is the row (left to right)
pub struct Board {
    board: ChessBoard,
}

impl Board {
    pub fn new() -> Board {
        let mut board: ChessBoard = [[Square::default(); 8]; 8]; //initialize empty 8*8 board
        let mut color = SquareColor::Black; //starting color of the bottom left corner

        //initialize empty row
        let default_row = [Square::default(); 8];

        for file in 0..8 {
            let mut row = default_row;
            //initialize squares to a row, for example A1, A2, A3, A4, A5, A6, A7, A8
            for rank in 0..8 {
                let sq = Square::new(file, rank, color, Pieces::default());
                row[rank as usize] = sq;
                color = Self::color_changer(color); //every other square is black or white
            }
            board[file as usize] = row;
            //when row changes, do additional color change
            color = Self::color_changer(color);
        }
        Board { board }
    }

    fn color_changer(color: SquareColor) -> SquareColor {
        match color {
            SquareColor::White => SquareColor::Black,
            SquareColor::Black => SquareColor::White,
        }
    }

    pub fn starting_position(&mut self) {
        let white_pieces = [
            Rook(White),
            Knight(White),
            Bishop(White),
            Queen(White),
            King(White),
            Bishop(White),
            Knight(White),
            Rook(White),
        ];
        let black_pieces = [
            Rook(Black),
            Knight(Black),
            Bishop(Black),
            Queen(Black),
            King(Black),
            Bishop(Black),
            Knight(Black),
            Rook(Black),
        ];
        let white_pawns = [Pawn(White); 8];
        let black_pawns = [Pawn(Black); 8];

        for i in 0..8 {
            self.board[0][i].piece = white_pieces[i];
        }
        for i in 0..8 {
            self.board[1][i].piece = white_pawns[i];
        }
        for i in 0..8 {
            self.board[7][i].piece = black_pieces[i];
        }
        for i in 0..8 {
            self.board[6][i].piece = black_pawns[i];
        }
    }

    pub fn get_board(&self) -> &ChessBoard {
        &self.board
    }

    // pub fn get_square(&self, file: u8, rank: u8) -> &Square {
    //     // let square = self.board[];
    //     square
    // }

    pub fn print_board_white(&self) {
        let mut clone_board = *self.get_board();
        clone_board.reverse();
        clone_board.iter().for_each(|row| {
            row.iter().for_each(|square| {
                print!("{} ", square.square_name());
            });
            println!();
        });
    }

    pub fn _print_board_black(&self) {
        let mut clone_board = *self.get_board();
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

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        piece::{PieceColor, Pieces},
    };

    #[test]
    fn chess_board_is_proper() {
        let chess: Chess = Chess::new();
        let chess_board = chess.board;
        assert_eq!(
            chess_board.get_board()[0][0].square_name(),
            String::from("A1")
        );

        assert_eq!(
            chess_board.get_board()[0][7].square_name(),
            String::from("A8")
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
            String::from("E3")
        );
        assert_eq!(
            chess_board.get_board()[7][0].square_name(),
            String::from("H1")
        );

        assert_eq!(
            chess_board.get_board()[7][1].square_name(),
            String::from("H2")
        );
        assert_eq!(
            chess_board.get_board()[6][1].square_name(),
            String::from("G2")
        );
        assert_eq!(
            chess_board.get_board()[1][0].square_name(),
            String::from("B1")
        );
        assert_eq!(
            chess_board.get_board()[0][1].square_name(),
            String::from("A2")
        );
    }

    #[test]
    fn squares_are_right_colors() {
        use crate::chessboard::square::SquareColor;
        let chess: Chess = Chess::new();
        let chess_board = chess.board;
        assert_eq!(
            chess_board.get_board()[0][0]._square_color(),
            SquareColor::Black
        );
        assert_eq!(
            chess_board.get_board()[1][1]._square_color(),
            SquareColor::Black
        );
        assert_eq!(
            chess_board.get_board()[2][1]._square_color(),
            SquareColor::White
        );
        assert_eq!(
            chess_board.get_board()[7][6]._square_color(),
            SquareColor::White
        );
        assert_eq!(
            chess_board.get_board()[0][7]._square_color(),
            SquareColor::White
        );
    }

    #[test]
    fn starting_position_works() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();
        assert_eq!(
            chess.board.get_board()[0][0].piece,
            Pieces::Rook(PieceColor::White)
        )
    }
}
