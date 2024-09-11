use std::{cmp::max, mem::MaybeUninit};

use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::{
    bitboard::KG_BITBOARD,
    change_bit, foreach_bb,
    position::Position,
    types::{
        bit, get_from, get_move_type, get_pt, get_to, is_demise, make_move_drop, make_move_normal,
        make_move_return, make_move_shoot, make_move_supply, Bitboard, ExtMove, Move, MoveType,
        Piece, PieceType, Side, Square, MOVE_DEMISE, RANK_NB,
    },
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

    pub fn at(&self, index: usize) -> &ExtMove {
        unsafe { &*self.moves.get_unchecked(index).as_ptr() }
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
    /// Pesudo-legal moves without capturing.
    NonCaptures,
    /// Pesudo-legal moves with capturing.
    Captures,
    /// Pesudo-legal moves with and without capturing.
    All,
    /// Pseudo-legal moves to evade.
    Evasion,
    /// Legal moves.
    Legal,
}

impl MoveList {
    /// Generates normal moves.
    fn generate_move_normal(&mut self, position: &Position, target: Bitboard) {
        for pt in PieceType::iter() {
            if pt == PieceType::None {
                continue;
            }
            foreach_bb!(position.pieces_pt_side(pt, position.side), sq, {
                let movable_sq =
                    KG_BITBOARD.movable_sq[pt.into_piece(position.side) as usize][sq as usize];
                foreach_bb!(movable_sq & target, sq2, {
                    self.push(make_move_normal(
                        position.grid[sq2 as usize].split().0,
                        sq,
                        sq2,
                    ));
                });
            });
        }
        foreach_bb!(position.heavy_attacks(position.side) & target, sq, {
            let from = if position.side == Side::Black {
                sq as usize - 16
            } else {
                sq as usize + 16
            };
            self.push(make_move_normal(
                position.grid[sq as usize].pt(),
                Square::from_usize(from).unwrap(),
                sq,
            ));
        });
    }

    /// Generates shoot moves.
    fn generate_move_shoot(&mut self, position: &Position, target: Bitboard) {
        let bb = position.pieces_pt_side(PieceType::Archer1, position.side)
            | position.pieces_pt_side(PieceType::Archer2, position.side);
        foreach_bb!(bb, sq, {
            foreach_bb!(position.arrow_attacks(sq) & target, sq2, {
                self.push(make_move_shoot(
                    position.grid[sq2 as usize].split().0,
                    sq,
                    sq2,
                ));
            });
        });
    }

    /// Generates return moves.
    fn generate_move_return(&mut self, position: &Position) {
        let bb = position.pieces_pt_side(PieceType::Archer0, position.side)
            | position.pieces_pt_side(PieceType::Archer1, position.side);
        foreach_bb!(
            position.pieces_pt_side(PieceType::Arrow, position.side),
            sq,
            {
                foreach_bb!(position.arrow_attacks(sq) & bb, sq2, {
                    self.push(make_move_return(sq, sq2));
                });
            }
        );
    }

    /// Generates drop moves.
    fn generate_move_drop(&mut self, position: &Position, target: Bitboard) {
        let mask = if position.side == Side::Black {
            0x000000FFFFFFFFFF
        } else {
            0xFFFFFFFFFF000000
        };
        let bb = !position.pieces() & mask & target;

        if position.count_hand(position.side, PieceType::Light) != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Light, sq)) });
        }
        if position.count_hand(position.side, PieceType::Heavy) != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Heavy, sq)) });
        }
        if position.count_hand(position.side, PieceType::General) != 0 {
            foreach_bb!(bb, sq, {
                self.push(make_move_drop(PieceType::General, sq))
            });
        }
        if position.count_hand(position.side, PieceType::Knight) != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Knight, sq)) });
        }
        let arrow = position.count_hand(position.side, PieceType::Arrow);
        if arrow != 0 {
            foreach_bb!(bb, sq, { self.push(make_move_drop(PieceType::Arrow, sq)) });
        }
        if position.count_hand(position.side, PieceType::Archer0) != 0 {
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
    fn generate_move_supply(&mut self, position: &Position) {
        if position.count_hand(position.side, PieceType::Arrow) != 0 {
            let bb = position.pieces_pt_side(PieceType::Archer0, position.side)
                | position.pieces_pt_side(PieceType::Archer1, position.side);
            foreach_bb!(bb, sq, {
                self.push(make_move_supply(sq));
            });
        }
    }

    /// Generates moves without capturing.
    fn generate_non_captures(&mut self, position: &Position) {
        let target = !position.pieces();
        self.generate_move_normal(position, target);
        self.generate_move_shoot(position, target);
        self.generate_move_return(position);
        self.generate_move_drop(position, 0xFFFFFFFFFFFFFFFF);
        self.generate_move_supply(position);
    }

    /// Generates moves with capturing.
    fn generate_captures(&mut self, position: &Position) {
        let target = !position.pieces_side(position.side) & position.pieces_side(!position.side);
        self.generate_move_normal(position, target);
        self.generate_move_shoot(position, target);
    }

    /// Generates moves to evade.
    fn generate_evasion(&mut self, position: &Position) {
        // Demise.
        let mut demise_sq = Square::NONE;
        if position.demise[position.side as usize] == 0 {
            demise_sq = position.piece_list[position.side as usize][PieceType::Prince as usize][0];
        } else if position.demise[position.side as usize] == 1 {
            demise_sq = position.piece_list[position.side as usize][PieceType::King as usize][0];
        }
        if demise_sq != Square::NONE && !position.is_attacked(demise_sq, position.side) {
            let start = self.size;
            self.generate(position, GenType::All);
            for s in self.slice_mut(start) {
                s.mv |= MOVE_DEMISE;
            }
        }

        let crown_sq = position.crown_sq(position.side);
        let mut checkers_count = 0;
        let mut attacks = 0;
        let opp_archer: u64 = position.pieces_pt_side(PieceType::Archer1, !position.side)
            | position.pieces_pt_side(PieceType::Archer2, !position.side);
        foreach_bb!(opp_archer, sq, {
            attacks |= position.arrow_attacks(sq);
        });
        let mut checker = Square::NONE;
        let mut slide_check = false;
        foreach_bb!(position.checkers(), c, {
            checkers_count += 1;
            checker = c;
            let pt = position.grid[c as usize].pt();
            if pt == PieceType::Heavy {
                slide_check = true;
                let x = checker as usize % 8;
                let y1 = checker as usize / 8;
                let y2 = crown_sq as usize / 8;
                if y1.abs_diff(y2) == 1 {
                    if position.side == Side::Black && y2 >= 1 {
                        change_bit!(attacks, (y2 - 1) * RANK_NB + x);
                    }
                    if position.side == Side::White && y2 <= 6 {
                        change_bit!(attacks, (y2 + 1) * RANK_NB + x);
                    }
                }
            }
            if pt == PieceType::Archer1 || pt == PieceType::Archer2 {
                slide_check = true;
                attacks |= KG_BITBOARD.line_bb[c as usize][crown_sq as usize];
                attacks |= KG_BITBOARD.movable_sq[position.grid[c as usize] as usize][c as usize];
            }
        });
        let crown_evasion =
            KG_BITBOARD.movable_sq[position.grid[crown_sq as usize] as usize][crown_sq as usize];
        foreach_bb!(
            crown_evasion & !attacks & !position.pieces_side(position.side),
            sq,
            {
                let cap = position.grid[sq as usize].pt();
                self.push(make_move_normal(cap, crown_sq, sq));
            }
        );

        if checkers_count == 1 {
            let target = if slide_check {
                (1 << checker as usize)
                    | KG_BITBOARD.between_bb[checker as usize][crown_sq as usize]
            } else {
                1 << checker as usize
            };
            self.generate_move_normal(position, target);
            self.generate_move_shoot(position, target);
            self.generate_move_drop(position, target);
        }
    }

    fn generate_legal(&mut self, position: &Position) {
        if position.checkers() != 0 {
            self.generate_evasion(position);
        } else {
            self.generate_captures(position);
            self.generate_non_captures(position);
        }
        let mut legal_moves = Vec::new();
        for mv in self.slice(0) {
            if position.is_legal(mv.mv) {
                legal_moves.push(mv.clone());
            }
        }

        self.size = legal_moves.len();

        let moves = self.slice_mut(0);
        for (i, mv) in legal_moves.into_iter().enumerate() {
            moves[i] = mv;
        }
    }

    /// Generates moves.
    pub fn generate(&mut self, position: &Position, gen: GenType) {
        match gen {
            GenType::NonCaptures => self.generate_non_captures(position),
            GenType::Captures => self.generate_captures(position),
            GenType::All => {
                self.generate_captures(position);
                self.generate_non_captures(position);
            }
            GenType::Evasion => self.generate_evasion(position),
            GenType::Legal => self.generate_legal(position),
        }
    }
}

impl Position {
    pub fn is_pseudo_legal(&self, mv: Move) -> bool {
        let typ = get_move_type(mv);
        let to = get_to(mv);
        if is_demise(mv) {
            if self.demise[self.side as usize] >= 2 {
                return false;
            }
            if mv == MOVE_DEMISE {
                return true;
            }
        }
        match typ {
            MoveType::Normal => {
                let (pt, side) = self.grid[to as usize].split();
                if pt != PieceType::None && side == self.side {
                    return false;
                }

                let from = get_from(mv);
                let p = self.grid[from as usize];
                if p == Piece::None || p.side() != self.side {
                    return false;
                }
                if p.pt() == PieceType::Heavy
                    && (from as usize).abs_diff(to as usize) == RANK_NB * 2
                {
                    let mid = (from as usize + to as usize) / 2;
                    if self.grid[mid] != Piece::None {
                        return false;
                    }
                } else if bit(
                    KG_BITBOARD.movable_sq[p as usize][from as usize],
                    to as usize,
                ) != 1
                {
                    return false;
                }
            }
            MoveType::Return => {
                let (pt, side) = self.grid[to as usize].split();
                if !(pt == PieceType::Archer0 || pt == PieceType::Archer1) || side != self.side {
                    return false;
                }

                let from = get_from(mv) as usize;
                let (pt, side) = self.grid[from].split();
                if pt != PieceType::Arrow || side != self.side {
                    return false;
                }

                let x1 = from % RANK_NB;
                let y1 = from / RANK_NB;
                let x2 = to as usize % RANK_NB;
                let y2 = to as usize / RANK_NB;
                let dist = max(x1.abs_diff(x2), y1.abs_diff(y2));
                if x1.abs_diff(x2) != dist && x1 != x2 {
                    return false;
                }
                if y1.abs_diff(y2) != dist && y1 != y2 {
                    return false;
                }
                for i in 1..dist {
                    let x = x1 as isize + (x2 as isize - x1 as isize) / dist as isize * i as isize;
                    let y = y1 as isize + (y2 as isize - y1 as isize) / dist as isize * i as isize;
                    if self.grid[y as usize * RANK_NB + x as usize] != Piece::None {
                        return false;
                    }
                }
            }
            MoveType::Shoot => {
                let (pt, side) = self.grid[to as usize].split();
                if pt != PieceType::None && side == self.side {
                    return false;
                }

                let from = get_from(mv) as usize;
                let (pt, side) = self.grid[from].split();
                if !(pt == PieceType::Archer1 || pt == PieceType::Archer2) || side != self.side {
                    return false;
                }

                let x1 = from % RANK_NB;
                let y1 = from / RANK_NB;
                let x2 = to as usize % RANK_NB;
                let y2 = to as usize / RANK_NB;
                let dist = max(x1.abs_diff(x2), y1.abs_diff(y2));
                if x1.abs_diff(x2) != dist && x1 != x2 {
                    return false;
                }
                if y1.abs_diff(y2) != dist && y1 != y2 {
                    return false;
                }
                for i in 1..dist {
                    let x = x1 as isize + (x2 as isize - x1 as isize) / dist as isize * i as isize;
                    let y = y1 as isize + (y2 as isize - y1 as isize) / dist as isize * i as isize;
                    if self.grid[y as usize * RANK_NB + x as usize] != Piece::None {
                        return false;
                    }
                }
            }
            MoveType::Drop => {
                let pt = get_pt(mv);
                if pt == PieceType::Archer1 {
                    if self.count_hand(self.side, PieceType::Archer0) == 0
                        || self.count_hand(self.side, PieceType::Arrow) == 0
                    {
                        return false;
                    }
                } else if pt == PieceType::Archer2 {
                    if self.count_hand(self.side, PieceType::Archer0) == 0
                        || self.count_hand(self.side, PieceType::Arrow) <= 1
                    {
                        return false;
                    }
                } else if self.count_hand(self.side, pt) == 0 {
                    return false;
                }

                if self.grid[to as usize] != Piece::None {
                    return false;
                }
            }
            MoveType::Supply => {
                if self.count_hand(self.side, PieceType::Arrow) == 0 {
                    return false;
                }

                let (pt, side) = self.grid[to as usize].split();
                if !(pt == PieceType::Archer0 || pt == PieceType::Archer1) || side != self.side {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_legal(&self, mv: Move) -> bool {
        match get_move_type(mv) {
            MoveType::Normal | MoveType::Return => {
                let from = get_from(mv);
                let to = get_to(mv);
                let demise = self.demise[self.side as usize] + if is_demise(mv) { 1 } else { 0 };
                let (blockers, crown_sq) = if demise % 2 == 0 {
                    (
                        self.blockers_king(),
                        self.piece_list[self.side as usize][PieceType::King as usize][0],
                    )
                } else {
                    (
                        self.blockers_prince(),
                        self.piece_list[self.side as usize][PieceType::Prince as usize][0],
                    )
                };
                if from == crown_sq {
                    !self.is_attacked(to, self.side)
                } else if blockers & (1 << from as usize) != 0 {
                    self.aligned(from, to, crown_sq)
                } else {
                    true
                }
            }
            _ => true,
        }
    }
}
