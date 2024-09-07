use crate::engine::movegen::GenType;

use super::{
    movegen::MoveList,
    position::Position,
    util::{get_capture, ExtMove, Move, PIECE_TYPE_NB},
};

enum Stage {
    CapturesInit,
    Captures,
    NonCapturesInit,
    NonCaptures,
    EvasionInit,
    Evasion,
}

pub struct MovePicker {
    cur: usize,
    moves: MoveList,
    stage: Stage,
}

const PIECE_VALUES: [i32; PIECE_TYPE_NB] = [0, 100, 200, 800, 600, 400, 400, 400, 400, 800, 1200];

fn score_captures(moves: &mut [ExtMove]) {
    for ext_move in moves {
        let mv = ext_move.mv;
        let cap = get_capture(mv);
        ext_move.score = PIECE_VALUES[cap as usize];
    }
}

fn score_noncaptures(moves: &mut [ExtMove]) {
    for ext_move in moves {
        ext_move.score = 0;
    }
}

fn pick_best(moves: &mut [ExtMove]) -> Move {
    let (max_index, _) = moves
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.cmp(y.1))
        .unwrap();
    moves.swap(0, max_index);
    moves[0].mv
}

fn select_best(moves: &mut [ExtMove], cur: &mut usize) -> Option<Move> {
    let len = moves.len();
    if *cur < len {
        let mv = pick_best(&mut moves[*cur..len]);
        *cur += 1;
        Some(mv)
    } else {
        None
    }
}

impl MovePicker {
    pub fn new(position: &Position) -> Self {
        MovePicker {
            cur: 0,
            moves: MoveList::new(),
            stage: if position.checkers() != 0 {
                Stage::EvasionInit
            } else {
                Stage::CapturesInit
            },
        }
    }

    pub fn next_move(&mut self, position: &Position) -> Option<Move> {
        loop {
            match self.stage {
                Stage::CapturesInit => {
                    self.moves.generate(position, GenType::Captures);
                    score_captures(self.moves.slice_mut(0));
                    self.stage = Stage::Captures;
                }
                Stage::Captures => {
                    if let Some(mv) = select_best(self.moves.slice_mut(0), &mut self.cur) {
                        return Some(mv);
                    }
                    self.cur = 0;
                    self.stage = Stage::NonCapturesInit;
                }
                Stage::NonCapturesInit => {
                    self.moves.size = 0;
                    self.moves.generate(position, GenType::NonCaptures);
                    score_noncaptures(self.moves.slice_mut(0));
                    self.stage = Stage::NonCaptures;
                }
                Stage::NonCaptures => {
                    if self.cur < self.moves.size {
                        let mv = self.moves.at(self.cur).mv;
                        self.cur += 1;
                        return Some(mv);
                    }
                    break;
                }
                Stage::EvasionInit => {
                    self.moves.generate(position, GenType::Evasion);
                    self.stage = Stage::Evasion;
                }
                Stage::Evasion => {
                    if self.cur < self.moves.size {
                        let mv = self.moves.at(self.cur).mv;
                        self.cur += 1;
                        return Some(mv);
                    }
                    break;
                }
            }
        }
        None
    }
}
