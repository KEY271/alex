use std::{
    env,
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderValue},
    routing::{get, post},
    Json, Router,
};
use engine::board::Board;
use serde::Deserialize;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod engine;

async fn get_board(State(board): State<Arc<Mutex<Board>>>) -> String {
    println!("GET: /api/board");
    board.lock().unwrap().to_string()
}

#[derive(Deserialize)]
struct BoardMfen {
    mfen: String,
}

async fn post_board(State(board): State<Arc<Mutex<Board>>>, Json(mfen): Json<BoardMfen>) {
    println!("POST: /api/board; {}", mfen.mfen);
    let mut board = board.lock().unwrap();
    *board = Board::from_str(&mfen.mfen).unwrap();
}

#[derive(Deserialize)]
struct Move {
    mfen: String,
}

async fn post_move(State(board): State<Arc<Mutex<Board>>>, Json(m): Json<Move>) {
    println!("POST: /api/move; {}", m.mfen);
    let mut board = board.lock().unwrap();
    let m = board.read_move(m.mfen).unwrap();
    board.do_move(m);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut app = Router::new()
        .route("/api/board", get(get_board))
        .route("/api/board", post(post_board))
        .route("/api/move", post(post_move));
    if !args.contains(&"--server-only".to_string()) {
        let static_dir = ServeDir::new("static");
        app = app.nest_service("/", static_dir);
    }
    let board = Board::from_str("bngpkgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB b -").unwrap();
    let state = Arc::new(Mutex::new(board));
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
