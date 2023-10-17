mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod gamestate;
mod moves;
mod piece;
mod player;

use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

use shuttle_actix_web::ShuttleActixWeb;

use crate::chess::Chess;

//cargo watch -qcx 'shuttle run'

#[get("/")]
async fn hello_world() -> impl Responder {
    let chess = init_chess().await;
    HttpResponse::Ok().body(chess.to_json())
}

// #[post("/move")]
// async fn make_move(mut payload: web::Payload) -> Result<HttpResponse, Error> {
//     // // payload is a stream of Bytes objects
//     // let mut body = web::BytesMut::new();
//     // while let Some(chunk) = payload.next().await {
//     //     let chunk = chunk?;
//     //     // limit max size of in-memory payload
//     //     if (body.len() + chunk.len()) > MAX_SIZE {
//     //         return Err(error::ErrorBadRequest("overflow"));
//     //     }
//     //     body.extend_from_slice(&chunk);
//     // }

//     // // body is loaded, now we can deserialize serde-json
//     // let obj = serde_json::from_slice::<MyObj>(&body)?;
//     // Ok(HttpResponse::Ok().json(obj)) // <- send response

//     Err("Not implemented".into())
// }

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
