use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::foreach_bb;

use super::board::{get_pos, Bitboard, Board, PieceType, Side, Square};

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
                moves.push(make_move_normal(
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
        moves.push(make_move_normal(
            board.grid[sq as usize].pt(),
            Square::from_usize(from).unwrap(),
            sq,
        ));
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

/// Generates drop moves.
fn generate_move_drop(board: &Board, moves: &mut Vec<Move>) {
    let mask = if board.side == Side::Black {
        0x000000FFFFFFFFFF
    } else {
        0xFFFFFFFFFF000000
    };
    let bb = !board.pieces() & mask;

    if board.count_hand(board.side, PieceType::Light) != 0 {
        foreach_bb!(bb, sq, { moves.push(make_move_drop(PieceType::Light, sq)) });
    }
    if board.count_hand(board.side, PieceType::Heavy) != 0 {
        foreach_bb!(bb, sq, { moves.push(make_move_drop(PieceType::Heavy, sq)) });
    }
    if board.count_hand(board.side, PieceType::General) != 0 {
        foreach_bb!(bb, sq, {
            moves.push(make_move_drop(PieceType::General, sq))
        });
    }
    if board.count_hand(board.side, PieceType::Knight) != 0 {
        foreach_bb!(bb, sq, {
            moves.push(make_move_drop(PieceType::Knight, sq))
        });
    }
    let arrow = board.count_hand(board.side, PieceType::Arrow);
    if arrow != 0 {
        foreach_bb!(bb, sq, { moves.push(make_move_drop(PieceType::Arrow, sq)) });
    }
    if board.count_hand(board.side, PieceType::Archer0) != 0 {
        foreach_bb!(bb, sq, {
            moves.push(make_move_drop(PieceType::Archer0, sq));
            if arrow >= 1 {
                moves.push(make_move_drop(PieceType::Archer1, sq));
            }
            if arrow >= 2 {
                moves.push(make_move_drop(PieceType::Archer2, sq));
            }
        });
    }
}

/// Generates supply.
fn generate_move_supply(board: &Board, moves: &mut Vec<Move>) {
    if board.count_hand(board.side, PieceType::Arrow) != 0 {
        let bb = board.pieces_pt_side(PieceType::Archer0, board.side)
            | board.pieces_pt_side(PieceType::Archer1, board.side);
        foreach_bb!(bb, sq, {
            moves.push(make_move_supply(sq));
        });
    }
}

/// Generates moves without capturing.
fn generate_non_captures(board: &Board, moves: &mut Vec<Move>) {
    let target = !board.pieces();
    generate_move_normal(board, moves, target);
    generate_move_shoot(board, moves, target);
    generate_move_return(board, moves);
    generate_move_drop(board, moves);
    generate_move_supply(board, moves);
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
