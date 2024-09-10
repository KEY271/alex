use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{extract::State, Json};
use serde::Deserialize;

use alex::{movegen::is_pseudo_legal, position::Position, search::bestmove};

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

#[derive(Deserialize)]
pub struct Bestmove {
    mfen: String,
    time: f64,
}

pub async fn post_bestmove(Json(bmv): Json<Bestmove>) -> String {
    println!("POST: /api/bestmove; {}, {}s", bmv.mfen, bmv.time);
    let mut position = Position::from_str(&bmv.mfen).unwrap();
    bestmove(&mut position, bmv.time)
}
