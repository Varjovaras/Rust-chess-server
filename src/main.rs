mod board;
mod chess;
mod file;
mod piece;
mod rank;
mod square;

use chess::Chess;

fn main() {
    let chess: Chess = Chess::new();
    let _chess_board = chess.board;
    // chess_board.print_squares();
}
