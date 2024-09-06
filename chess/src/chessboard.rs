pub mod file;
pub mod rank;
pub mod square;

use std::array::from_fn;

use crate::{
    chess::Chess,
    piece::{
        Piece::{self, King},
        PieceColor::{self, Black, White},
    },
};

use self::{
    file::File,
    rank::Rank,
    square::{Square, SquareColor},
};

pub type ChessBoard = [[Square; 8]; 8];

// pub fn new_board() -> ChessBoard {
//     let mut board: ChessBoard = [[Square::default(); 8]; 8]; //initialize empty 8*8 board
//     let mut color = SquareColor::Black; //starting color of the bottom left corner

//     //initialize empty row
//     let default_row = [Square::default(); 8];
//     let files = File::get_files();
//     let ranks = Rank::get_ranks();

//     for file in files {
//         let mut row = default_row;
//         //initialize squares to a row, for example A1, A2, A3, A4, A5, A6, A7, A8
//         for rank in ranks {
//             let sq = Square::new(file, rank, color, Piece::default());
//             row[rank as usize] = sq;
//             color = color_changer(color); //every other square is black or white
//         }
//         board[file as usize] = row;
//         //when row changes, do additional color change
//         color = color_changer(color);
//     }
//     board
// }

#[must_use]
pub fn new_board() -> ChessBoard {
    let mut color = SquareColor::Black; //starting color of the bottom left corner

    let files = File::get_files();
    let ranks = Rank::get_ranks();

    let board: ChessBoard = from_fn(|file| {
        let mut rank_color = color;
        let rank_array = from_fn(|rank| {
            let sq = Square::new(files[file], ranks[rank], rank_color, Piece::default());
            rank_color = color_changer(rank_color); //every other square is black or white
            sq
        });
        color = color_changer(color); // change starting color for each rank
        rank_array
    });

    board
}
const fn color_changer(color: SquareColor) -> SquareColor {
    //for initializing board
    match color {
        SquareColor::White => SquareColor::Black,
        SquareColor::Black => SquareColor::White,
    }
}

#[must_use]
pub fn starting_position() -> ChessBoard {
    let mut board = new_board();
    let pieces = [
        Piece::Rook,
        Piece::Knight,
        Piece::Bishop,
        Piece::Queen,
        Piece::King,
        Piece::Bishop,
        Piece::Knight,
        Piece::Rook,
    ];

    set_pieces(
        &mut board,
        White,
        0,
        &pieces
            .iter()
            .map(|piece_fn| piece_fn(White))
            .collect::<Vec<_>>(),
    );
    set_pieces(&mut board, White, 1, &[Piece::Pawn(White); 8]);
    set_pieces(&mut board, Black, 6, &[Piece::Pawn(Black); 8]);
    set_pieces(
        &mut board,
        Black,
        7,
        &pieces
            .iter()
            .map(|piece_fn| piece_fn(Black))
            .collect::<Vec<_>>(),
    );
    board
}

fn set_pieces(board: &mut ChessBoard, color: PieceColor, rank: usize, pieces: &[Piece]) {
    for (file, &piece_type) in pieces.iter().enumerate() {
        board[file][rank].piece = match piece_type {
            Piece::Rook(_) => Piece::Rook(color),
            Piece::Knight(_) => Piece::Knight(color),
            Piece::Bishop(_) => Piece::Bishop(color),
            Piece::Queen(_) => Piece::Queen(color),
            Piece::King(_) => Piece::King(color),
            Piece::Pawn(_) => Piece::Pawn(color),
            Piece::None => panic!("Invalid piece type"),
        };
    }
}

#[must_use]
pub fn get_white_king(board: &ChessBoard) -> Option<&Square> {
    for file in board.iter().take(8) {
        for square in file {
            if square.piece == King(White) {
                return Some(square);
            }
        }
    }
    None
}

#[must_use]
pub fn get_black_king(board: &ChessBoard) -> Option<&Square> {
    for file in board.iter().take(8) {
        for square in file {
            if square.piece == King(Black) {
                return Some(square);
            }
        }
    }
    None
}

#[must_use]
pub fn get_squares_with_white_pieces(board: &ChessBoard) -> Vec<&Square> {
    board
        .iter()
        .flatten()
        .filter(|&square| square.piece.color() == White)
        .collect()
}

#[must_use]
pub fn get_squares_with_black_pieces(board: &ChessBoard) -> Vec<&Square> {
    board
        .iter()
        .flatten()
        .filter(|&square| square.piece.color() == Black)
        .collect()
}

#[must_use]
pub fn get_adjacent_squares(sq: &Square, board: &ChessBoard) -> Vec<Square> {
    let mut adjacent_squares: Vec<Square> = Vec::new();
    let file = sq.file as isize;
    let rank = sq.rank as isize;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for &(file_direction, rank_direction) in &directions {
        let new_file = file + file_direction;
        let new_rank = rank + rank_direction;
        if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
            #[allow(clippy::cast_sign_loss)]
            adjacent_squares.push(board[new_file as usize][new_rank as usize].clone());
        }
    }

    adjacent_squares
}

pub fn add_possible_moves_to_squares(chess: &mut Chess) {
    let possible_moves: Vec<Vec<_>> = chess
        .board
        .iter()
        .map(|row| {
            row.iter()
                .map(|square| square.possible_legal_moves(chess))
                .collect()
        })
        .collect();

    for (i, row) in chess.board.iter_mut().enumerate() {
        for (j, square) in row.iter_mut().enumerate() {
            square.possible_moves.clone_from(&possible_moves[i][j]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        piece::{Piece, PieceColor},
    };

    #[test]
    fn chess_board_is_proper() {
        let chess: Chess = Chess::default();
        assert_eq!(chess.board[0][0]._square_name(), String::from("A1"));
        assert_eq!(chess.board[0][7]._square_name(), String::from("A8"));
        assert_eq!(chess.board[1][1]._square_name(), String::from("B2"));
        assert_eq!(chess.board[2][2]._square_name(), String::from("C3"));
        assert_eq!(chess.board[3][3]._square_name(), String::from("D4"));
        assert_eq!(chess.board[4][4]._square_name(), String::from("E5"));
        assert_eq!(chess.board[5][5]._square_name(), String::from("F6"));
        assert_eq!(chess.board[6][6]._square_name(), String::from("G7"));
        assert_eq!(chess.board[7][7]._square_name(), String::from("H8"));
        assert_eq!(chess.board[4][2]._square_name(), String::from("E3"));
        assert_eq!(chess.board[7][0]._square_name(), String::from("H1"));
        assert_eq!(chess.board[7][1]._square_name(), String::from("H2"));
        assert_eq!(chess.board[6][1]._square_name(), String::from("G2"));
        assert_eq!(chess.board[1][0]._square_name(), String::from("B1"));
        assert_eq!(chess.board[0][1]._square_name(), String::from("A2"));
    }

    #[test]
    fn squares_are_right_colors() {
        use crate::chessboard::square::SquareColor;
        let chess: Chess = Chess::default();
        assert_eq!(chess.board[0][0]._square_color(), SquareColor::Black);
        assert_eq!(chess.board[1][1]._square_color(), SquareColor::Black);
        assert_eq!(chess.board[2][1]._square_color(), SquareColor::White);
        assert_eq!(chess.board[7][6]._square_color(), SquareColor::White);
        assert_eq!(chess.board[0][7]._square_color(), SquareColor::White);
    }

    #[test]
    fn starting_position_works() {
        let mut chess: Chess = Chess::default();
        chess.starting_position();
        assert_eq!(chess.board[0][0].piece, Piece::Rook(PieceColor::White));
    }
}
