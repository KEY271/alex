use core::fmt;
use std::{fmt::Write, ops::Not, str::FromStr};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::movegen::{get_from, get_move_type, get_pt, get_to, make_move_normal, Move, MoveType};

/// Square of the grid.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
/// Count of squares.
pub const SQUARE_NB: usize = 64;

/// Count of ranks.
pub const RANK_NB: usize = 8;

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ix = *self as usize % RANK_NB;
        let iy = *self as usize / RANK_NB;
        write!(f, "{}{}", (ix + 65) as u8 as char, iy + 1)
    }
}

fn read_file(c: u8) -> Result<usize, String> {
    let a = 'A' as u8;
    if c < a {
        return Err("Invalid Character.".to_string());
    }
    let x = (c - a) as usize;
    if x >= RANK_NB {
        return Err("Invalid Character.".to_string());
    }
    return Ok(x);
}

fn read_rank(c: u8) -> Result<usize, String> {
    let a = '1' as u8;
    if c < a {
        return Err("Invalid Character.".to_string());
    }
    let y = (c - a) as usize;
    if y >= RANK_NB {
        return Err("Invalid Character.".to_string());
    }
    return Ok(y);
}

macro_rules! for_pos {
    ($ix:ident, $iy:ident, $i:ident, $e:expr) => {
        for $iy in 0..RANK_NB {
            for $ix in 0..RANK_NB {
                let $i = $iy * RANK_NB + $ix;
                $e;
            }
        }
    };
}

/// Type of the piece.
#[derive(FromPrimitive, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
#[rustfmt::skip]
pub enum PieceType {
    None, Light, Heavy, King1, King2, Prince1, Prince2, General, Knight, Arrow, Archer0, Archer1, Archer2,
}

/// Count of piece types.
pub const PIECE_TYPE_NB: usize = 13;

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::None => write!(f, ". "),
            PieceType::Light => write!(f, "L "),
            PieceType::Heavy => write!(f, "H "),
            PieceType::King1 => write!(f, "K "),
            PieceType::King2 => write!(f, "K'"),
            PieceType::Prince1 => write!(f, "P "),
            PieceType::Prince2 => write!(f, "P'"),
            PieceType::General => write!(f, "G "),
            PieceType::Knight => write!(f, "N "),
            PieceType::Arrow => write!(f, "R "),
            PieceType::Archer0 => write!(f, "A0"),
            PieceType::Archer1 => write!(f, "A1"),
            PieceType::Archer2 => write!(f, "A2"),
        }
    }
}

/// Type of the piece with the side.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
#[rustfmt::skip]
pub enum Piece {
    None,
    BLight, BHeavy, BKing1, BKing2, BPrince1, BPrince2, BGeneral, BKnight, BArrow, BArcher0, BArcher1, BArcher2,
    PAD1, PAD2, PAD3, PAD4,
    WLight, WHeavy, WKing1, WKing2, WPrince1, WPrince2, WGeneral, WKnight, WArrow, WArcher0, WArcher1, WArcher2,
}

/// Count of pieces.
pub const PIECE_NB: usize = 29;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::None => write!(f, "."),
            Piece::BLight => write!(f, "L"),
            Piece::BHeavy => write!(f, "H"),
            Piece::BKing1 => write!(f, "K"),
            Piece::BKing2 => write!(f, "K"),
            Piece::BPrince1 => write!(f, "P"),
            Piece::BPrince2 => write!(f, "P"),
            Piece::BGeneral => write!(f, "G"),
            Piece::BKnight => write!(f, "N"),
            Piece::BArrow => write!(f, "R"),
            Piece::BArcher0 => write!(f, "A"),
            Piece::BArcher1 => write!(f, "B"),
            Piece::BArcher2 => write!(f, "C"),
            Piece::PAD1 => write!(f, "*"),
            Piece::PAD2 => write!(f, "*"),
            Piece::PAD3 => write!(f, "*"),
            Piece::PAD4 => write!(f, "*"),
            Piece::WLight => write!(f, "l"),
            Piece::WHeavy => write!(f, "h"),
            Piece::WKing1 => write!(f, "k"),
            Piece::WKing2 => write!(f, "k"),
            Piece::WPrince1 => write!(f, "p"),
            Piece::WPrince2 => write!(f, "p"),
            Piece::WGeneral => write!(f, "g"),
            Piece::WKnight => write!(f, "n"),
            Piece::WArrow => write!(f, "r"),
            Piece::WArcher0 => write!(f, "a"),
            Piece::WArcher1 => write!(f, "b"),
            Piece::WArcher2 => write!(f, "c"),
        }
    }
}

/// Type of the side.
/// Black takes the first move.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum Side {
    Black,
    White,
}

/// Count of sides.
pub const SIDE_NB: usize = 2;

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Black => Side::White,
            Side::White => Side::Black,
        }
    }
}

impl PieceType {
    pub fn into_piece(&self, side: Side) -> Piece {
        if side == Side::Black {
            Piece::from_usize(*self as usize).unwrap()
        } else {
            Piece::from_usize(*self as usize + 16).unwrap()
        }
    }
}

impl Piece {
    pub fn split(&self) -> (PieceType, Side) {
        if *self >= Piece::WLight {
            (
                PieceType::from_usize(*self as usize - 16).unwrap(),
                Side::White,
            )
        } else {
            (PieceType::from_usize(*self as usize).unwrap(), Side::Black)
        }
    }

    pub fn pt(&self) -> PieceType {
        if *self >= Piece::WLight {
            PieceType::from_usize(*self as usize - 16).unwrap()
        } else {
            PieceType::from_usize(*self as usize).unwrap()
        }
    }

    pub fn side(&self) -> Side {
        if *self >= Piece::WLight {
            Side::White
        } else {
            Side::Black
        }
    }
}

/// Hand.
pub type Hand = u32;
const HAND_LIGHT_CAP: u32 = 0b000000000000000000001111;
const HAND_LIGHT_SHIFT: u32 = 0;
const HAND_HEAVY_CAP: u32 = 0b000000000000000011110000;
const HAND_HEAVY_SHIFT: u32 = 4;
const HAND_GENERAL_CAP: u32 = 0b000000000000111100000000;
const HAND_GENERAL_SHIFT: u32 = 8;
const HAND_KNIGHT_CAP: u32 = 0b000000001111000000000000;
const HAND_KNIGHT_SHIFT: u32 = 12;
const HAND_ARROW_CAP: u32 = 0b000011110000000000000000;
const HAND_ARROW_SHIFT: u32 = 16;
const HAND_ARCHER_CAP: u32 = 0b111100000000000000000000;
const HAND_ARCHER_SHIFT: u32 = 20;

/// Bitboard.
pub type Bitboard = u64;

pub fn pretty_bb(bb: Bitboard) -> String {
    let mut output = String::new();
    for iy in (0..RANK_NB).rev() {
        for ix in 0..RANK_NB {
            let _ = write!(&mut output, "{}", bit(bb, iy * RANK_NB + ix));
        }
        if iy > 0 {
            let _ = writeln!(&mut output);
        }
    }
    output
}

fn bit(bb: Bitboard, i: usize) -> u64 {
    (bb >> i) & 1
}

/// Changes a bit of the bitboard.
macro_rules! change_bit {
    ($b:expr, $i:expr) => {
        $b ^= 1 << $i;
    };
}

fn popcount(bb: Bitboard) -> u64 {
    let bb = (bb & 0x5555555555555555) + ((bb >> 1) & 0x5555555555555555);
    let bb = (bb & 0x3333333333333333) + ((bb >> 2) & 0x3333333333333333);
    let bb = (bb & 0x0f0f0f0f0f0f0f0f) + ((bb >> 4) & 0x0f0f0f0f0f0f0f0f);
    let bb = (bb & 0x00ff00ff00ff00ff) + ((bb >> 8) & 0x00ff00ff00ff00ff);
    let bb = (bb & 0x0000ffff0000ffff) + ((bb >> 16) & 0x0000ffff0000ffff);
    (bb & 0x00000000ffffffff) + ((bb >> 32) & 0x000000ffffffffff)
}

pub fn get_pos(bb: Bitboard) -> Square {
    Square::from_u64(popcount((bb & bb.wrapping_neg()) - 1)).unwrap()
}

#[macro_export]
macro_rules! foreach_bb {
    ($board:expr, $sq:ident, $e:expr) => {
        let mut bb = $board;
        while bb != 0 {
            let $sq = get_pos(bb);
            $e;
            bb &= bb.wrapping_sub(1);
        }
    };
}

/// Returns a y-flipped bitboard.
fn flipped(bb: Bitboard) -> Bitboard {
    let mut new_bb = 0;
    for i in 0..RANK_NB {
        new_bb ^= ((bb >> (i * RANK_NB)) & 0xFF) << (SQUARE_NB - RANK_NB - i * RANK_NB);
    }
    new_bb
}

/// Count of occupation.
pub const OCC_NB: usize = 64;

/// Board.
#[derive(PartialEq, Eq)]
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

    /// Squares the piece can move to.
    pub movable_sq: [[Bitboard; SQUARE_NB]; PIECE_NB],
    // Kindergarden bitboard.
    diagonal_mask: [Bitboard; SQUARE_NB],
    anti_diagonal_mask: [Bitboard; SQUARE_NB],
    rank_mask: [Bitboard; SQUARE_NB],
    fill_up_attacks: [[Bitboard; RANK_NB]; OCC_NB],
    a_file_attacks: [[Bitboard; RANK_NB]; OCC_NB],
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

        write!(f, " {}", if self.side == Side::Black { 'b' } else { 'w' })?;
        Ok(())
    }
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
                        PieceType::King1 | PieceType::King2 => {
                            if ix.abs_diff(jx) <= 1
                                && iy.abs_diff(jy) <= 1
                                && !(ix == jx && iy == jy)
                            {
                                change_bit!(bb, j);
                            }
                        }
                        PieceType::Prince1 | PieceType::Prince2 => {
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
        let diagonal_a2_h7 = 0x0080402010080400;
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

    pub fn count_hand(&self, side: Side, pt: PieceType) -> u32 {
        let hand = self.hands[side as usize];
        match pt {
            PieceType::Light => (hand & HAND_LIGHT_CAP) >> HAND_LIGHT_SHIFT,
            PieceType::Heavy => (hand & HAND_HEAVY_CAP) >> HAND_HEAVY_SHIFT,
            PieceType::General => (hand & HAND_GENERAL_CAP) >> HAND_GENERAL_SHIFT,
            PieceType::Knight => (hand & HAND_KNIGHT_CAP) >> HAND_KNIGHT_SHIFT,
            PieceType::Arrow => (hand & HAND_ARROW_CAP) >> HAND_ARROW_SHIFT,
            PieceType::Archer0 => (hand & HAND_ARCHER_CAP) >> HAND_ARCHER_SHIFT,
            _ => panic!(),
        }
    }

    pub fn add_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] += match pt {
            PieceType::Light => 1 << HAND_LIGHT_SHIFT,
            PieceType::Heavy => 1 << HAND_HEAVY_SHIFT,
            PieceType::General => 1 << HAND_GENERAL_SHIFT,
            PieceType::Knight => 1 << HAND_KNIGHT_SHIFT,
            PieceType::Arrow => 1 << HAND_ARROW_SHIFT,
            PieceType::Archer0 => 1 << HAND_ARCHER_SHIFT,
            PieceType::Archer1 => 1 << HAND_ARCHER_SHIFT + 1 << HAND_ARROW_SHIFT,
            PieceType::Archer2 => 1 << HAND_ARCHER_SHIFT + 2 << HAND_ARROW_SHIFT,
            _ => panic!(),
        };
    }

    pub fn remove_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] -= match pt {
            PieceType::Light => 1 << HAND_LIGHT_SHIFT,
            PieceType::Heavy => 1 << HAND_HEAVY_SHIFT,
            PieceType::General => 1 << HAND_GENERAL_SHIFT,
            PieceType::Knight => 1 << HAND_KNIGHT_SHIFT,
            PieceType::Arrow => 1 << HAND_ARROW_SHIFT,
            PieceType::Archer0 => 1 << HAND_ARCHER_SHIFT,
            PieceType::Archer1 => 1 << HAND_ARCHER_SHIFT + 1 << HAND_ARROW_SHIFT,
            PieceType::Archer2 => 1 << HAND_ARCHER_SHIFT + 2 << HAND_ARROW_SHIFT,
            _ => panic!(),
        };
    }

    pub fn do_move(&mut self, m: Move) {
        let to = get_to(m);
        match get_move_type(m) {
            MoveType::Normal => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let pt2 = self.grid[to as usize].pt();
                if pt2 != PieceType::None {
                    self.add_hand(side, pt2);
                }

                change_bit!(self.boards[pt as usize], to as usize);
                change_bit!(self.sides[side as usize], to as usize);
                self.grid[to as usize] = self.grid[from as usize];

                change_bit!(self.boards[pt as usize], from as usize);
                change_bit!(self.sides[side as usize], from as usize);
                self.grid[from as usize] = Piece::None;
            }
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                change_bit!(self.boards[PieceType::Arrow as usize], from as usize);
                change_bit!(self.sides[side as usize], from as usize);
                self.grid[from as usize] = Piece::None;

                if pt == PieceType::Archer0 {
                    change_bit!(self.boards[PieceType::Archer0 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer1.into_piece(side);
                } else if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer2 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer2.into_piece(side);
                }
            }
            MoveType::Shoot => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let pt2 = self.grid[to as usize].pt();
                if pt2 != PieceType::None {
                    self.add_hand(side, pt2);
                }

                if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], from as usize);
                    change_bit!(self.boards[PieceType::Archer0 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer0.into_piece(side);
                } else if pt == PieceType::Archer2 {
                    change_bit!(self.boards[PieceType::Archer2 as usize], from as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer1.into_piece(side);
                }

                change_bit!(self.sides[side as usize], to as usize);
                change_bit!(self.boards[PieceType::Arrow as usize], to as usize);
                self.grid[to as usize] = PieceType::Arrow.into_piece(side);
            }
            MoveType::Drop => {
                let pt = get_pt(m);
                self.remove_hand(self.side, pt);
                change_bit!(self.sides[self.side as usize], to as usize);
                change_bit!(self.boards[pt as usize], to as usize);
                self.grid[to as usize] = pt.into_piece(self.side);
            }
            MoveType::Supply => {
                self.remove_hand(self.side, PieceType::Arrow);
                let pt = self.grid[to as usize].pt();
                if pt == PieceType::Archer0 {
                    change_bit!(self.boards[PieceType::Archer0 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer1.into_piece(self.side);
                } else if pt == PieceType::Archer1 {
                    change_bit!(self.boards[PieceType::Archer1 as usize], to as usize);
                    change_bit!(self.boards[PieceType::Archer2 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer2.into_piece(self.side);
                }
            }
        }

        self.side = !self.side;
    }

    /// Make a move from mfen.
    pub fn read_move(&self, mfen: String) -> Result<Move, String> {
        if mfen.len() != 4 {
            return Err("Invalid length.".to_string());
        }
        let mfen = mfen.as_bytes();
        let x1 = read_file(mfen[0])?;
        let y1 = read_rank(mfen[1])?;
        let from = Square::from_usize(y1 * RANK_NB + x1).unwrap();
        let x2 = read_file(mfen[2])?;
        let y2 = read_rank(mfen[3])?;
        let to = Square::from_usize(y2 * RANK_NB + x2).unwrap();
        let m = make_move_normal(self.grid[to as usize].pt(), from, to);
        Ok(m)
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut ix = 0;
        let mut iy = RANK_NB - 1;
        let s: Vec<&str> = s.split(" ").collect();
        if s.len() != 2 {
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
                'L' => Piece::BLight,
                'H' => Piece::BHeavy,
                'K' => Piece::BKing2,
                'P' => Piece::BPrince1,
                'G' => Piece::BGeneral,
                'N' => Piece::BKnight,
                'R' => Piece::BArrow,
                'A' => Piece::BArcher0,
                'B' => Piece::BArcher1,
                'C' => Piece::BArcher2,
                'l' => Piece::WLight,
                'h' => Piece::WHeavy,
                'k' => Piece::WKing2,
                'p' => Piece::WPrince1,
                'g' => Piece::WGeneral,
                'n' => Piece::WKnight,
                'r' => Piece::WArrow,
                'a' => Piece::WArcher0,
                'b' => Piece::WArcher1,
                'c' => Piece::WArcher2,
                c => {
                    let i = c as i32 - 48;
                    if i < 0 || ix + i as usize > RANK_NB {
                        return Err(format!("invalid char: {}.", c));
                    }
                    ix += i as usize;
                    continue;
                }
            };
            let i = iy * RANK_NB + ix;
            board.grid[i] = piece;
            let (pt, side) = piece.split();
            change_bit!(board.boards[pt as usize], i);
            change_bit!(board.sides[side as usize], i);
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
        Ok(board)
    }
}
