mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod gamestate;
mod moves;
mod piece;
mod player;

use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

use crate::chess::Chess;

//cargo watch -qcx 'shuttle run'

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn init_chess() -> Chess {
    let mut chess = Chess::new();
    chess.starting_position();
    chess.make_move_from_str("e2", "e4");
    chess
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    let _chess = init_chess().await;

    Ok(config.into())
}
