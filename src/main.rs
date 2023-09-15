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
    println!("{:?}", chess.board[0][1]);

    let sq1 = *chess.get_square_from_str("e", "2");
    let sq2 = *chess.get_square_from_str("e", "4");

    chess.make_move(&sq1, &sq2);

    // let mut i = 0;
    // loop {
    //     let mut chess: Chess = Chess::new();
    //     chess.starting_position();
    //     let chess_board = chess.board;
    //     if i == 100000000 {
    //         break;
    //     }

    //     if i % 10000 == 0 {
    //         println!("{}", i);
    //         chessboard::print_board_white(&chess_board);
    //     }
    //     i += 1;
    // }
}
