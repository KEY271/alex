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

/// Position.
#[derive(PartialEq, Eq, Clone)]
pub struct Position {
    pub side: Side,
    /// Piece at the square.
    pub grid: [Piece; SQUARE_NB],
    /// Bitboards of the piece type.
    pub boards: [Bitboard; PIECE_TYPE_NB],
    /// Bitboard of occupied squares of sides.
    pub sides: [Bitboard; SIDE_NB],
    /// Hands of sides.
    pub hands: [Hand; SIDE_NB],
    /// Count of demise.
    pub demise: [usize; SIDE_NB],
    /// Effect.
    pub effects: [[usize; SQUARE_NB]; SIDE_NB],
    /// Count of piece.
    pub piece_count: [[usize; PIECE_TYPE_NB]; SIDE_NB],
    /// Square of piece type.
    pub piece_list: [[[Square; 8]; PIECE_TYPE_NB]; SIDE_NB],
    /// Index of piece.
    pub index: [usize; SQUARE_NB],

    /// Squares the piece can move to.
    pub movable_sq: [[Bitboard; SQUARE_NB]; PIECE_NB],
    // Kindergarden bitboard.
    diagonal_mask: [Bitboard; SQUARE_NB],
    anti_diagonal_mask: [Bitboard; SQUARE_NB],
    rank_mask: [Bitboard; SQUARE_NB],
    fill_up_attacks: [[Bitboard; RANK_NB]; OCC_NB],
    a_file_attacks: [[Bitboard; RANK_NB]; OCC_NB],
}

impl Position {
    /// Create an empty board.
    pub fn new() -> Position {
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
        Position {
            side: Side::Black,
            movable_sq,
            boards: [0; PIECE_TYPE_NB],
            sides: [0, 0],
            hands: [0, 0],
            demise: [0, 0],
            effects: [[0; SQUARE_NB]; SIDE_NB],
            piece_count: [[0; PIECE_TYPE_NB]; SIDE_NB],
            piece_list: [[[Square::NONE; 8]; PIECE_TYPE_NB]; SIDE_NB],
            index: [8; SQUARE_NB],
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

    pub fn calculate_effects(&self) -> [[usize; SQUARE_NB]; SIDE_NB] {
        let mut effects = [[0; SQUARE_NB]; SIDE_NB];
        for i in 0..SQUARE_NB {
            foreach_bb!(self.movable_sq[self.grid[i] as usize][i], sq2, {
                effects[self.grid[i].side() as usize][sq2 as usize] += 1;
            });
        }
        effects
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

    pub fn count_hand(&self, side: Side, pt: PieceType) -> u32 {
        count_hand(self.hands[side as usize], pt)
    }

    pub fn add_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] += to_hand(pt);
    }

    pub fn remove_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] -= to_hand(pt);
    }

    fn add_piece(&mut self, pt: PieceType, side: Side, sq: Square) {
        change_bit!(self.boards[pt as usize], sq as usize);
        change_bit!(self.sides[side as usize], sq as usize);
        let p = pt.into_piece(side);
        self.grid[sq as usize] = p;
        self.add_effect(sq, p);
        let count = self.piece_count[side as usize][pt as usize];
        self.piece_list[side as usize][pt as usize][count] = sq;
        self.piece_count[side as usize][pt as usize] += 1;
        self.index[sq as usize] = count;
    }

    fn remove_piece(&mut self, sq: Square) {
        let p = self.grid[sq as usize];
        let (pt, side) = p.split();
        change_bit!(self.boards[pt as usize], sq as usize);
        change_bit!(self.sides[side as usize], sq as usize);
        self.remove_effect(sq, p);
        let count = self.piece_count[side as usize][pt as usize];
        let index = self.index[sq as usize];
        self.piece_list[side as usize][pt as usize][index] =
            self.piece_list[side as usize][pt as usize][count - 1];
        self.index[self.piece_list[side as usize][pt as usize][count - 1] as usize] = index;
        self.piece_list[side as usize][pt as usize][count - 1] = Square::NONE;
        self.piece_count[side as usize][pt as usize] -= 1;
        self.index[sq as usize] = 8;
        self.grid[sq as usize] = Piece::None;
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
                let cap = get_capture(m);

                if cap != PieceType::None {
                    self.remove_piece(to);
                    self.add_hand(side, cap);
                }
                self.remove_piece(from);
                self.add_piece(pt, side, to);
            }
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                self.remove_piece(from);
                self.remove_piece(to);
                if pt == PieceType::Archer0 {
                    self.add_piece(PieceType::Archer1, side, to);
                } else if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer2, side, to);
                }
            }
            MoveType::Shoot => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let cap = get_capture(m);

                if cap != PieceType::None {
                    self.remove_piece(to);
                    self.add_hand(side, cap);
                }

                self.remove_piece(from);
                if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer0, side, from);
                } else if pt == PieceType::Archer2 {
                    self.add_piece(PieceType::Archer1, side, from);
                }
                self.add_piece(PieceType::Arrow, side, to);
            }
            MoveType::Drop => {
                let pt = get_pt(m);
                self.remove_hand(self.side, pt);
                self.add_piece(pt, self.side, to);
            }
            MoveType::Supply => {
                self.remove_hand(self.side, PieceType::Arrow);
                let pt = self.grid[to as usize].pt();
                self.remove_piece(to);
                if pt == PieceType::Archer0 {
                    self.add_piece(PieceType::Archer1, self.side, to);
                } else if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer2, self.side, to);
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
                let pt = self.grid[to as usize].pt();
                let from = get_from(m);
                let cap = get_capture(m);

                self.remove_piece(to);
                if cap != PieceType::None {
                    self.remove_hand(side, cap);
                    self.add_piece(cap, !side, to);
                }
                self.add_piece(pt, side, from);
            }
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                self.add_piece(PieceType::Arrow, side, from);
                self.remove_piece(to);
                if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer0, side, to);
                } else if pt == PieceType::Archer2 {
                    self.add_piece(PieceType::Archer1, side, to);
                }
            }
            MoveType::Shoot => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let cap = get_capture(m);

                self.remove_piece(to);
                if cap != PieceType::None {
                    self.remove_hand(side, cap);
                    self.add_piece(cap, !side, to);
                }
                self.remove_piece(from);
                if pt == PieceType::Archer0 {
                    self.add_piece(PieceType::Archer1, side, from);
                } else if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer2, side, from);
                }
            }
            MoveType::Drop => {
                let pt = get_pt(m);

                self.add_hand(side, pt);
                self.remove_piece(to);
            }
            MoveType::Supply => {
                let pt = self.grid[to as usize].pt();

                self.add_hand(side, PieceType::Arrow);
                self.remove_piece(to);
                if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer0, side, to);
                } else if pt == PieceType::Archer2 {
                    self.add_piece(PieceType::Archer1, side, to);
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

    pub fn crown_sq(&self, side: Side) -> Square {
        if self.demise[side as usize] % 2 == 0 {
            self.piece_list[side as usize][PieceType::King as usize][0]
        } else {
            self.piece_list[side as usize][PieceType::Prince as usize][0]
        }
    }

    pub fn calculate_checks(&self) -> Bitboard {
        let crown: Square = self.crown_sq(self.side);
        let mut checkers = 0;
        for i in 0..SQUARE_NB {
            let p = self.grid[i];
            if p == Piece::None || p.side() == self.side {
                continue;
            }
            foreach_bb!(self.movable_sq[p as usize][i], sq2, {
                if sq2 == crown {
                    checkers |= 1 << i;
                }
            });
        }
        if self.heavy_attacks(!self.side) & (1 << crown as usize) != 0 {
            if self.side == Side::Black {
                checkers |= 1 << (crown as usize + RANK_NB * 2);
            } else {
                checkers |= 1 << (crown as usize - RANK_NB * 2);
            }
        }
        let archer = self.pieces_pt_side(PieceType::Archer1, !self.side)
            | self.pieces_pt_side(PieceType::Archer2, !self.side);
        foreach_bb!(archer, sq, {
            let attacks = self.arrow_attacks(sq);
            if attacks & (1 << crown as usize) != 0 {
                checkers |= 1 << sq as usize;
            }
        });
        checkers
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

impl fmt::Display for Position {
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

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Position::new();
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
            let (pt, side) = piece.split();
            if pt != PieceType::None {
                board.add_piece(pt, side, Square::from_usize(i).unwrap());
            }
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

        board.effects = board.calculate_effects();

        Ok(board)
    }
}
