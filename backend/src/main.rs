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
use serde::{Deserialize, Serialize};
use shuttle_axum::ShuttleAxum;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct MoveRequest {
    pub list_of_moves: Vec<(String, String)>,
    pub new_move: (String, String),
}

#[derive(Debug, Serialize)]
struct MoveResponse {
    pub chess: Chess,
}

use crate::chess::Chess;
//cargo watch -qcx 'shuttle run'

async fn move_chess(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<MoveRequest>,
) -> (StatusCode, Json<MoveResponse>) {
    println!("{:?}", payload);
    let mut chess = Chess::new_starting_position();

    for move_tuple in payload.list_of_moves {
        chess.make_move_from_str(move_tuple.0.as_str(), move_tuple.1.as_str());
    }

    chess.make_move_from_str(payload.new_move.0.as_str(), payload.new_move.1.as_str());
    chess.make_move_from_str("e7", "e5");
    chess.make_move_from_str("d2", "d4");
    chess.make_move_from_str("d7", "d5");

    (StatusCode::CREATED, Json(MoveResponse { chess }))
}

async fn chess() -> String {
    serde_json::to_string(&Chess::new_starting_position()).unwrap()
}

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        // allow requests from any origin
        .allow_origin(Any);
    let router = Router::new()
        .route("/chess", get(chess))
        .route("/chess", post(move_chess))
        .layer(cors);

    Ok(router.into())
}
