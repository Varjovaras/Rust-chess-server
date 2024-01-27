mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod game_state;
mod make_chess_move;
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
    list_of_moves: Vec<((String, usize), (String, usize))>,
    new_move: [String; 2],
}

#[derive(Debug, Serialize)]
struct MoveResponse {
    pub chess: Chess,
}

use crate::{
    chess::Chess,
    chessboard::{file::File, rank::Rank},
};

async fn move_chess(Json(payload): Json<MoveRequest>) -> (StatusCode, Json<MoveResponse>) {
    //setup new chess and iter and handle moves provided by payload
    let mut chess = Chess::new_starting_position();

    for move_tuple in &payload.list_of_moves {
        let start_sq = chess.get_square(
            File::try_from(move_tuple.0 .0.as_str()).expect("invalid file"), // try to convert from string
            Rank::try_from(move_tuple.0 .1).expect("invalid rank"), // rank is usize already
        );
        let end_sq = chess.get_square(
            File::try_from(move_tuple.1 .0.as_str()).expect("invalid file"), // try to convert from string
            Rank::try_from(move_tuple.1 .1).expect("invalid rank"),
        );
        chess.make_move(start_sq, end_sq);
    }

    chess.make_move_from_str(&payload.new_move[0], &payload.new_move[1]);
    let response = MoveResponse { chess };
    (StatusCode::OK, Json(response))
}
async fn chess() -> &'static str {
    ""
}

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn axum() -> ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(Any);
    let router = Router::new()
        .route("/api/chess", get(chess))
        .route("/api/chess", post(move_chess))
        .layer(cors);

    Ok(router.into())
}
