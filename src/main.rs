mod chess;
mod square;

use chess::Chess;

fn main() {
    let chess_board: Chess = Chess::new();
    println!("{:?}", chess_board.board)
}
