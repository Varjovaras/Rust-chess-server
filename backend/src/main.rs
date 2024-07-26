use axum::{
    http::{header::CONTENT_TYPE, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use shuttle_axum::ShuttleAxum;
use tower_http::cors::{Any, CorsLayer};
use chess::Chess;
use chess::chessboard::file::File;
use chess::chessboard::rank::Rank;

#[derive(Debug, Deserialize)]
struct MoveRequest {
    list_of_moves: Vec<((String, usize), (String, usize))>,
    new_move: [String; 2],
}

#[derive(Debug, Serialize)]
struct MoveResponse {
    pub chess: Chess,
}


async fn move_chess(Json(payload): Json<MoveRequest>) -> (StatusCode, Json<MoveResponse>) {
    //setup new chess and iter and handle moves provided by payload
    let mut chess = Chess::new_starting_position();

    for move_tuple in &payload.list_of_moves {
        let start_sq = chess.get_square(
            File::try_from(move_tuple.0.0.as_str()).expect("invalid file"), // try to convert from string
            Rank::try_from(move_tuple.0.1).expect("invalid rank"), // rank is usize already
        );
        let end_sq = chess.get_square(
            File::try_from(move_tuple.1.0.as_str()).expect("invalid file"), // try to convert from string
            Rank::try_from(move_tuple.1.1).expect("invalid rank"),
        );
        chess.make_move(&start_sq, &end_sq);
    }

    chess.make_move_from_str(&payload.new_move[0], &payload.new_move[1]);
    let response = MoveResponse { chess };
    (StatusCode::OK, Json(response))
}

async fn chess() -> (StatusCode, Json<MoveResponse>) {
    let chess = Chess::new_starting_position();
    let response = MoveResponse { chess };
    (StatusCode::OK, Json(response))
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

// fn main() {
//     let mut chess = Chess::new_starting_position();
//     chess.make_move_from_str("e2", "e4");
//     chess._print_board_white();
//     chess.make_move_from_str("d7", "d5");

//     println!("{:?}", chess.board[4][3]);
// }
