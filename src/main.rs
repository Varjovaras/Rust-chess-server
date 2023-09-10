mod chess;
mod chessboard;
mod moves;
mod piece;

use chess::Chess;

fn main() {
    let mut chess: Chess = Chess::new();
    chess.starting_position();
    chessboard::print_board_white(&chess.board);
    // chess_board.print_board_black();
    println!("{:?}", chess.board[0]);
    let mut i = 0;
    loop {
        let chess: Chess = Chess::new();
        let chess_board = chess.board;
        if i == 100000000 {
            break;
        }

        if i % 10000 == 0 {
            println!("{}", i);
            chessboard::print_board_white(&chess_board);
        }
        i += 1;
    }
}
