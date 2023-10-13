mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod moves;
mod piece;

use chess::Chess;
use chessboard::{file::File, rank::Rank};
use rand::Rng;

fn main() {
    let mut chess: Chess = Chess::new();
    chess.starting_position();

    let mut i = 0;

    loop {
        let files = File::get_files();
        let ranks = Rank::get_ranks();
        let mut rng = rand::thread_rng();
        let n1: usize = rng.gen_range(0..8);
        let n2: usize = rng.gen_range(0..8);
        let n3: usize = rng.gen_range(0..8);
        let n4: usize = rng.gen_range(0..8);

        let start_sq_file = files[n1];
        let start_sq_rank = ranks[n2];
        let end_sq_file = files[n3];
        let end_sq_rank = ranks[n4];

        let mut sq1 = *chess.get_square(start_sq_file, start_sq_rank);
        let mut sq2 = *chess.get_square(end_sq_file, end_sq_rank);
        chess.make_move(&mut sq1, &mut sq2);

        i += 1;

        if i % 10000 == 0 {
            println!("i = {}", i);
        }

        if i % 10000 == 0 {
            println!("i = {}", i);
            chess.print_board_white();
        }
        if chess.white_won || chess.black_won {
            println!("i = {}", i);
            println!("White won: {}", chess.white_won);
            println!("Black won: {}", chess.black_won);
            chess.print_board_white();
            break;
        }
        // chess.print_board_white();
    }
}
