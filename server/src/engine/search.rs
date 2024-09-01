use super::{board::Board, movepick::MovePicker, util::move_to_mfen};

pub fn bestmove(board: &mut Board) -> String {
    let mut picker = MovePicker::new();
    let mv = picker.next_move(board).unwrap();
    move_to_mfen(mv, board.side)
}
