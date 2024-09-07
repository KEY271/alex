use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{extract::State, Json};
use serde::Deserialize;

use crate::engine::{movegen::is_pseudo_legal, position::Position, search::bestmove};

pub async fn get_board(State(position): State<Arc<Mutex<Position>>>) -> String {
    println!("GET: /api/board");
    position.lock().unwrap().to_string()
}

#[derive(Deserialize)]
pub struct BoardMfen {
    mfen: String,
}

pub async fn post_board(State(position): State<Arc<Mutex<Position>>>, Json(mfen): Json<BoardMfen>) {
    println!("POST: /api/board; {}", mfen.mfen);
    let mut position = position.lock().unwrap();
    *position = Position::from_str(&mfen.mfen).unwrap();
}

#[derive(Deserialize)]
pub struct MoveMfen {
    mfen: String,
}

pub async fn post_move(State(position): State<Arc<Mutex<Position>>>, Json(m): Json<MoveMfen>) {
    println!("POST: /api/move; {}", m.mfen);
    let mut position = position.lock().unwrap();
    let mv = position.read_move(m.mfen).unwrap();
    if is_pseudo_legal(&position, mv) {
        position.do_move(mv, None);
    } else {
        println!("illegal!");
    }
}

pub async fn post_bestmove(Json(mfen): Json<BoardMfen>) -> String {
    println!("POST: /api/bestmove; {}", mfen.mfen);
    let mut position = Position::from_str(&mfen.mfen).unwrap();
    bestmove(&mut position)
}
