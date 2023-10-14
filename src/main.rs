mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod gameover;
mod moves;
mod piece;
mod player;

use chess::Chess;
use chessboard::{file::File, rank::Rank};
use rand::Rng;

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

fn main() {
    random_move_simulator();
    Chess::new().print_board_white();
}

fn random_move_simulator() {
    let mut chess: Chess = Chess::new();
    chess.starting_position();
    let mut i = 0;
    let mut white_wins = 0;
    let mut black_wins = 0;
    let mut ties = 0;
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

        if i % 100000000 == 0 {
            println!("White wins: {}", white_wins);
            println!("Black wins: {}", black_wins);
            println!("Ties: {}", ties);
            break;
        }
        if chess.white_player.won || chess.black_player.won || chess.tie {
            // println!("i = {}", i);
            // println!("White won: {}", chess.white_won);
            // println!("Black won: {}", chess.black_won);
            // println!("Tie: {}", chess.tie);
            if chess.white_player.won {
                white_wins += 1;
            } else if chess.black_player.won {
                black_wins += 1;
            } else if chess.tie {
                ties += 1;
            }
            // chess.print_board_white();
            println!("White wins: {}", white_wins);
            println!("Black wins: {}", black_wins);
            println!("Ties: {}", ties);
            chess = Chess::new();
            chess.starting_position();
            // break;
        }
    }
}
