mod castling;
mod check;
mod chess;
mod chessboard;
mod moves;
mod piece;

use chess::Chess;
use rand::Rng;

fn main() {
    let mut chess: Chess = Chess::new();
    chess.starting_position();
    // // chess_board.print_board_black();
    // // println!("{:?}", chess.board[0]);
    // // println!("{:?}", chess.board[0][1]);

    let files = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let ranks = ["1", "2", "3", "4", "5", "6", "7", "8"];

    // chessboard::print_board_white(&chess.board);
    let mut sq1 = *chess.get_square_from_str("e", "2");
    let mut sq2 = *chess.get_square_from_str("e", "4");

    chess.make_move(&mut sq1, &mut sq2);
    let mut chess: Chess = Chess::new();
    chess.starting_position();

    let mut i = 0;
    loop {
        chess = Chess::new();
        chess.starting_position();
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..=7);
        let random_number2 = rng.gen_range(0..=7);
        let random_number3 = rng.gen_range(0..=7);
        let random_number4 = rng.gen_range(0..=7);

        // chess._print_board_white();
        chess._make_move_from_str(
            format!("{}{}", files[random_number], ranks[random_number2]).as_str(),
            format!("{}{}", files[random_number3], ranks[random_number4]).as_str(),
        );

        if i == 10000000 {
            break;
        }

        if i % 1000000 == 0 {
            println!("{}", i);
            chess._print_board_white();
        }
        i += 1;
    }
}
