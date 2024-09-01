use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{extract::State, Json};
use serde::Deserialize;

use crate::engine::{board::Board, search::bestmove};

pub async fn get_board(State(board): State<Arc<Mutex<Board>>>) -> String {
    println!("GET: /api/board");
    board.lock().unwrap().to_string()
}

#[derive(Deserialize)]
pub struct BoardMfen {
    mfen: String,
}

pub async fn post_board(State(board): State<Arc<Mutex<Board>>>, Json(mfen): Json<BoardMfen>) {
    println!("POST: /api/board; {}", mfen.mfen);
    let mut board = board.lock().unwrap();
    *board = Board::from_str(&mfen.mfen).unwrap();
}

#[derive(Deserialize)]
pub struct MoveMfen {
    mfen: String,
}

pub async fn post_move(State(board): State<Arc<Mutex<Board>>>, Json(m): Json<MoveMfen>) {
    println!("POST: /api/move; {}", m.mfen);
    let mut board = board.lock().unwrap();
    let m = board.read_move(m.mfen).unwrap();
    board.do_move(m);
}

pub async fn post_bestmove(Json(mfen): Json<BoardMfen>) -> String {
    println!("POST: /api/bestmove; {}", mfen.mfen);
    let mut board = Board::from_str(&mfen.mfen).unwrap();
    bestmove(&mut board)
}
