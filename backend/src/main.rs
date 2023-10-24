mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod gamestate;
mod moves;
mod piece;
mod player;

use axum::{
    http::{header::CONTENT_TYPE, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use piece::Piece;
use serde::{Deserialize, Serialize};
use shuttle_axum::ShuttleAxum;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct MoveRequest {
    list_of_moves: Vec<[(String, String, String, Piece); 2]>,
    pub new_move: (String, String),
}

#[derive(Debug, Serialize)]
struct MoveResponse {
    pub chess: Chess,
}

use crate::chess::Chess;

async fn move_chess(Json(payload): Json<MoveRequest>) -> (StatusCode, Json<MoveResponse>) {
    println!("{:?}", payload);
    let mut chess = Chess::new_starting_position();

    for move_tuple in payload.list_of_moves {
        chess.make_move_from_str(move_tuple[0].0.as_str(), move_tuple[1].1.as_str());
    }

    chess.make_move_from_str(payload.new_move.0.as_str(), payload.new_move.1.as_str());
    chess.make_move_from_str("e7", "e5");
    chess.make_move_from_str("d2", "d4");
    chess.make_move_from_str("d7", "d5");

    let response = MoveResponse { chess };
    (StatusCode::OK, Json(response))
}
async fn chess() -> String {
    serde_json::to_string(&Chess::new_starting_position()).unwrap()
}

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(Any);
    let router = Router::new()
        .route("/chess", get(chess))
        .route("/chess", post(move_chess))
        .layer(cors);

    Ok(router.into())
}
