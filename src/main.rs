use std::str::FromStr;

use board::Board;

extern crate num_derive;

extern crate strum;

mod board;

fn main() {
    let board = Board::from_str("bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB").unwrap();
    println!("{}\n", board);
}
