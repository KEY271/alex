use core::fmt;
use std::{fmt::Write, ops::Not, str::FromStr};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::movegen::{get_from, get_move_type, get_to, Move, MoveType};

/// Square of the grid.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
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
pub enum PieceType {
    None, Light, Heavy, King1, King2, Prince1, Prince2, General, Knight, Arrow, Archer0, Archer1, Archer2,
}

/// Count of piece types.
pub const PIECE_TYPE_NB: usize = 13;

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceType::None     => write!(f, ". "),
            PieceType::Light    => write!(f, "L "),
            PieceType::Heavy    => write!(f, "H "),
            PieceType::King1    => write!(f, "K "),
            PieceType::King2    => write!(f, "K'"),
            PieceType::Prince1  => write!(f, "P "),
            PieceType::Prince2  => write!(f, "P'"),
            PieceType::General  => write!(f, "G "),
            PieceType::Knight   => write!(f, "N "),
            PieceType::Arrow    => write!(f, "R "),
            PieceType::Archer0  => write!(f, "A0"),
            PieceType::Archer1  => write!(f, "A1"),
            PieceType::Archer2  => write!(f, "A2"),
        }
    }
}

/// Type of the piece with the side.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
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
            Piece::None     => write!(f, ". "),
            Piece::BLight   => write!(f, "L "),
            Piece::BHeavy   => write!(f, "H "),
            Piece::BKing1   => write!(f, "K "),
            Piece::BKing2   => write!(f, "K'"),
            Piece::BPrince1 => write!(f, "P "),
            Piece::BPrince2 => write!(f, "P'"),
            Piece::BGeneral => write!(f, "G "),
            Piece::BKnight  => write!(f, "N "),
            Piece::BArrow   => write!(f, "R "),
            Piece::BArcher0 => write!(f, "A0"),
            Piece::BArcher1 => write!(f, "A1"),
            Piece::BArcher2 => write!(f, "A2"),
            Piece::PAD1     => write!(f, "**"),
            Piece::PAD2     => write!(f, "**"),
            Piece::PAD3     => write!(f, "**"),
            Piece::PAD4     => write!(f, "**"),
            Piece::WLight   => write!(f, "l "),
            Piece::WHeavy   => write!(f, "h "),
            Piece::WKing1   => write!(f, "k "),
            Piece::WKing2   => write!(f, "k'"),
            Piece::WPrince1 => write!(f, "p "),
            Piece::WPrince2 => write!(f, "p'"),
            Piece::WGeneral => write!(f, "g "),
            Piece::WKnight  => write!(f, "n "),
            Piece::WArrow   => write!(f, "r "),
            Piece::WArcher0 => write!(f, "a0"),
            Piece::WArcher1 => write!(f, "a1"),
            Piece::WArcher2 => write!(f, "a2"),
        }
    }
}

/// Type of the side.
/// Black takes the first move.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum Side {
    Black, White
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
            (PieceType::from_usize(*self as usize - 16).unwrap(), Side::White)
        } else {
            (PieceType::from_usize(*self as usize).unwrap(), Side::Black)
        }
    }
}

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
pub struct Board {
    /// Side.
    pub side: Side,
    /// Piece at the square.
    pub grid: [Piece; SQUARE_NB],
    /// Squares the piece can move to.
    pub movable_sq: [[Bitboard; SQUARE_NB]; PIECE_NB],
    /// Bitboards of the piece type with side.
    pub boards: [[Bitboard; PIECE_TYPE_NB]; SIDE_NB],
    /// Bitboard of occupied squares.
    pub occupied: Bitboard,
    /// Bitboard of occupied squares of sides.
    pub sides: [Bitboard; SIDE_NB],

    diagonal_mask: [Bitboard; SQUARE_NB],
    anti_diagonal_mask: [Bitboard; SQUARE_NB],
    rank_mask: [Bitboard; SQUARE_NB],

    fill_up_attacks: [[Bitboard; RANK_NB]; OCC_NB],
    a_file_attacks: [[Bitboard; RANK_NB]; OCC_NB],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for iy in (0..RANK_NB).rev() {
            for ix in 0..RANK_NB {
                write!(f, "{}", self.grid[iy * RANK_NB + ix])?;
                if ix < RANK_NB - 1 {
                    write!(f, " ")?;
                }
            }
            if iy > 0 {
                writeln!(f)?;
            }
        }
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
                                || iy >= 5 && ix.abs_diff(jx) == 1 && iy == jy {
                                change_bit!(bb, j);
                            }
                        },
                        PieceType::King1 | PieceType::King2 => {
                            if ix.abs_diff(jx) <= 1 && iy.abs_diff(jy) <= 1 && !(ix == jx && iy == jy) {
                                change_bit!(bb, j);
                            }
                        },
                        PieceType::Prince1 | PieceType::Prince2 => {
                            if ix == jx && iy + 1 == jy
                                || ix.abs_diff(jx) == 1 && iy.abs_diff(jy) == 1 {
                                change_bit!(bb, j);
                            }
                        },
                        PieceType::General => {
                            if ix.abs_diff(jx) + iy.abs_diff(jy) == 1
                                || ix.abs_diff(jx) == 1 && iy + 1 == jy {
                                change_bit!(bb, j);
                            }
                        },
                        PieceType::Knight => {
                            if ix.abs_diff(jx) + iy.abs_diff(jy) == 3 && ix != jx && iy != jy {
                                change_bit!(bb, j);
                            }
                        },
                        PieceType::Arrow => continue,
                        PieceType::Archer0 | PieceType::Archer1 | PieceType::Archer2 => {
                            if ix.abs_diff(jx) + iy.abs_diff(jy) == 1 {
                                change_bit!(bb, j);
                            }
                        },
                    }
                });
                movable_sq[pt.into_piece(Side::Black) as usize][i] = bb;
                movable_sq[pt.into_piece(Side::White) as usize][(RANK_NB - 1 - iy) * RANK_NB + ix] = flipped(bb);
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
                for i in file+1..RANK_NB {
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
                for i in rank+1..RANK_NB {
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
            boards: [[0; PIECE_TYPE_NB]; SIDE_NB],
            occupied: 0,
            sides: [0, 0],
            diagonal_mask,
            anti_diagonal_mask,
            rank_mask,
            fill_up_attacks,
            a_file_attacks,
            grid: [Piece::None; SQUARE_NB]
        }
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
            let board = self.boards[Side::Black as usize][PieceType::Heavy as usize] << 8;
            let board = board & !self.occupied;
            board << 8
        } else {
            let board = self.boards[Side::White as usize][PieceType::Heavy as usize] >> 8;
            let board = board & !self.occupied;
            board >> 8
        }
    }

    pub fn arrow_attacks(&self, sq: Square) -> Bitboard {
        self.file_attacks(self.occupied, sq) | self.rank_attacks(self.occupied, sq)
            | self.diagonal_attacks(self.occupied, sq) | self.anti_diagonal_attacks(self.occupied, sq)
    }

    pub fn do_move(&mut self, m: Move) {
        let to = get_to(m);
        match get_move_type(m) {
            MoveType::Normal => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();

                change_bit!(self.occupied, to as usize);
                change_bit!(self.boards[side as usize][pt as usize], to as usize);
                self.grid[to as usize] = self.grid[from as usize];

                change_bit!(self.occupied, from as usize);
                change_bit!(self.boards[side as usize][pt as usize], from as usize);
                self.grid[from as usize] = Piece::None;
            },
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                change_bit!(self.occupied, from as usize);
                change_bit!(self.boards[side as usize][PieceType::Arrow as usize], from as usize);
                self.grid[from as usize] = Piece::None;

                if pt == PieceType::Archer0 {
                    change_bit!(self.boards[side as usize][PieceType::Archer0 as usize], to as usize);
                    change_bit!(self.boards[side as usize][PieceType::Archer1 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer1.into_piece(side);
                } else if pt == PieceType::Archer1 {
                    change_bit!(self.boards[side as usize][PieceType::Archer1 as usize], to as usize);
                    change_bit!(self.boards[side as usize][PieceType::Archer2 as usize], to as usize);
                    self.grid[to as usize] = PieceType::Archer2.into_piece(side);
                }
            },
            MoveType::Shoot  => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();

                if pt == PieceType::Archer1 {
                    change_bit!(self.boards[side as usize][PieceType::Archer1 as usize], from as usize);
                    change_bit!(self.boards[side as usize][PieceType::Archer0 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer0.into_piece(side);
                } else if pt == PieceType::Archer2 {
                    change_bit!(self.boards[side as usize][PieceType::Archer2 as usize], from as usize);
                    change_bit!(self.boards[side as usize][PieceType::Archer1 as usize], from as usize);
                    self.grid[from as usize] = PieceType::Archer1.into_piece(side);
                }

                change_bit!(self.occupied, to as usize);
                change_bit!(self.boards[side as usize][PieceType::Arrow as usize], to as usize);
                self.grid[to as usize] = PieceType::Arrow.into_piece(side);
            },
            MoveType::Drop   => {
                todo!()
            },
            MoveType::Supply => {
                todo!()
            },
        }

        self.side = !self.side;
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut ix = 0;
        let mut iy = RANK_NB - 1;
        for c in s.chars() {
            let piece = match c {
                '/' => {
                    if ix != RANK_NB {
                        return Err("invalid row.".to_string())
                    }
                    ix = 0;
                    iy -= 1;
                    if iy == RANK_NB {
                        return Err("too many rows.".to_string())
                    }
                    continue;
                },
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
                        return Err(format!("invalid char: {}.", c))
                    }
                    ix += i as usize;
                    continue;
                }
            };
            let i = iy * RANK_NB + ix;
            board.grid[i] = piece;
            let (pt, side) = piece.split();
            change_bit!(board.boards[side as usize][pt as usize], i);
            change_bit!(board.occupied, i);
            change_bit!(board.sides[side as usize], i);
            ix += 1;
        }
        if ix != RANK_NB || iy != 0 {
            Err("invalid number.".to_string())
        } else {
            Ok(board)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    #[test]
    fn initial_position() {
        let board: Board = "bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB".parse().unwrap();
        let answer = "\
            a1 n  g  k' p  g  n  a1\n\
            l  l  h  h  h  h  l  l \n\
            .  .  .  .  .  .  .  . \n\
            .  .  .  .  .  .  .  . \n\
            .  .  .  .  .  .  .  . \n\
            .  .  .  .  .  .  .  . \n\
            L  L  H  H  H  H  L  L \n\
            A1 N  G  P  K' G  N  A1\
        ";
        assert_eq!(format!("{}", board), answer.to_string());
    }
}
