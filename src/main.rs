mod chess;
mod chessboard;
mod piece;

use chess::Chess;

fn main() {
    let chess: Chess = Chess::new();
    let chess_board = chess.board;
    // chess_board.print_board_white();
    println!("{:?}", chess_board.get_board());
    chess_board.print_board_black();
}
