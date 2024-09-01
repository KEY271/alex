use super::{board::Board, eval::eval, movepick::MovePicker, util::move_to_mfen};

pub fn bestmove(board: &mut Board) -> String {
    let mut picker = MovePicker::new();
    let mut bestmove = None;
    let mut besteval = 0;
    loop {
        let mv = picker.next_move(board);
        if let Some(mv) = mv {
            board.do_move(mv);
            let ev = -eval(board);
            board.undo_move(mv);
            if bestmove.is_some() {
                if besteval < ev {
                    bestmove = Some(mv);
                    besteval = ev;
                }
            } else {
                bestmove = Some(mv);
                besteval = ev;
            }
        } else {
            break;
        }
    }
    move_to_mfen(bestmove.unwrap(), board.side)
}
