mod castling;
mod check;
mod checkmate;
mod chess;
mod chessboard;
mod gamestate;
mod moves;
mod piece;
mod player;
use std::io;

use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, post, App, HttpResponse, HttpServer, Responder,
};

// use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

use crate::chess::Chess;

//cargo watch -qcx 'shuttle run'

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/chess")]
async fn get_chess() -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&Chess::new_starting_position()).unwrap())
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(hello)
            .service(get_chess)
            .wrap(Logger::default())
        // .service(user::info)
    })
    .bind(("127.0.0.1", 8000))?
    .workers(2)
    .run()
    .await
}
