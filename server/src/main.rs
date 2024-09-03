use std::{
    env,
    str::FromStr,
    sync::{Arc, Mutex},
};

use api::{get_board, post_bestmove, post_board, post_move};
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue},
    routing::{get, post},
    Router,
};
use engine::position::Position;

use tower_http::{cors::CorsLayer, services::ServeDir};

mod api;
mod engine;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = Router::new()
        .route("/api/board", get(get_board))
        .route("/api/board", post(post_board))
        .route("/api/move", post(post_move))
        .route("/api/bestmove", post(post_bestmove));
    if !args.contains(&"--server-only".to_string()) {
        let static_dir = ServeDir::new("static");
        app = app.nest_service("/", static_dir);
    }
    let position =
        Position::from_str("bngpkgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB b - 0 0").unwrap();
    let state = Arc::new(Mutex::new(position));
    let origins = ["http://127.0.0.1:5173".parse::<HeaderValue>().unwrap()];
    let app = app.with_state(state).layer(
        CorsLayer::new()
            .allow_origin(origins)
            .allow_headers([CONTENT_TYPE]),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Server: http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}
