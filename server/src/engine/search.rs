use super::{
    eval::eval,
    movegen::{GenType, MoveList},
    movepick::MovePicker,
    position::Position,
    util::{move_to_mfen, Move, Value, VALUE_INF},
};

fn search(position: &mut Position) -> Move {
    let mut moves = MoveList::new();
    moves.generate(position, GenType::All);
    let res = search_root(&moves, position, -VALUE_INF, VALUE_INF);
    res.into_iter().max_by_key(|v| v.1).unwrap().0
}

fn search_root(
    moves: &MoveList,
    position: &mut Position,
    alpha: Value,
    beta: Value,
) -> Vec<(Move, Value)> {
    let mut vec = Vec::new();

    let mut alpha = alpha;

    for i in 0..moves.size {
        let mv = moves.at(i).mv;
        position.do_move(mv);
        let ev = -search_node(position, -beta, -alpha, 3);
        vec.push((mv, ev));
        position.undo_move(mv);
        println!("{}:{}", move_to_mfen(mv, position.side), ev);
        if ev > alpha {
            alpha = ev;
        }
        if alpha >= beta {
            return vec;
        }
    }

    vec
}

fn search_node(position: &mut Position, alpha: Value, beta: Value, depth: usize) -> Value {
    if depth <= 0 {
        return eval(position);
    }

    let mut bestvalue = Value::MIN;
    let mut alpha = alpha;

    let mut picker = MovePicker::new();
    loop {
        let mv = picker.next_move(position);
        if let Some(mv) = mv {
            position.do_move(mv);
            let ev = -search_node(position, -beta, -alpha, depth - 1);
            position.undo_move(mv);

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

pub fn bestmove(position: &mut Position) -> String {
    let mv = search(position);
    move_to_mfen(mv, position.side)
}
