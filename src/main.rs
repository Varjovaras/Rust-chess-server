mod chess;
mod chessboard;
mod empty_piece;
mod pawn;
mod piece;

use chess::Chess;

fn main() {
    let chess: Chess = Chess::new();
    let chess_board = chess.board;
    chess_board.print_board_white();
    chess_board.print_board_black();
}
