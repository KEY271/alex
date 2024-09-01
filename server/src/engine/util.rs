use core::fmt;
use std::{fmt::Write, ops::Not};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum_macros::EnumIter;

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

pub fn read_file(c: u8) -> Result<usize, String> {
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

pub fn read_rank(c: u8) -> Result<usize, String> {
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

#[macro_export]
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
    None, Light, Heavy, King, Prince, General, Knight, Arrow, Archer0, Archer1, Archer2,
}
/// Count of piece types.
pub const PIECE_TYPE_NB: usize = 13;

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::None => write!(f, "."),
            PieceType::Light => write!(f, "L"),
            PieceType::Heavy => write!(f, "H"),
            PieceType::King => write!(f, "K"),
            PieceType::Prince => write!(f, "P"),
            PieceType::General => write!(f, "G"),
            PieceType::Knight => write!(f, "N"),
            PieceType::Arrow => write!(f, "R"),
            PieceType::Archer0 => write!(f, "A"),
            PieceType::Archer1 => write!(f, "B"),
            PieceType::Archer2 => write!(f, "C"),
        }
    }
}

/// Type of the piece with the side.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
#[rustfmt::skip]
pub enum Piece {
    None,
    BLight, BHeavy, BKing, BPrince, BGeneral, BKnight, BArrow, BArcher0, BArcher1, BArcher2,
    PAD1, PAD2, PAD3, PAD4, PAD5, PAD6,
    WLight, WHeavy, WKing, WPrince, WGeneral, WKnight, WArrow, WArcher0, WArcher1, WArcher2,
}
/// Count of pieces.
pub const PIECE_NB: usize = 29;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::None => write!(f, "."),
            Piece::BLight => write!(f, "L"),
            Piece::BHeavy => write!(f, "H"),
            Piece::BKing => write!(f, "K"),
            Piece::BPrince => write!(f, "P"),
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
            Piece::PAD5 => write!(f, "*"),
            Piece::PAD6 => write!(f, "*"),
            Piece::WLight => write!(f, "l"),
            Piece::WHeavy => write!(f, "h"),
            Piece::WKing => write!(f, "k"),
            Piece::WPrince => write!(f, "p"),
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

    pub fn from_char(mfen: u8) -> PieceType {
        match mfen {
            b'L' | b'l' => PieceType::Light,
            b'H' | b'h' => PieceType::Heavy,
            b'K' | b'k' => PieceType::King,
            b'P' | b'p' => PieceType::Prince,
            b'G' | b'g' => PieceType::General,
            b'N' | b'n' => PieceType::Knight,
            b'R' | b'r' => PieceType::Arrow,
            b'A' | b'a' => PieceType::Archer0,
            b'B' | b'b' => PieceType::Archer1,
            b'C' | b'c' => PieceType::Archer2,
            _ => PieceType::None,
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

pub fn count_hand(hand: Hand, pt: PieceType) -> u32 {
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

pub fn to_hand(pt: PieceType) -> Hand {
    match pt {
        PieceType::Light => 1 << HAND_LIGHT_SHIFT,
        PieceType::Heavy => 1 << HAND_HEAVY_SHIFT,
        PieceType::General => 1 << HAND_GENERAL_SHIFT,
        PieceType::Knight => 1 << HAND_KNIGHT_SHIFT,
        PieceType::Arrow => 1 << HAND_ARROW_SHIFT,
        PieceType::Archer0 => 1 << HAND_ARCHER_SHIFT,
        PieceType::Archer1 => (1 << HAND_ARCHER_SHIFT) + (1 << HAND_ARROW_SHIFT),
        PieceType::Archer2 => (1 << HAND_ARCHER_SHIFT) + (2 << HAND_ARROW_SHIFT),
        _ => 0,
    }
}

/// Bitboard.
pub type Bitboard = u64;

#[allow(dead_code)]
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
#[macro_export]
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
            let $sq = crate::engine::util::get_pos(bb);
            $e;
            bb &= bb.wrapping_sub(1);
        }
    };
}

/// Returns a y-flipped bitboard.
pub fn flipped(bb: Bitboard) -> Bitboard {
    let mut new_bb = 0;
    for i in 0..RANK_NB {
        new_bb ^= ((bb >> (i * RANK_NB)) & 0xFF) << (SQUARE_NB - RANK_NB - i * RANK_NB);
    }
    new_bb
}

/// Move.
pub type Move = u32;

/// Type of the move.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum MoveType {
    /// Move a piece.
    Normal,
    /// Return an arrow to an archer.
    Return,
    /// Shoot an arrow.
    Shoot,
    /// Drop a piece from hand.
    Drop,
    /// Supply an arrow to an archer.
    Supply,
}

/// Mask of a captured piece.
const MOVE_CAP: u32 = 0b1111_0_000_000000_000000;
const MOVE_CAP_SHIFT: u32 = 16;
/// Mask of a demise flag.
pub const MOVE_DEMISE: u32 = 0b0000_1_000_000000_000000;
/// Mask of a move type.
const MOVE_TYPE: u32 = 0b0000_0_111_000000_000000;
const MOVE_TYPE_SHIFT: u32 = 12;
/// Mask of a square the piece move from.
/// Or the piece type if the move type is drop.
const MOVE_FROM: u32 = 0b0000_0_000_111111_000000;
const MOVE_FROM_SHIFT: u32 = 6;
/// Mask of a square the piece move to.
const MOVE_TO: u32 = 0b0000_0_000_000000_111111;
const MOVE_TO_SHIFT: u32 = 0;

pub fn get_capture(m: Move) -> PieceType {
    PieceType::from_u32((m & MOVE_CAP) >> MOVE_CAP_SHIFT).unwrap()
}

pub fn is_demise(m: Move) -> bool {
    m & MOVE_DEMISE != 0
}

pub fn get_move_type(m: Move) -> MoveType {
    MoveType::from_u32((m & MOVE_TYPE) >> MOVE_TYPE_SHIFT).unwrap()
}

pub fn get_from(m: Move) -> Square {
    Square::from_u32((m & MOVE_FROM) >> MOVE_FROM_SHIFT).unwrap()
}

pub fn get_pt(m: Move) -> PieceType {
    PieceType::from_u32((m & MOVE_FROM) >> MOVE_FROM_SHIFT).unwrap()
}

pub fn get_to(m: Move) -> Square {
    Square::from_u32((m & MOVE_TO) >> MOVE_TO_SHIFT).unwrap()
}

pub fn move_to_mfen(m: Move, side: Side) -> String {
    match get_move_type(m) {
        MoveType::Normal => format!("{}{}", get_from(m), get_to(m)),
        MoveType::Return => format!("{}{}", get_from(m), get_to(m)),
        MoveType::Shoot => format!("{}{}S", get_from(m), get_to(m)),
        MoveType::Drop => format!("{}{}", get_to(m), get_pt(m).into_piece(side)),
        MoveType::Supply => {
            let to = get_to(m);
            if side == Side::Black {
                format!("{}R", to)
            } else {
                format!("{}r", to)
            }
        }
    }
}

pub fn make_move_normal(cap: PieceType, from: Square, to: Square) -> Move {
    ((cap as u32) << MOVE_CAP_SHIFT)
        + ((MoveType::Normal as u32) << MOVE_TYPE_SHIFT)
        + ((from as u32) << MOVE_FROM_SHIFT)
        + ((to as u32) << MOVE_TO_SHIFT)
}
pub fn make_move_return(from: Square, to: Square) -> Move {
    ((MoveType::Return as u32) << MOVE_TYPE_SHIFT)
        + ((from as u32) << MOVE_FROM_SHIFT)
        + ((to as u32) << MOVE_TO_SHIFT)
}
pub fn make_move_shoot(cap: PieceType, from: Square, to: Square) -> Move {
    ((cap as u32) << MOVE_CAP_SHIFT)
        + ((MoveType::Shoot as u32) << MOVE_TYPE_SHIFT)
        + ((from as u32) << MOVE_FROM_SHIFT)
        + ((to as u32) << MOVE_TO_SHIFT)
}
pub fn make_move_drop(pt: PieceType, to: Square) -> Move {
    ((MoveType::Drop as u32) << MOVE_TYPE_SHIFT)
        + ((pt as u32) << MOVE_FROM_SHIFT)
        + ((to as u32) << MOVE_TO_SHIFT)
}
pub fn make_move_supply(to: Square) -> Move {
    ((MoveType::Supply as u32) << MOVE_TYPE_SHIFT) + ((to as u32) << MOVE_TO_SHIFT)
}
