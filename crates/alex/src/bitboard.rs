use std::sync::LazyLock;

use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::{
    change_bit,
    position::OCC_NB,
    types::{Bitboard, PieceType, Side, Square, PIECE_NB, PIECE_TYPE_NB, RANK_NB, SQUARE_NB},
};

pub static KG_BITBOARD: LazyLock<Bitboards> = LazyLock::new(|| Bitboards::new());

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

/// Returns a y-flipped bitboard.
pub fn flipped(bb: Bitboard) -> Bitboard {
    let mut new_bb = 0;
    for i in 0..RANK_NB {
        new_bb ^= ((bb >> (i * RANK_NB)) & 0xFF) << (SQUARE_NB - RANK_NB - i * RANK_NB);
    }
    new_bb
}

#[derive(Clone)]
pub struct Bitboards {
    /// Squares the piece can move to.
    pub movable_sq: [[Bitboard; SQUARE_NB]; PIECE_NB],
    // Kindergarden bitboard.
    pub diagonal_mask: [Bitboard; SQUARE_NB],
    pub anti_diagonal_mask: [Bitboard; SQUARE_NB],
    pub rank_mask: [Bitboard; SQUARE_NB],
    pub fill_up_attacks: [[Bitboard; RANK_NB]; OCC_NB],
    pub a_file_attacks: [[Bitboard; RANK_NB]; OCC_NB],
    /// Line between squares.
    pub between_bb: [[Bitboard; SQUARE_NB]; SQUARE_NB],
    /// Line passing squares.
    pub line_bb: [[Bitboard; SQUARE_NB]; SQUARE_NB],
    pub check_bb: [[Bitboard; SQUARE_NB]; PIECE_NB],
    /// Arrow attacks if there is no piece.
    pub arrow_attacks: [Bitboard; SQUARE_NB],
}

impl Bitboards {
    fn new() -> Self {
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

        let mut between_bb = [[0; SQUARE_NB]; SQUARE_NB];
        for_pos!(ix, iy, i, {
            for_pos!(jx, jy, j, {
                if i == j {
                    continue;
                }
                if ix == jx || iy == jy || ix + iy == jx + jy || ix + jy == iy + jx {
                    let d = ix.abs_diff(jx).max(iy.abs_diff(jy));
                    for k in 1..d {
                        let x = ix as isize + (jx as isize - ix as isize) * k as isize / d as isize;
                        let y = iy as isize + (jy as isize - iy as isize) * k as isize / d as isize;
                        change_bit!(between_bb[i][j], y as usize * RANK_NB + x as usize);
                    }
                }
            });
        });

        let mut line_bb = [[0; SQUARE_NB]; SQUARE_NB];
        for_pos!(ix, iy, i, {
            for_pos!(jx, jy, j, {
                if i == j {
                    continue;
                }
                if ix == jx || iy == jy || ix + iy == jx + jy || ix + jy == iy + jx {
                    let d = ix.abs_diff(jx).max(iy.abs_diff(jy));
                    for k in -8..9 {
                        let x = ix as isize + (jx as isize - ix as isize) * k / d as isize;
                        let y = iy as isize + (jy as isize - iy as isize) * k / d as isize;
                        if 0 <= x && x < 8 && 0 <= y && y < 8 {
                            change_bit!(line_bb[i][j], y as usize * RANK_NB + x as usize);
                        }
                    }
                }
            });
        });

        let mut check_bb = [[0; SQUARE_NB]; PIECE_NB];
        for pt in 1..PIECE_TYPE_NB {
            let pt = PieceType::from_usize(pt).unwrap();
            let p1 = pt.into_piece(Side::Black);
            let p2 = pt.into_piece(Side::White);
            for i in 0..SQUARE_NB {
                for j in 0..SQUARE_NB {
                    if movable_sq[p1 as usize][j] & (1 << i) != 0 {
                        check_bb[p1 as usize][i] |= 1 << j;
                    }
                    if movable_sq[p2 as usize][j] & (1 << i) != 0 {
                        check_bb[p2 as usize][i] |= 1 << j;
                    }
                }
            }
        }

        let mut arrow_attacks = [0; SQUARE_NB];
        for_pos!(ix, iy, i, {
            let mut bb = 0;
            for_pos!(jx, jy, j, {
                if i == j {
                    continue;
                }
                if ix == jx || iy == jy || ix + iy == jx + jy || ix + jy == iy + jx {
                    change_bit!(bb, j);
                }
            });
            arrow_attacks[i] = bb;
        });
        Bitboards {
            movable_sq,
            diagonal_mask,
            anti_diagonal_mask,
            rank_mask,
            fill_up_attacks,
            a_file_attacks,
            between_bb,
            line_bb,
            check_bb,
            arrow_attacks,
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
        let diagonal_a2_h7 = 0x0004081020408000;
        let occ = afile & (occ >> (sq as usize & 7));
        let occ = occ.wrapping_mul(diagonal_a2_h7) >> 58;
        self.a_file_attacks[occ as usize][sq as usize >> 3] << (sq as usize & 7)
    }

    pub fn heavy_attacks(&self, heavy_side: Bitboard, pieces: Bitboard, side: Side) -> Bitboard {
        if side == Side::Black {
            let board = heavy_side << 8;
            let board = board & !pieces;
            board << 8
        } else {
            let board = heavy_side >> 8;
            let board = board & !pieces;
            board >> 8
        }
    }

    pub fn arrow_attacks(&self, pieces: Bitboard, sq: Square) -> Bitboard {
        let occupied = pieces;
        self.file_attacks(occupied, sq)
            | self.rank_attacks(occupied, sq)
            | self.diagonal_attacks(occupied, sq)
            | self.anti_diagonal_attacks(occupied, sq)
    }
}
