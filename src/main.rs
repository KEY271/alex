use std::str::FromStr;

extern crate num_derive;

extern crate strum;

mod board;
mod movegen;

fn main() {
    let board = board::Board::from_str("bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB").unwrap();
    println!("{}\n", board);

    let mut moves = Vec::new();
    movegen::generate(&board, movegen::GenType::NonCaptures, &mut moves);
    moves.sort();
    for m in &moves {
        println!("{}", movegen::pretty_move(*m));
    }
}
