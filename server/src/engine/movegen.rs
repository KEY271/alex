use num_traits::FromPrimitive;
use strum::IntoEnumIterator;

use crate::{
    engine::util::{
        make_move_drop, make_move_normal, make_move_return, make_move_shoot, make_move_supply,
    },
    foreach_bb,
};

use super::{
    board::Board,
    util::{Bitboard, Move, PieceType, Side, Square},
};

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
