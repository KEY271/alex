use std::{
    env,
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{extract::State, http::HeaderValue, routing::get, Router};
use engine::board::Board;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod engine;

async fn get_board(State(board): State<Arc<Mutex<Board>>>) -> String {
    println!("GET: /api/board");
    board.lock().unwrap().to_string()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = Router::new().route("/api/board", get(get_board));
    if !args.contains(&"--server-only".to_string()) {
        let static_dir = ServeDir::new("static");
        app = app.nest_service("/", static_dir);
    }
    let board = Board::from_str("bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB").unwrap();
    let state = Arc::new(Mutex::new(board));
    let origins = ["http://127.0.0.1:5173".parse::<HeaderValue>().unwrap()];
    let app = app
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(origins));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Server: http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}
