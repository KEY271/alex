use super::{
    board::Board,
    movegen::{generate, GenType},
    util::move_to_mfen,
};

pub fn bestmove(board: &mut Board) -> String {
    let mut moves = Vec::new();
    generate(board, GenType::NonCaptures, &mut moves);
    generate(board, GenType::Captures, &mut moves);
    move_to_mfen(moves[0], board.side)
}
