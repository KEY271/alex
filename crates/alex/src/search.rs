use std::mem::MaybeUninit;

use chrono::{DateTime, Local, TimeDelta};

use super::{
    eval::eval,
    movegen::{GenType, MoveList},
    movepick::MovePicker,
    position::Position,
    types::{Move, Value, MAX_PLY, VALUE_INF, VALUE_WIN},
};

struct TimeKeeper {
    start: DateTime<Local>,
    duration: TimeDelta,
}

impl TimeKeeper {
    fn new(duration: f64) -> Self {
        let start = chrono::Local::now();
        let secs = duration.floor() as i64;
        let nanos = ((duration - secs as f64) * 1e9) as u32;
        let duration = TimeDelta::new(secs, nanos).unwrap();

        TimeKeeper { start, duration }
    }

    fn passed(&self) -> bool {
        let now = chrono::Local::now();
        now - self.start > self.duration
    }
}

pub struct SearchInfo {
    pub mv: Move,
    pub depth: usize,
    pub value: Value,
    pub root_moves: Vec<(Move, Value, Vec<Move>)>,
}

pub fn search(position: &mut Position, time: f64) -> Option<SearchInfo> {
    let keeper = TimeKeeper::new(time);
    let mut moves = MoveList::new();
    moves.generate(position, GenType::Legal);
    let mut depth = 1;
    let mut result = Vec::new();
    loop {
        let res = search_root(&moves, position, -VALUE_INF, VALUE_INF, depth, &keeper);
        if keeper.passed() {
            break;
        } else {
            result = res;
        }
        depth += 1;
    }
    // max_by_key returns the last max value, so we need to reverse the iterator.
    if let Some((mv, value, _)) = result.clone().into_iter().rev().max_by_key(|v| v.1) {
        let mut root_moves = Vec::new();
        for (mv, value, line) in result {
            let mut moves = Vec::new();
            for i in 0..line.size {
                moves.push(unsafe { *line.moves.get_unchecked(i).as_ptr() });
            }
            root_moves.push((mv, value, moves));
        }
        Some(SearchInfo {
            mv,
            depth,
            value,
            root_moves,
        })
    } else {
        None
    }
}

const MAX_MOVE: usize = 64;

#[derive(Clone)]
struct Line {
    moves: [MaybeUninit<Move>; MAX_MOVE],
    size: usize,
}

impl Line {
    fn new() -> Self {
        Line {
            moves: unsafe { MaybeUninit::uninit().assume_init() },
            size: 0,
        }
    }
}

fn search_root(
    moves: &MoveList,
    position: &mut Position,
    alpha: Value,
    beta: Value,
    depth: usize,
    keeper: &TimeKeeper,
) -> Vec<(Move, Value, Line)> {
    let mut vec = Vec::new();

    let mut alpha = alpha;

    for i in 0..moves.size {
        if keeper.passed() {
            return vec;
        }
        let mut line = Line::new();
        let mv = moves.at(i).mv;
        position.do_move(mv, None);
        let ev = -search_node(position, -beta, -alpha, depth - 1, keeper, &mut line);
        vec.push((mv, ev, line));
        position.undo_move(mv);
        if ev > alpha {
            alpha = ev;
        }
        if alpha >= beta {
            return vec;
        }
    }

    vec
}

fn search_node(
    position: &mut Position,
    alpha: Value,
    beta: Value,
    depth: usize,
    keeper: &TimeKeeper,
    pline: &mut Line,
) -> Value {
    if keeper.passed() {
        return 0;
    }

    if depth <= 0 {
        pline.size = 0;
        return qsearch(position, alpha, beta, 0, keeper);
    }

    let mut line = Line::new();

    let mut bestvalue = -VALUE_INF;
    let mut alpha = alpha;

    let mut picker = MovePicker::new(position);
    let mut move_count = 0;
    loop {
        let mv = picker.next_move(position);
        if let Some(mv) = mv {
            if !position.is_legal(mv) {
                continue;
            }
            move_count += 1;

            position.do_move(mv, None);
            let ev = -search_node(position, -beta, -alpha, depth - 1, keeper, &mut line);
            position.undo_move(mv);

            if ev > bestvalue {
                bestvalue = ev;
            }
            if ev > alpha {
                alpha = ev;
                if alpha < beta {
                    unsafe {
                        *pline.moves.get_unchecked_mut(0).as_mut_ptr() = mv;
                    }
                    pline.moves[1..line.size + 1].copy_from_slice(&line.moves[..line.size]);
                    pline.size = line.size + 1;
                }
            }
            if alpha >= beta {
                break;
            }
        } else {
            break;
        }
    }

    if move_count == 0 {
        -VALUE_WIN
    } else {
        bestvalue
    }
}

fn qsearch(
    position: &mut Position,
    alpha: Value,
    beta: Value,
    ply: usize,
    keeper: &TimeKeeper,
) -> Value {
    if keeper.passed() || ply >= MAX_PLY {
        return 0;
    }

    // Pruning by an evaluation.
    let stand_pat = eval(position);
    if stand_pat >= beta {
        return stand_pat;
    }
    let mut alpha = alpha;
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut bestvalue = -VALUE_INF;

    let mut picker = MovePicker::qsearch();
    let mut move_count = 0;
    loop {
        let mv = picker.next_move(position);
        if let Some(mv) = mv {
            if !position.is_legal(mv) {
                continue;
            }
            move_count += 1;

            position.do_move(mv, None);
            let ev = -qsearch(position, -beta, -alpha, ply + 1, keeper);
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

    if move_count == 0 {
        if position.checkers() != 0 {
            -VALUE_WIN
        } else {
            stand_pat
        }
    } else {
        bestvalue
    }
}
