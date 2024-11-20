#![allow(clippy::redundant_pub_crate)]
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use chess::{
    chessboard::{file::File, rank::Rank},
    Chess,
};
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use shuttle_axum::ShuttleAxum;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio::{
    sync::{watch, Mutex},
    time::sleep,
};
use tower_http::cors::{Any, CorsLayer};

struct State {
    clients_count: usize,
    rx: watch::Receiver<Message>,
    chess: Arc<Mutex<Chess>>,
}

const PAUSE_SECS: u64 = 15;
const STATUS_URI: &str = "https://api.shuttle.rs";

#[derive(Serialize)]
struct Response {
    clients_count: usize,
    #[serde(rename = "dateTime")]
    date_time: DateTime<Utc>,
    is_up: bool,
}

#[derive(Debug, Deserialize, Clone)]
struct MoveRequest {
    #[allow(clippy::type_complexity)]
    list_of_moves: Vec<((String, usize), (String, usize), (usize, usize))>,
    new_move: (String, String, (usize, usize)),
}

#[derive(Debug, Deserialize, Clone)]
struct ResetRequest {
    action: String,
}

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    let (tx, rx) = watch::channel(Message::Text("{}".to_string()));
    let chess = Arc::new(Mutex::new(Chess::new_starting_position()));
    let state = Arc::new(Mutex::new(State {
        clients_count: 0,
        rx,
        chess,
    }));

    // Spawn a thread to continually check the status of the api
    let state_send = state.clone();
    tokio::spawn(async move {
        let duration = Duration::from_secs(PAUSE_SECS);

        loop {
            let is_up = reqwest::get(STATUS_URI).await;
            let is_up = is_up.is_ok();

            let response = Response {
                clients_count: state_send.lock().await.clients_count,
                date_time: Utc::now(),
                is_up,
            };
            #[allow(clippy::unwrap_used)]
            let msg = serde_json::to_string(&response).unwrap();

            if tx.send(Message::Text(msg)).is_err() {
                break;
            }

            sleep(duration).await;
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/websocket", get(websocket_handler))
        .route("/status", get(get_status))
        .layer(cors)
        .layer(Extension(state));

    Ok(router.into())
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Mutex<State>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

#[allow(clippy::too_many_lines)]
async fn websocket(stream: WebSocket, state: Arc<Mutex<State>>) {
    let (mut sender, mut receiver) = stream.split();
    let mut rx = {
        let mut state = state.lock().await;
        state.clients_count += 1;
        state.rx.clone()
    };

    let chess = state.lock().await.chess.clone();
    // Create a channel for communication between tasks
    let (tx, mut rx_channel) = mpsc::channel(100);

    let initial_chess_state = {
        let chess_game = chess.lock().await;
        serde_json::json!({
            "type": "initial_state",
            "chess": *chess_game
        })
    };

    if (sender
        .send(Message::Text(
            serde_json::to_string(&initial_chess_state).expect("Failed initializing chess state"),
        ))
        .await)
        .is_err()
    {
        return; // Client disconnected
    }

    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rx.changed() => {
                    let msg = rx.borrow().clone();
                    if sender.send(msg).await.is_err() {
                        break;
                    }
                }
                Some(msg) = rx_channel.recv() => {
                    if sender.send(msg).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(move_request) = serde_json::from_str::<MoveRequest>(&text) {
                let mut chess_game = chess.lock().await;
                // Apply previous moves
                for move_tuple in &move_request.list_of_moves {
                    let start_sq = chess_game.get_square(
                        File::try_from(move_tuple.0 .0.as_str()).expect("invalid file"),
                        Rank::try_from(move_tuple.0 .1).expect("invalid rank"),
                    );
                    let end_sq = chess_game.get_square(
                        File::try_from(move_tuple.1 .0.as_str()).expect("invalid file"),
                        Rank::try_from(move_tuple.1 .1).expect("invalid rank"),
                    );
                    #[allow(clippy::cast_possible_truncation)]
                    #[allow(clippy::cast_possible_wrap)]
                    let promoted_piece = (move_tuple.2 .0 as i32, move_tuple.2 .1 as i32);
                    chess_game.make_move(&start_sq, &end_sq, promoted_piece);
                }

                let promoted_piece = match move_request.new_move.2 {
                    (1, 0 | 1) => Some("QUEEN"),
                    (2, 0 | 1) => Some("ROOK"),
                    (3, 0 | 1) => Some("KNIGHT"),
                    (4, 0 | 1) => Some("BISHOP"),
                    _ => None,
                };

                // Make the new move
                chess_game.make_move_from_str(
                    &move_request.new_move.0,
                    &move_request.new_move.1,
                    promoted_piece,
                );

                // Send updated chess state to all clients
                let response = serde_json::json!({
                    "type": "update",
                    "chess": *chess_game
                });
                #[allow(clippy::unwrap_used)]
                let response_json = serde_json::to_string(&response).unwrap();
                if tx.send(Message::Text(response_json)).await.is_err() {
                    break;
                }
                // Explicitly drop chess_game here
                drop(chess_game);
            } else if let Ok(reset_request) = serde_json::from_str::<ResetRequest>(&text) {
                if reset_request.action == "reset" {
                    let mut chess_game = chess.lock().await;
                    *chess_game = Chess::new_starting_position();

                    // Send updated chess state to all clients
                    let response = serde_json::json!({
                        "type": "reset",
                        "chess": *chess_game
                    });
                    #[allow(clippy::unwrap_used)]
                    let response_json = serde_json::to_string(&response).unwrap();
                    if tx.send(Message::Text(response_json)).await.is_err() {
                        break;
                    }

                    // Explicitly drop chess_game here
                    drop(chess_game);
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    state.lock().await.clients_count -= 1;
}

async fn get_status() -> impl IntoResponse {
    Chess::new_starting_position().to_json()
}
