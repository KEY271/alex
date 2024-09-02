use core::fmt;
use std::{str::FromStr, usize};

use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::{
    change_bit,
    engine::util::{flipped, get_capture, get_from, get_pt, PieceType},
    for_pos, foreach_bb,
};

use super::util::{
    count_hand, get_move_type, get_to, is_demise, make_move_drop, make_move_normal,
    make_move_return, make_move_shoot, make_move_supply, read_file, read_rank, to_hand, Bitboard,
    Hand, Move, MoveType, Piece, Side, Square, MOVE_DEMISE, PIECE_NB, PIECE_TYPE_NB, RANK_NB,
    SIDE_NB, SQUARE_NB,
};

/// Count of occupation.
pub const OCC_NB: usize = 64;

/// Board.
#[derive(PartialEq, Eq, Clone)]
pub struct Board {
    pub side: Side,
    /// Piece at the square.
    pub grid: [Piece; SQUARE_NB],
    /// Bitboards of the piece type.
    boards: [Bitboard; PIECE_TYPE_NB],
    /// Bitboard of occupied squares of sides.
    sides: [Bitboard; SIDE_NB],
    /// Hands of sides.
    hands: [Hand; SIDE_NB],
    /// Count of demise.
    pub demise: [usize; SIDE_NB],
    /// Effect.
    pub effects: [[usize; SQUARE_NB]; SIDE_NB],
    /// Count of piece.
    pub pieces: [[usize; PIECE_TYPE_NB]; SIDE_NB],
    /// Square of king.
    pub king_sq: [Option<Square>; SIDE_NB],
    /// Square of prince.
    pub prince_sq: [Option<Square>; SIDE_NB],

    /// Squares the piece can move to.
    pub movable_sq: [[Bitboard; SQUARE_NB]; PIECE_NB],
    // Kindergarden bitboard.
    diagonal_mask: [Bitboard; SQUARE_NB],
    anti_diagonal_mask: [Bitboard; SQUARE_NB],
    rank_mask: [Bitboard; SQUARE_NB],
    fill_up_attacks: [[Bitboard; RANK_NB]; OCC_NB],
    a_file_attacks: [[Bitboard; RANK_NB]; OCC_NB],
}

impl Board {
    /// Create an empty board.
    pub fn new() -> Board {
        let mut movable_sq = [[0; SQUARE_NB]; PIECE_NB];
        for pt in PieceType::iter() {
            if pt == PieceType::None {
                continue;
            }
            for_pos!(ix, iy, i, {
                let mut bb = 0;
                for_pos!(jx, jy, j, {
                    match pt {
                        PieceType::None => continue,
                        PieceType::Light | PieceType::Heavy => {
                            if ix == jx && iy + 1 == jy
                                || iy >= 5 && ix.abs_diff(jx) == 1 && iy == jy
                            {
                                change_bit!(bb, j);
                            }
                        }
                        PieceType::King => {
                            if ix.abs_diff(jx) <= 1
                                && iy.abs_diff(jy) <= 1
                                && !(ix == jx && iy == jy)
                            {
                                change_bit!(bb, j);
                            }
                        }
                        PieceType::Prince => {
                            if ix == jx && iy + 1 == jy
                                || ix.abs_diff(jx) == 1 && iy.abs_diff(jy) == 1
                            {
                                change_bit!(bb, j);
                            }
                        }
                        PieceType::General => {
                            if ix.abs_diff(jx) + iy.abs_diff(jy) == 1
                                || ix.abs_diff(jx) == 1 && iy + 1 == jy
                            {
                                change_bit!(bb, j);
                            }
                        }
                        PieceType::Knight => {
                            if ix.abs_diff(jx) + iy.abs_diff(jy) == 3 && ix != jx && iy != jy {
                                change_bit!(bb, j);
                            }
                        }
                        PieceType::Arrow => continue,
                        PieceType::Archer0 | PieceType::Archer1 | PieceType::Archer2 => {
                            if ix.abs_diff(jx) + iy.abs_diff(jy) == 1 {
                                change_bit!(bb, j);
                            }
                        }
                    }
                });
                movable_sq[pt.into_piece(Side::Black) as usize][i] = bb;
                movable_sq[pt.into_piece(Side::White) as usize]
                    [(RANK_NB - 1 - iy) * RANK_NB + ix] = flipped(bb);
            });
        }

        let mut diagonal_mask = [0; SQUARE_NB];
        let mut anti_diagonal_mask = [0; SQUARE_NB];
        let mut rank_mask = [0; SQUARE_NB];
        for_pos!(ix, iy, i, {
            for_pos!(jx, jy, j, {
                if ix + jy == iy + jx {
                    change_bit!(diagonal_mask[i], j);
                }
                if ix + iy == jx + jy {
                    change_bit!(anti_diagonal_mask[i], j);
                }
                if iy == jy {
                    change_bit!(rank_mask[i], j);
                }
            });
        });

        let mut fill_up_attacks = [[0; RANK_NB]; OCC_NB];
        for file in 0..RANK_NB {
            for occ in 0..OCC_NB {
                let mut u = 0;
                // Check left of the square.
                if file > 0 {
                    for i in (0..file).rev() {
                        u |= 1 << i;
                        if (occ << 1) & (1 << i) != 0 {
                            break;
                        }
                    }
                }
                // Check right of the square.
                for i in file + 1..RANK_NB {
                    u |= 1 << i;
                    if (occ << 1) & (1 << i) != 0 {
                        break;
                    }
                }
                // Fill up.
                u |= u << 8;
                u |= u << 16;
                u |= u << 32;
                fill_up_attacks[occ][file] = u;
            }
        }
        let mut a_file_attacks = [[0; RANK_NB]; OCC_NB];
        for rank in 0..RANK_NB {
            for occ in 0..OCC_NB {
                let mut u = 0;
                // Check below the square.
                if rank > 0 {
                    for i in (0..rank).rev() {
                        u |= 1 << (i * RANK_NB);
                        if (occ << 1) & (1 << i) != 0 {
                            break;
                        }
                    }
                }
                // Check above the square.
                for i in rank + 1..RANK_NB {
                    u |= 1 << (i * RANK_NB);
                    if (occ << 1) & (1 << i) != 0 {
                        break;
                    }
                }
                a_file_attacks[occ][rank] = u;
            }
        }
        Board {
            side: Side::Black,
            movable_sq,
            boards: [0; PIECE_TYPE_NB],
            sides: [0, 0],
            hands: [0, 0],
            demise: [0, 0],
            effects: [[0; SQUARE_NB]; SIDE_NB],
            pieces: [[0; PIECE_TYPE_NB]; SIDE_NB],
            king_sq: [None, None],
            prince_sq: [None, None],
            diagonal_mask,
            anti_diagonal_mask,
            rank_mask,
            fill_up_attacks,
            a_file_attacks,
            grid: [Piece::None; SQUARE_NB],
        }
    }

    pub fn pieces(&self) -> Bitboard {
        self.sides[Side::Black as usize] | self.sides[Side::White as usize]
    }

    #[allow(dead_code)]
    pub fn pieces_pt(&self, pt: PieceType) -> Bitboard {
        self.boards[pt as usize]
    }

    pub fn pieces_side(&self, side: Side) -> Bitboard {
        self.sides[side as usize]
    }

    pub fn pieces_pt_side(&self, pt: PieceType, side: Side) -> Bitboard {
        self.boards[pt as usize] & self.sides[side as usize]
    }

    pub fn diagonal_attacks(&self, occ: u64, sq: Square) -> Bitboard {
        let bfile = 0x0202020202020202;
        let occ = (self.diagonal_mask[sq as usize] & occ).wrapping_mul(bfile) >> 58;
        self.diagonal_mask[sq as usize] & self.fill_up_attacks[occ as usize][sq as usize & 7]
    }

    pub fn anti_diagonal_attacks(&self, occ: u64, sq: Square) -> Bitboard {
        let bfile = 0x0202020202020202;
        let occ = (self.anti_diagonal_mask[sq as usize] & occ).wrapping_mul(bfile) >> 58;
        self.anti_diagonal_mask[sq as usize] & self.fill_up_attacks[occ as usize][sq as usize & 7]
    }

    pub fn rank_attacks(&self, occ: u64, sq: Square) -> Bitboard {
        let bfile = 0x0202020202020202;
        let occ = (self.rank_mask[sq as usize] & occ).wrapping_mul(bfile) >> 58;
        self.rank_mask[sq as usize] & self.fill_up_attacks[occ as usize][sq as usize & 7]
    }

    pub fn file_attacks(&self, occ: u64, sq: Square) -> Bitboard {
        let afile = 0x0101010101010101;
        let diagonal_a2_h7 = 0x0004081020408000;
        let occ = afile & (occ >> (sq as usize & 7));
        let occ = occ.wrapping_mul(diagonal_a2_h7) >> 58;
        self.a_file_attacks[occ as usize][sq as usize >> 3] << (sq as usize & 7)
    }

    pub fn heavy_attacks(&self, side: Side) -> Bitboard {
        if side == Side::Black {
            let board = self.pieces_pt_side(PieceType::Heavy, Side::Black) << 8;
            let board = board & !self.pieces();
            board << 8
        } else {
            let board = self.pieces_pt_side(PieceType::Heavy, Side::White) >> 8;
            let board = board & !self.pieces();
            board >> 8
        }
    }

    pub fn arrow_attacks(&self, sq: Square) -> Bitboard {
        let occupied = self.pieces();
        self.file_attacks(occupied, sq)
            | self.rank_attacks(occupied, sq)
            | self.diagonal_attacks(occupied, sq)
            | self.anti_diagonal_attacks(occupied, sq)
    }

    fn calculate_effects(&mut self) {
        for i in 0..SQUARE_NB {
            self.add_effect(Square::from_usize(i).unwrap(), self.grid[i]);
        }
    }

    fn add_effect(&mut self, sq: Square, p: Piece) {
        foreach_bb!(self.movable_sq[p as usize][sq as usize], sq2, {
            self.effects[p.side() as usize][sq2 as usize] += 1;
        });
    }

    fn remove_effect(&mut self, sq: Square, p: Piece) {
        foreach_bb!(self.movable_sq[p as usize][sq as usize], sq2, {
            self.effects[p.side() as usize][sq2 as usize] -= 1;
        });
    }

    pub fn calculate_arrow_effect(&self, side: Side) -> [usize; SQUARE_NB] {
        let mut effect = [0; SQUARE_NB];
        let arrow: u64 = self.pieces_pt_side(PieceType::Archer1, side)
            | self.pieces_pt_side(PieceType::Archer2, side);
        foreach_bb!(arrow, sq, {
            foreach_bb!(self.arrow_attacks(sq), sq2, {
                effect[sq2 as usize] += 1;
            });
        });
        effect
    }

    pub fn count_hand(&self, side: Side, pt: PieceType) -> u32 {
        count_hand(self.hands[side as usize], pt)
    }

    pub fn add_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] += to_hand(pt);
    }

    pub fn remove_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] -= to_hand(pt);
    }

    pub fn do_move(&mut self, m: Move) {
        if is_demise(m) {
            self.demise[self.side as usize] += 1;
            if m == MOVE_DEMISE {
                return;
            }
        }
        let to = get_to(m);
        match get_move_type(m) {
            MoveType::Normal => {
                let from = get_from(m);
                let p = self.grid[from as usize];
                let (pt, side) = p.split();
                let cap = self.grid[to as usize];
                let cap_typ = cap.pt();

                // capture
                if cap_typ != PieceType::None {
                    change_bit!(self.boards[cap_typ as usize], to as usize);
                    change_bit!(self.sides[cap.side() as usize], to as usize);
                    self.remove_effect(to, cap);
                    self.add_hand(side, cap_typ);
                    self.pieces[!side as usize][cap_typ as usize] -= 1;
                    if cap_typ == PieceType::King {
                        self.king_sq[!side as usize] = None;
                    }
                    if cap_typ == PieceType::Prince {
                        self.prince_sq[!side as usize] = None;
                    }
                }

                // to
                change_bit!(self.boards[pt as usize], to as usize);
                change_bit!(self.sides[side as usize], to as usize);
                self.grid[to as usize] = p;
                self.add_effect(to, p);
                if pt == PieceType::King {
                    self.king_sq[self.side as usize] = Some(to);
                }
                if pt == PieceType::Prince {
                    self.prince_sq[self.side as usize] = Some(to);
                }

                // from
                change_bit!(self.boards[pt as usize], from as usize);
                change_bit!(self.sides[side as usize], from as usize);
                self.grid[from as usize] = Piece::None;
                self.remove_effect(from, p);
            }
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                // arrow
                change_bit!(self.boards[PieceType::Arrow as usize], from as usize);
                change_bit!(self.sides[side as usize], from as usize);
                self.grid[from as usize] = Piece::None;
                self.pieces[side as usize][PieceType::Arrow as usize] -= 1;

                // archer
                if pt == PieceType::Archer0 {
                    change_bit!(self.boards[PieceType::Archer0 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer1.into_piece(side);
                    self.pieces[side as usize][PieceType::Archer0 as usize] -= 1;
                    self.pieces[side as usize][PieceType::Archer1 as usize] += 1;
                } else if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer2 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer2.into_piece(side);
                    self.pieces[side as usize][PieceType::Archer1 as usize] -= 1;
                    self.pieces[side as usize][PieceType::Archer2 as usize] += 1;
                }
            }
            MoveType::Shoot => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let cap = self.grid[to as usize];
                let cap_typ = cap.pt();

                // capture
                if cap_typ != PieceType::None {
                    change_bit!(self.boards[cap_typ as usize], to as usize);
                    change_bit!(self.sides[cap.side() as usize], to as usize);
                    self.remove_effect(to, cap);
                    self.add_hand(side, cap_typ);
                    self.pieces[!side as usize][cap_typ as usize] -= 1;
                    if cap_typ == PieceType::King {
                        self.king_sq[!side as usize] = None;
                    }
                    if cap_typ == PieceType::Prince {
                        self.prince_sq[!side as usize] = None;
                    }
                }

                // archer
                if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], from as usize);
                    change_bit!(self.boards[PieceType::Archer0 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer0.into_piece(side);
                    self.pieces[side as usize][PieceType::Archer1 as usize] -= 1;
                    self.pieces[side as usize][PieceType::Archer0 as usize] += 1;
                } else if pt == PieceType::Archer2 {
                    change_bit!(self.boards[PieceType::Archer2 as usize], from as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer1.into_piece(side);
                    self.pieces[side as usize][PieceType::Archer2 as usize] -= 1;
                    self.pieces[side as usize][PieceType::Archer1 as usize] += 1;
                }

                // arrow
                change_bit!(self.sides[side as usize], to as usize);
                change_bit!(self.boards[PieceType::Arrow as usize], to as usize);
                self.grid[to as usize] = PieceType::Arrow.into_piece(side);
                self.pieces[side as usize][PieceType::Arrow as usize] += 1;
            }
            MoveType::Drop => {
                let pt = get_pt(m);
                self.remove_hand(self.side, pt);
                change_bit!(self.sides[self.side as usize], to as usize);
                change_bit!(self.boards[pt as usize], to as usize);
                self.grid[to as usize] = pt.into_piece(self.side);
                self.add_effect(to, self.grid[to as usize]);
                self.pieces[self.side as usize][pt as usize] += 1;
            }
            MoveType::Supply => {
                self.remove_hand(self.side, PieceType::Arrow);
                let pt = self.grid[to as usize].pt();
                if pt == PieceType::Archer0 {
                    change_bit!(self.boards[PieceType::Archer0 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer1.into_piece(self.side);
                    self.pieces[self.side as usize][PieceType::Archer0 as usize] -= 1;
                    self.pieces[self.side as usize][PieceType::Archer1 as usize] += 1;
                } else if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer2 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer2.into_piece(self.side);
                    self.pieces[self.side as usize][PieceType::Archer1 as usize] -= 1;
                    self.pieces[self.side as usize][PieceType::Archer2 as usize] += 1;
                }
            }
        }

        self.side = !self.side;
    }

    pub fn undo_move(&mut self, m: Move) {
        if is_demise(m) {
            self.demise[self.side as usize] -= 1;
            if m == MOVE_DEMISE {
                return;
            }
        }
        // Change side in advance.
        self.side = !self.side;

        let to = get_to(m);
        let side = self.side;
        match get_move_type(m) {
            MoveType::Normal => {
                let p = self.grid[to as usize];
                let pt = p.pt();
                let from = get_from(m);
                let cap = get_capture(m);

                // capture
                self.grid[to as usize] = Piece::None;
                if cap != PieceType::None {
                    change_bit!(self.boards[cap as usize], to as usize);
                    change_bit!(self.sides[!side as usize], to as usize);
                    let cap_piece = cap.into_piece(!side);
                    self.grid[to as usize] = cap_piece;
                    self.add_effect(to, cap_piece);
                    self.remove_hand(side, cap);
                    self.pieces[!side as usize][cap as usize] += 1;
                    if cap == PieceType::King {
                        self.king_sq[!side as usize] = Some(to);
                    }
                    if cap == PieceType::Prince {
                        self.prince_sq[!side as usize] = Some(to);
                    }
                }

                // to
                change_bit!(self.boards[pt as usize], to as usize);
                change_bit!(self.sides[side as usize], to as usize);
                self.remove_effect(to, p);

                // from
                change_bit!(self.boards[pt as usize], from as usize);
                change_bit!(self.sides[side as usize], from as usize);
                self.grid[from as usize] = p;
                self.add_effect(from, p);
                if pt == PieceType::King {
                    self.king_sq[side as usize] = Some(from);
                }
                if pt == PieceType::Prince {
                    self.prince_sq[side as usize] = Some(from);
                }
            }
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                // arrow
                change_bit!(self.boards[PieceType::Arrow as usize], from as usize);
                change_bit!(self.sides[side as usize], from as usize);
                self.grid[from as usize] = PieceType::Arrow.into_piece(side);
                self.pieces[side as usize][PieceType::Arrow as usize] += 1;

                // archer
                if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer0 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    self.pieces[side as usize][PieceType::Archer0 as usize] += 1;
                    self.pieces[side as usize][PieceType::Archer1 as usize] -= 1;
                    self.grid[to as usize] = PieceType::Archer0.into_piece(side);
                } else if pt == PieceType::Archer2 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer2 as usize], to as usize);
                    self.pieces[side as usize][PieceType::Archer1 as usize] += 1;
                    self.pieces[side as usize][PieceType::Archer2 as usize] -= 1;
                    self.grid[to as usize] = PieceType::Archer1.into_piece(side);
                }
            }
            MoveType::Shoot => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let cap = get_capture(m);

                // capture
                self.grid[to as usize] = Piece::None;
                if cap != PieceType::None {
                    change_bit!(self.boards[cap as usize], to as usize);
                    change_bit!(self.sides[!side as usize], to as usize);
                    let cap_piece = cap.into_piece(!side);
                    self.grid[to as usize] = cap_piece;
                    self.add_effect(to, cap_piece);
                    self.remove_hand(side, cap);
                    self.pieces[!side as usize][cap as usize] += 1;
                    if cap == PieceType::King {
                        self.king_sq[!side as usize] = Some(to);
                    }
                    if cap == PieceType::Prince {
                        self.prince_sq[!side as usize] = Some(to);
                    }
                }

                // archer
                if pt == PieceType::Archer0 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], from as usize);
                    change_bit!(self.boards[PieceType::Archer0 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer1.into_piece(side);
                    self.pieces[side as usize][PieceType::Archer1 as usize] += 1;
                    self.pieces[side as usize][PieceType::Archer0 as usize] -= 1;
                } else if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer2 as usize], from as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer2.into_piece(side);
                    self.pieces[side as usize][PieceType::Archer2 as usize] += 1;
                    self.pieces[side as usize][PieceType::Archer1 as usize] -= 1;
                }

                // arrow
                change_bit!(self.sides[side as usize], to as usize);
                change_bit!(self.boards[PieceType::Arrow as usize], to as usize);
                self.pieces[side as usize][PieceType::Arrow as usize] -= 1;
            }
            MoveType::Drop => {
                let pt = get_pt(m);
                self.add_hand(side, pt);
                change_bit!(self.sides[side as usize], to as usize);
                change_bit!(self.boards[pt as usize], to as usize);
                self.remove_effect(to, self.grid[to as usize]);
                self.grid[to as usize] = Piece::None;
                self.pieces[side as usize][pt as usize] -= 1;
            }
            MoveType::Supply => {
                self.add_hand(side, PieceType::Arrow);
                let pt = self.grid[to as usize].pt();
                if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer0 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer0.into_piece(self.side);
                    self.pieces[side as usize][PieceType::Archer0 as usize] += 1;
                    self.pieces[side as usize][PieceType::Archer1 as usize] -= 1;
                } else if pt == PieceType::Archer2 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer2 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer1.into_piece(self.side);
                    self.pieces[side as usize][PieceType::Archer1 as usize] += 1;
                    self.pieces[side as usize][PieceType::Archer2 as usize] -= 1;
                }
            }
        }
    }

    /// Make a move from mfen.
    pub fn read_move(&self, mfen: String) -> Result<Move, String> {
        if mfen == "D" {
            return Ok(MOVE_DEMISE);
        }
        let mfen = mfen.as_bytes();
        if mfen.len() == 4 || mfen.len() == 5 {
            let x1 = read_file(mfen[0])?;
            let y1 = read_rank(mfen[1])?;
            let from = Square::from_usize(y1 * RANK_NB + x1).unwrap();
            let x2 = read_file(mfen[2])?;
            let y2 = read_rank(mfen[3])?;
            let to = Square::from_usize(y2 * RANK_NB + x2).unwrap();
            let cap = self.grid[to as usize];
            if mfen.len() == 5 {
                if mfen[4] == b'S' {
                    Ok(make_move_shoot(cap.pt(), from, to))
                } else {
                    Err("Invalid end character.".to_string())
                }
            } else {
                if cap.pt() != PieceType::None && cap.side() == self.side {
                    Ok(make_move_return(from, to))
                } else {
                    Ok(make_move_normal(cap.pt(), from, to))
                }
            }
        } else if mfen.len() == 3 {
            let x = read_file(mfen[0])?;
            let y = read_rank(mfen[1])?;
            let to = Square::from_usize(y * RANK_NB + x).unwrap();
            let pt = PieceType::from_char(mfen[2]);
            let to_pt = self.grid[to as usize].pt();
            if to_pt == PieceType::Archer0 || to_pt == PieceType::Archer1 {
                Ok(make_move_supply(to))
            } else {
                Ok(make_move_drop(pt, to))
            }
        } else {
            Err("Invalid length.".to_string())
        }
    }
}

impl Piece {
    fn from_char(c: char) -> Result<Self, String> {
        let p = match c {
            'L' => Piece::BLight,
            'H' => Piece::BHeavy,
            'K' => Piece::BKing,
            'P' => Piece::BPrince,
            'G' => Piece::BGeneral,
            'N' => Piece::BKnight,
            'R' => Piece::BArrow,
            'A' => Piece::BArcher0,
            'B' => Piece::BArcher1,
            'C' => Piece::BArcher2,
            'l' => Piece::WLight,
            'h' => Piece::WHeavy,
            'k' => Piece::WKing,
            'p' => Piece::WPrince,
            'g' => Piece::WGeneral,
            'n' => Piece::WKnight,
            'r' => Piece::WArrow,
            'a' => Piece::WArcher0,
            'b' => Piece::WArcher1,
            'c' => Piece::WArcher2,
            _ => return Err(format!("invalid char: {}.", c)),
        };
        Ok(p)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for iy in (0..RANK_NB).rev() {
            let mut ix = 0;
            while ix < RANK_NB {
                let piece = self.grid[iy * RANK_NB + ix];
                if piece == Piece::None {
                    let x = ix;
                    ix += 1;
                    while ix < RANK_NB {
                        if self.grid[iy * RANK_NB + ix] == Piece::None {
                            ix += 1;
                        } else {
                            break;
                        }
                    }
                    write!(f, "{}", ix - x)?;
                } else {
                    ix += 1;
                    write!(f, "{}", piece)?;
                }
            }
            if iy > 0 {
                write!(f, "/")?;
            }
        }

        write!(f, " {} ", if self.side == Side::Black { 'b' } else { 'w' })?;

        if self.hands[0] == 0 && self.hands[1] == 0 {
            write!(f, "-")?;
        } else {
            let pts = [
                PieceType::Light,
                PieceType::Heavy,
                PieceType::General,
                PieceType::Knight,
                PieceType::Arrow,
                PieceType::Archer0,
            ];
            for pt in pts {
                let count = self.count_hand(Side::Black, pt);
                let piece = pt.into_piece(Side::Black);
                if count > 0 {
                    write!(f, "{}", piece)?;
                }
                if count > 1 {
                    write!(f, "{}", count)?;
                }
            }
            for pt in pts {
                let count = self.count_hand(Side::White, pt);
                let piece = pt.into_piece(Side::White);
                if count > 0 {
                    write!(f, "{}", piece)?;
                }
                if count > 1 {
                    write!(f, "{}", count)?;
                }
            }
        }

        write!(f, " {}", self.demise[0])?;
        write!(f, " {}", self.demise[1])?;
        Ok(())
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut ix = 0;
        let mut iy = RANK_NB - 1;
        let s: Vec<&str> = s.split(" ").collect();
        if s.len() != 5 {
            return Err("invalid mfen.".to_string());
        }
        for c in s[0].chars() {
            let piece = match c {
                '/' => {
                    if ix != RANK_NB {
                        return Err("invalid row.".to_string());
                    }
                    ix = 0;
                    if iy == 0 {
                        return Err("too many rows.".to_string());
                    }
                    iy -= 1;
                    continue;
                }
                c => {
                    if let Ok(p) = Piece::from_char(c) {
                        p
                    } else {
                        let i = c as i32 - 48;
                        if i < 0 || ix + i as usize > RANK_NB {
                            return Err(format!("invalid char: {}.", c));
                        }
                        ix += i as usize;
                        continue;
                    }
                }
            };
            let i = iy * RANK_NB + ix;
            board.grid[i] = piece;
            let (pt, side) = piece.split();
            if pt == PieceType::King {
                board.king_sq[side as usize] = Some(Square::from_usize(i).unwrap());
            }
            if pt == PieceType::Prince {
                board.prince_sq[side as usize] = Some(Square::from_usize(i).unwrap());
            }
            change_bit!(board.boards[pt as usize], i);
            change_bit!(board.sides[side as usize], i);
            board.pieces[side as usize][pt as usize] += 1;
            ix += 1;
        }
        if ix != RANK_NB || iy != 0 {
            return Err("invalid number.".to_string());
        }

        if s[1] == "b" {
            board.side = Side::Black;
        } else if s[1] == "w" {
            board.side = Side::White;
        } else {
            return Err("invalid turn.".to_string());
        }

        if s[2] != "-" {
            let hand: Vec<char> = s[2].chars().collect();
            let mut i = 0;
            while i < hand.len() {
                let p = Piece::from_char(hand[i])?;
                i += 1;
                if i >= hand.len() || Piece::from_char(hand[i]).is_ok() {
                    board.add_hand(p.side(), p.pt());
                    break;
                }
                let count = hand[i] as i32 - 48;
                if count <= 1 {
                    return Err(format!("invalid char: {}.", hand[i]));
                }
                i += 1;
                for _ in 0..count {
                    board.add_hand(p.side(), p.pt());
                }
            }
        }

        if let Ok(count) = s[3].parse() {
            board.demise[0] = count;
        } else {
            return Err(format!("invalid demise: {}", s[3]));
        }

        if let Ok(count) = s[4].parse() {
            board.demise[1] = count;
        } else {
            return Err(format!("invalid demise: {}", s[4]));
        }

        board.calculate_effects();

        Ok(board)
    }
}
