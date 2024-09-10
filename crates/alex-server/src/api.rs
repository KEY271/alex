use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use alex::{
    position::Position,
    search::search,
    types::{move_to_mfen, Value},
};

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
    if let Ok(mv) = position.read_move(m.mfen.clone()) {
        if position.is_pseudo_legal(mv) {
            position.do_move(mv, None);
        } else {
            println!("illegal move: {}", m.mfen);
        }
    } else {
        println!("unknown move: {}", m.mfen);
    }
}

#[derive(Deserialize)]
pub struct Go {
    mfen: String,
    time: f64,
}

#[derive(Serialize)]
pub struct Bestmove {
    mfen: String,
    depth: usize,
    value: Value,
}

pub async fn post_bestmove(Json(bmv): Json<Go>) -> Json<Bestmove> {
    println!("POST: /api/bestmove; {}, {}s", bmv.mfen, bmv.time);
    let mut position = Position::from_str(&bmv.mfen).unwrap();
    if let Some(info) = search(&mut position, bmv.time) {
        Json(Bestmove {
            mfen: move_to_mfen(info.mv, position.side),
            depth: info.depth,
            value: info.value,
        })
    } else {
        Json(Bestmove {
            mfen: "resign".to_string(),
            depth: 0,
            value: 0,
        })
    }
}
