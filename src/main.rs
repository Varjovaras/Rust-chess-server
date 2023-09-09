mod chess;
mod chessboard;
mod moves;
mod piece;

use chess::Chess;

fn main() {
    let mut chess: Chess = Chess::new();
    chess.starting_position();
    chess.board.print_board_white();
    // chess_board.print_board_black();
    println!("{:?}", chess.board.get_board()[0]);
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
