mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod moves;
mod piece;

use chess::Chess;
use std::io::{self};

fn main() -> io::Result<()> {
    let mut chess: Chess = Chess::new();
    chess.starting_position();
    // // chess_board.print_board_black();
    // // println!("{:?}", chess.board[0]);
    // // println!("{:?}", chess.board[0][1]);

    // let files = ["a", "b", "c", "d", "e", "f", "g", "h"];
    // let ranks = ["1", "2", "3", "4", "5", "6", "7", "8"];

    // chessboard::print_board_white(&chess.board);
    let mut sq1 = *chess.get_square_from_str("e", "2");
    let mut sq2 = *chess.get_square_from_str("e", "4");

    chess.make_move(&mut sq1, &mut sq2);
    let mut chess: Chess = Chess::new();
    chess.starting_position();

    let mut i = 0;

    loop {
        // let mut sq1 = String::new();
        // let mut sq2 = String::new();
        chess.make_move_from_str("f2", "f3");
        chess.make_move_from_str("e7", "e5");
        chess.make_move_from_str("g2", "g4");
        chess.make_move_from_str("d8", "h4");

        // println!("Enter sq1 :");
        // println!("Enter sq1 :");
        // io::stdin().read_line(&mut sq2)?;

        // print!("\x1B[2J\x1B[1;1H");
        // chess.make_move_from_str(sq1.as_str(), sq2.as_str());

        // chess.print_board_white();
        i += 1;
        if i % 10 == 0 {
            println!("i = {}", i);
            chess.print_board_white();
        }

        if i % 100 == 0 {
            break;
        }

        if chess.game_over {}

        // io::stdin().read_line(&mut sq1)?;
    }

    Ok(())
}
