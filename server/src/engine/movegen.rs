use std::mem::MaybeUninit;

use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::{
    engine::util::{
        make_move_drop, make_move_normal, make_move_return, make_move_shoot, make_move_supply,
    },
    foreach_bb,
};

use super::{
    board::Board,
    util::{Bitboard, ExtMove, Move, PieceType, Side, Square},
};

const MAX_MOVE: usize = 520;

pub struct MoveList {
    pub moves: [MaybeUninit<ExtMove>; MAX_MOVE],
    pub size: usize,
}

impl MoveList {
    pub fn new() -> Self {
        unsafe {
            MoveList {
                moves: MaybeUninit::uninit().assume_init(),
                size: 0,
            }
        }
    }

    pub fn push(&mut self, mv: Move) {
        unsafe {
            (*self.moves.get_unchecked_mut(self.size).as_mut_ptr()).mv = mv;
        }
        self.size += 1;
    }

    pub fn at(&self, index: usize) -> ExtMove {
        unsafe { self.moves.get_unchecked(index).assume_init_read() }
    }

    pub fn slice(&self, begin: usize) -> &[ExtMove] {
        unsafe {
            std::slice::from_raw_parts(self.moves.get_unchecked(begin).as_ptr(), self.size - begin)
        }
    }
    pub fn slice_mut(&mut self, begin: usize) -> &mut [ExtMove] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.moves.get_unchecked_mut(begin).as_ptr() as *mut ExtMove,
                self.size - begin,
            )
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GenType {
    /// Moves without capturing.
    NonCaptures,
    /// Moves with capturing.
    Captures,
    /// All moves.
    All,
}

impl MoveList {
    /// Generates normal moves.
    fn generate_move_normal(&mut self, board: &Board, target: Bitboard) {
        for pt in PieceType::iter() {
            if pt == PieceType::None {
                continue;
            }
            foreach_bb!(board.pieces_pt_side(pt, board.side), sq, {
                let movable_sq = board.movable_sq[pt.into_piece(board.side) as usize][sq as usize];
                foreach_bb!(movable_sq & target, sq2, {
                    self.push(make_move_normal(
                        board.grid[sq2 as usize].split().0,
                        sq,
                        sq2,
                    ));
                });
            });
        }
        foreach_bb!(board.heavy_attacks(board.side) & target, sq, {
            let from = if board.side == Side::Black {
                sq as usize - 16
            } else {
                sq as usize + 16
            };
            self.push(make_move_normal(
                board.grid[sq as usize].pt(),
                Square::from_usize(from).unwrap(),
                sq,
            ));
        });
    }

    /// Generates shoot moves.
    fn generate_move_shoot(&mut self, board: &Board, target: Bitboard) {
        let bb = board.pieces_pt_side(PieceType::Archer1, board.side)
            | board.pieces_pt_side(PieceType::Archer2, board.side);
        foreach_bb!(bb, sq, {
            foreach_bb!(board.arrow_attacks(sq) & target, sq2, {
                self.push(make_move_shoot(board.grid[sq2 as usize].split().0, sq, sq2));
            });
        });
    }

    /// Generates return moves.
    fn generate_move_return(&mut self, board: &Board) {
        let bb = board.pieces_pt_side(PieceType::Archer0, board.side)
            | board.pieces_pt_side(PieceType::Archer1, board.side);
        foreach_bb!(board.pieces_pt_side(PieceType::Arrow, board.side), sq, {
            foreach_bb!(board.arrow_attacks(sq) & bb, sq2, {
                self.push(make_move_return(sq, sq2));
            });
        });
    }

    /// Generates drop moves.
    fn generate_move_drop(&mut self, board: &Board) {
        let mask = if board.side == Side::Black {
            0x000000FFFFFFFFFF
        } else {
            0xFFFFFFFFFF000000
        };
        let bb = !board.pieces() & mask;

        if board.count_hand(board.side, PieceType::Light) != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Light, sq)) });
        }
        if board.count_hand(board.side, PieceType::Heavy) != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Heavy, sq)) });
        }
        if board.count_hand(board.side, PieceType::General) != 0 {
            foreach_bb!(bb, sq, {
                self.push(make_move_drop(PieceType::General, sq))
            });
        }
        if board.count_hand(board.side, PieceType::Knight) != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Knight, sq)) });
        }
        let arrow = board.count_hand(board.side, PieceType::Arrow);
        if arrow != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Arrow, sq)) });
        }
        if board.count_hand(board.side, PieceType::Archer0) != 0 {
            foreach_bb!(bb, sq, {
                self.push(make_move_drop(PieceType::Archer0, sq));
                if arrow >= 1 {
                    self.push(make_move_drop(PieceType::Archer1, sq));
                }
                if arrow >= 2 {
                    self.push(make_move_drop(PieceType::Archer2, sq));
                }
            });
        }
    }

    /// Generates supply.
    fn generate_move_supply(&mut self, board: &Board) {
        if board.count_hand(board.side, PieceType::Arrow) != 0 {
            let bb = board.pieces_pt_side(PieceType::Archer0, board.side)
                | board.pieces_pt_side(PieceType::Archer1, board.side);
            foreach_bb!(bb, sq, {
                self.push(make_move_supply(sq));
            });
        }
    }

    /// Generates moves without capturing.
    fn generate_non_captures(&mut self, board: &Board) {
        let target = !board.pieces();
        self.generate_move_normal(board, target);
        self.generate_move_shoot(board, target);
        self.generate_move_return(board);
        self.generate_move_drop(board);
        self.generate_move_supply(board);
    }

    /// Generates moves with capturing.
    fn generate_captures(&mut self, board: &Board) {
        let target = !board.pieces_side(board.side) & board.pieces_side(!board.side);
        self.generate_move_normal(board, target);
        self.generate_move_shoot(board, target);
    }

    /// Generates moves.
    pub fn generate(&mut self, board: &Board, gen: GenType) {
        match gen {
            GenType::NonCaptures => self.generate_non_captures(board),
            GenType::Captures => self.generate_captures(board),
            GenType::All => {
                self.generate_captures(board);
                self.generate_non_captures(board);
            }
        }
    }
}
