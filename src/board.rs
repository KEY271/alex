use core::fmt;
use std::str::FromStr;

use num_derive::FromPrimitive;

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
pub const SQUARE_NB: usize = 64;

/// Type of the piece.
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum PieceType {
    None, Light, Heavy, King1, King2, Prince1, Prince2, General, Knight, Arrow, Archer0, Archer1, Archer2,
}

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
            Piece::BKnight  => write!(f, "K "),
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

pub const RANK_NB: usize = 8;

/// Board.
pub struct Board {
    /// Piece at the square.
    pub grid: [Piece; SQUARE_NB],
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
        Board {
            grid: [Piece::None; SQUARE_NB]
        }
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
            board.grid[iy * RANK_NB + ix] = piece;
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
            A1 K  G  P  K' G  K  A1\
        ";
        assert_eq!(format!("{}", board), answer.to_string());
    }
}
