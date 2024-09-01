use super::{
    board::Board,
    eval::eval,
    movegen::{GenType, MoveList},
    movepick::MovePicker,
    util::{move_to_mfen, Move, Value, VALUE_INF},
};

fn search(board: &mut Board) -> Move {
    let mut moves = MoveList::new();
    moves.generate(board, GenType::All);
    let res = search_root(&moves, board, -VALUE_INF, VALUE_INF);
    res.into_iter().max_by_key(|v| v.1).unwrap().0
}

fn search_root(
    moves: &MoveList,
    board: &mut Board,
    alpha: Value,
    beta: Value,
) -> Vec<(Move, Value)> {
    let mut vec = Vec::new();

    let mut alpha = alpha;

    for i in 0..moves.size {
        let mv = moves.at(i).mv;
        board.do_move(mv);
        let ev = -search_node(board, -beta, -alpha, 3);
        vec.push((mv, ev));
        board.undo_move(mv);
        println!("{}:{}", move_to_mfen(mv, board.side), ev);
        if ev > alpha {
            alpha = ev;
        }
        if alpha >= beta {
            return vec;
        }
    }

    vec
}

fn search_node(board: &mut Board, alpha: Value, beta: Value, depth: usize) -> Value {
    if depth <= 0 {
        return eval(board);
    }

    let mut bestvalue = Value::MIN;
    let mut alpha = alpha;

    let mut picker = MovePicker::new();
    loop {
        let mv = picker.next_move(board);
        if let Some(mv) = mv {
            board.do_move(mv);
            let ev = -search_node(board, -beta, -alpha, depth - 1);
            board.undo_move(mv);

            if ev > bestvalue {
                bestvalue = ev;
            }
            if ev > alpha {
                alpha = ev;
            }
            if alpha >= beta {
                break;
            }
        } else {
            break;
        }
    }

    bestvalue
}

pub fn bestmove(board: &mut Board) -> String {
    let mv = search(board);
    move_to_mfen(mv, board.side)
}
