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
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
// use tower::{Service, ServiceBuilder, ServiceExt};
use tower_http::cors::{any, CorsLayer};

// use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

use crate::chess::Chess;
//cargo watch -qcx 'shuttle run'

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn chess() -> String {
    serde_json::to_string(&Chess::new_starting_position()).unwrap()
}

async fn chess_move() -> impl IntoResponse {
    let mut chess = Chess::new();
    chess.starting_position();
    chess.make_move_from_str("e2", "e4");
    (
        StatusCode::OK,
        serde_json::to_string(&Chess::new_starting_position()).unwrap(),
    )
}

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(any());
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/chess", get(chess))
        .route("/chess", post(chess_move))
        .layer(cors);

    Ok(router.into())
}
