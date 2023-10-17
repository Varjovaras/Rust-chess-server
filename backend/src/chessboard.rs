pub mod file;
pub mod rank;
pub mod square;

use crate::piece::{
    Piece::Bishop,
    Piece::King,
    Piece::Pawn,
    Piece::Queen,
    Piece::Rook,
    Piece::{self, Knight},
    PieceColor::Black,
    PieceColor::{self, White},
};

use self::{
    file::File,
    rank::Rank,
    square::{Square, SquareColor},
};

pub type ChessBoard = [[Square; 8]; 8];

pub fn new_board() -> ChessBoard {
    let mut board: ChessBoard = [[Square::default(); 8]; 8]; //initialize empty 8*8 board
    let mut color = SquareColor::Black; //starting color of the bottom left corner

    //initialize empty row
    let default_row = [Square::default(); 8];
    let files = File::get_files();
    let ranks = Rank::get_ranks();

    for file in files {
        let mut row = default_row;
        //initialize squares to a row, for example A1, A2, A3, A4, A5, A6, A7, A8
        for rank in ranks {
            let sq = Square::new(file, rank, color, Piece::default());
            row[rank as usize] = sq;
            color = self::color_changer(color); //every other square is black or white
        }
        board[file as usize] = row;
        //when row changes, do additional color change
        color = self::color_changer(color);
    }
    board
}

fn color_changer(color: SquareColor) -> SquareColor {
    //for initializing board
    match color {
        SquareColor::White => SquareColor::Black,
        SquareColor::Black => SquareColor::White,
    }
}

pub fn starting_position() -> ChessBoard {
    let mut clone_board = new_board();
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

    for (i, white_piece) in white_pieces.iter().enumerate() {
        clone_board[i][0].piece = *white_piece;
    }
    for (i, white_pawn) in white_pawns.iter().enumerate() {
        clone_board[i][1].piece = *white_pawn;
    }
    for (i, black_pawn) in black_pawns.iter().enumerate() {
        clone_board[i][6].piece = *black_pawn;
    }
    for (i, black_piece) in black_pieces.iter().enumerate() {
        clone_board[i][7].piece = *black_piece;
    }
    clone_board
}

pub fn get_white_king(board: &ChessBoard) -> Option<&Square> {
    for file in board.iter().take(8) {
        for square in file {
            if square.piece == Piece::King(PieceColor::White) {
                return Some(square);
            }
        }
    }
    None
}

pub fn get_black_king(board: &ChessBoard) -> Option<&Square> {
    for file in board.iter().take(8) {
        for square in file {
            if square.piece == Piece::King(PieceColor::Black) {
                return Some(square);
            }
        }
    }
    None
}

pub fn get_squares_with_white_pieces(board: &ChessBoard) -> Vec<Square> {
    let mut squares_with_pieces: Vec<Square> = Vec::new();
    for file in board.iter().take(8) {
        for square in file {
            if square.piece.color() == &PieceColor::White {
                squares_with_pieces.push(*square);
            }
        }
    }
    squares_with_pieces
}

pub fn get_squares_with_black_pieces(board: &ChessBoard) -> Vec<Square> {
    let mut squares_with_pieces: Vec<Square> = Vec::new();
    for file in board.iter().take(8) {
        for square in file {
            if square.piece.color() == &PieceColor::Black {
                squares_with_pieces.push(*square);
            }
        }
    }
    squares_with_pieces
}

pub fn get_adjancent_squares(sq: &Square, board: &ChessBoard) -> Vec<Square> {
    let mut adjancent_squares: Vec<Square> = Vec::new();
    let file = sq.file as usize;
    let rank = sq.rank as usize;
    if file > 0 {
        adjancent_squares.push(board[file - 1][rank]);
    }
    if file < 7 {
        adjancent_squares.push(board[file + 1][rank]);
    }
    if rank > 0 {
        adjancent_squares.push(board[file][rank - 1]);
    }
    if rank < 7 {
        adjancent_squares.push(board[file][rank + 1]);
    }
    if file > 0 && rank > 0 {
        adjancent_squares.push(board[file - 1][rank - 1]);
    }
    if file > 0 && rank < 7 {
        adjancent_squares.push(board[file - 1][rank + 1]);
    }
    if file < 7 && rank > 0 {
        adjancent_squares.push(board[file + 1][rank - 1]);
    }
    if file < 7 && rank < 7 {
        adjancent_squares.push(board[file + 1][rank + 1]);
    }

    adjancent_squares
}

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        piece::{Piece, PieceColor},
    };

    #[test]
    fn chess_board_is_proper() {
        let chess: Chess = Chess::new();
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
        let chess: Chess = Chess::new();
        assert_eq!(chess.board[0][0]._square_color(), SquareColor::Black);
        assert_eq!(chess.board[1][1]._square_color(), SquareColor::Black);
        assert_eq!(chess.board[2][1]._square_color(), SquareColor::White);
        assert_eq!(chess.board[7][6]._square_color(), SquareColor::White);
        assert_eq!(chess.board[0][7]._square_color(), SquareColor::White);
    }

    #[test]
    fn starting_position_works() {
        let mut chess: Chess = Chess::new();
        chess.starting_position();
        assert_eq!(chess.board[0][0].piece, Piece::Rook(PieceColor::White))
    }
}
