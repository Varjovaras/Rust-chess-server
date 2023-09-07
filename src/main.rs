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
    // chess_board.print_board_black();
    println!("{:?}", chess_board.get_board()[0]);
    let mut i = 0;
    loop {
        let chess: Chess = Chess::new();
        let chess_board = chess.board;
        if i == 100000000 {
            break;
        }

        if i % 10000 == 0 {
            println!("{}", i);
            chess_board.print_board_white();
        }
        i += 1;
    }
}
