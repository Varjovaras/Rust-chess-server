mod board;
mod chess;
mod piece;
mod square;

use chess::Chess;

fn main() {
    let chess: Chess = Chess::new();
    let chess_board = chess.board;
    chess_board.print_squares();
}
