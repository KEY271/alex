use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::{board::{get_pos, Bitboard, Board, PieceType, Side, Square}, foreach_bb};

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
    Supply
}

/// Mask of a captured piece.
const MOVE_CAP: u32     = 0b11110000000000000000;
const MOVE_CAP_SHIFT: u32 = 16;
/// Mask of a demise flag.
const MOVE_DEMISE: u32  = 0b00001000000000000000;
const MOVE_DEMISE_SHIFT: u32 = 15;
/// Mask of a move type.
const MOVE_TYPE: u32    = 0b00000111000000000000;
const MOVE_TYPE_SHIFT: u32 = 12;
/// Mask of a square the piece move from.
/// Or the piece type if the move type is drop.
const MOVE_FROM: u32    = 0b00000000111111000000;
const MOVE_FROM_SHIFT: u32 = 6;
/// Mask of a square the piece move to.
const MOVE_TO: u32      = 0b00000000000000111111;
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

pub fn get_to(m: Move) -> Square {
    Square::from_u32((m & MOVE_TO) >> MOVE_TO_SHIFT).unwrap()
}

pub fn pretty_move(m: Move) -> String {
    match get_move_type(m) {
        MoveType::Normal => format!("{} {}", get_from(m), get_to(m)),
        MoveType::Return => format!("R {} {}", get_from(m), get_to(m)),
        MoveType::Shoot  => format!("S {} {}", get_from(m), get_to(m)),
        MoveType::Drop   => todo!(),
        MoveType::Supply => todo!(),
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
    ((MoveType::Supply as u32) << MOVE_TYPE_SHIFT)
        + ((to as u32) << MOVE_TO_SHIFT)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GenType {
    /// Moves without capturing.
    NonCaptures,
    /// Moves with capturing.
    Captures,
}

/// Generates normal moves.
fn generate_move_normal(board: &Board, moves: &mut Vec<Move>, target: Bitboard) {
    for pt in PieceType::iter() {
        if pt == PieceType::None {
            continue;
        }
        foreach_bb!(board.pieces_pt_side(pt, board.side), sq, {
            let movable_sq = board.movable_sq[pt.into_piece(board.side) as usize][sq as usize];
            foreach_bb!(movable_sq & target, sq2, {
                moves.push(make_move_normal(board.grid[sq2 as usize].split().0, sq, sq2));
            });
        });
    }
    foreach_bb!(board.heavy_attacks(board.side) & target, sq, {
        let from = if board.side == Side::Black { sq as usize - 16 } else { sq as usize + 16 };
        moves.push(make_move_normal(board.grid[sq as usize].pt(), Square::from_usize(from).unwrap(), sq));
    });
}

/// Generates shoot moves.
fn generate_move_shoot(board: &Board, moves: &mut Vec<Move>, target: Bitboard) {
    let bb = board.pieces_pt_side(PieceType::Archer1, board.side)
        | board.pieces_pt_side(PieceType::Archer2, board.side);
    foreach_bb!(bb, sq, {
        foreach_bb!(board.arrow_attacks(sq) & target, sq2, {
            moves.push(make_move_shoot(board.grid[sq2 as usize].split().0, sq, sq2));
        });
    });
}

/// Generates return moves.
fn generate_move_return(board: &Board, moves: &mut Vec<Move>) {
    let bb = board.pieces_pt_side(PieceType::Archer0, board.side)
        | board.pieces_pt_side(PieceType::Archer1, board.side);
    foreach_bb!(board.pieces_pt_side(PieceType::Arrow, board.side), sq, {
        foreach_bb!(board.arrow_attacks(sq) & bb, sq2, {
            moves.push(make_move_return(sq, sq2));
        });
    });
}

/// Generates moves without capturing.
fn generate_non_captures(board: &Board, moves: &mut Vec<Move>) {
    let target = !board.pieces();
    generate_move_normal(board, moves, target);
    generate_move_shoot(board, moves, target);
    generate_move_return(board, moves);
}

/// Generates moves with capturing.
fn generate_captures(board: &Board, moves: &mut Vec<Move>) {
    let target = !board.pieces_side(board.side) & board.pieces_side(!board.side);
    generate_move_normal(board, moves, target);
    generate_move_shoot(board, moves, target);
}

/// Generates moves.
pub fn generate(board: &Board, gen: GenType, moves: &mut Vec<Move>) {
    match gen {
        GenType::NonCaptures => generate_non_captures(board, moves),
        GenType::Captures => generate_captures(board, moves),
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::{Board, PieceType, Square}, movegen::{make_move_normal, pretty_move}};

    use super::{generate, GenType};
    #[test]
    fn initial_position() {
        let mut board: Board = "bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB".parse().unwrap();
        let mut moves = Vec::new();
        generate(&board, GenType::NonCaptures, &mut moves);
        moves.sort();
        let answer = "B1 A3,B1 C3,G1 F3,G1 H3,A2 A3,B2 B3,C2 C3,C2 C4,D2 D3,D2 D4,E2 E3,E2 E4,F2 F3,F2 F4,G2 G3,H2 H3";
        assert_eq!(moves.iter().map(|m| pretty_move(*m)).collect::<Vec<String>>().join(","), answer);

        board.do_move(make_move_normal(PieceType::None, Square::B2, Square::B3));

        let mut moves = Vec::new();
        generate(&board, GenType::NonCaptures, &mut moves);
        moves.sort();
        let answer = "A7 A6,B7 B6,C7 C5,C7 C6,D7 D5,D7 D6,E7 E5,E7 E6,F7 F5,F7 F6,G7 G6,H7 H6,B8 A6,B8 C6,G8 F6,G8 H6";
        assert_eq!(moves.iter().map(|m| pretty_move(*m)).collect::<Vec<String>>().join(","), answer);

        board.do_move(make_move_normal(PieceType::None, Square::B7, Square::B6));

        let mut moves = Vec::new();
        generate(&board, GenType::NonCaptures, &mut moves);
        moves.sort();
        let answer = "B1 A3,B1 C3,C1 B2,G1 F3,G1 H3,A2 A3,C2 C3,C2 C4,D2 D3,D2 D4,E2 E3,E2 E4,F2 F3,F2 F4,G2 G3,H2 H3,B3 B4,S A1 B2,S A1 C3,S A1 D4,S A1 E5,S A1 F6";
        assert_eq!(moves.iter().map(|m| pretty_move(*m)).collect::<Vec<String>>().join(","), answer);

        let mut moves = Vec::new();
        generate(&board, GenType::Captures, &mut moves);
        moves.sort();
        let answer = "S A1 G7";
        assert_eq!(moves.iter().map(|m| pretty_move(*m)).collect::<Vec<String>>().join(","), answer);
    }
}
