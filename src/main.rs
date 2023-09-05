mod chess;
mod piece;
mod square;

use chess::Chess;

fn main() {
    let chess_board: Chess = Chess::new();
    chess_board.print_squares();
}
