use num_traits::FromPrimitive;

use crate::foreach_bb;

use super::{
    position::Position,
    util::{PieceType, Value, PIECE_TYPE_NB, SQUARE_NB},
};

//                                                 NONE  L    H    K    P    G    N    R    A0   A1   A2
// Value of a piece.
#[rustfmt::skip]
const PIECE_VALUES: [Value; PIECE_TYPE_NB]      = [0,    100, 200, 800, 600, 400, 400, 200, 200, 500, 600];
// Value of a piece in hand.
#[rustfmt::skip]
const PIECE_VALUES_HAND: [Value; PIECE_TYPE_NB] = [0,    100, 200, 0,   0,   400, 400, 250, 200, 0,   0];

// Value of an effect.
const EFFECT: Value = 10;
// Value of an effect on ally.
const OUR_EFFECT: Value = 3;
// Value of an effect on opponent.
const OPP_EFFECT: Value = 5;

/// Returns a static evaluation of the position from the point of view of the side to move.
pub fn eval(position: &Position) -> Value {
    let mut value = 0;
    let our_pieces = position.piece_count[position.side as usize];
    let opp_pieces = position.piece_count[!position.side as usize];
    for i in 1..PIECE_TYPE_NB {
        let pt = PieceType::from_usize(i).unwrap();
        value += PIECE_VALUES[i] * (our_pieces[i] as Value - opp_pieces[i] as Value);
        if pt != PieceType::King
            && pt != PieceType::Prince
            && pt != PieceType::Archer1
            && pt != PieceType::Archer2
        {
            value += PIECE_VALUES_HAND[i] * position.count_hand(position.side, pt) as Value;
            value -= PIECE_VALUES_HAND[i] * position.count_hand(!position.side, pt) as Value;
        }
    }
    let mut our_effects = position.effects[position.side as usize];
    let mut opp_effects = position.effects[!position.side as usize];

    let our_archer: u64 = position.pieces_pt_side(PieceType::Archer1, position.side)
        | position.pieces_pt_side(PieceType::Archer2, position.side);
    let opp_archer: u64 = position.pieces_pt_side(PieceType::Archer1, !position.side)
        | position.pieces_pt_side(PieceType::Archer2, !position.side);
    foreach_bb!(our_archer, sq, {
        foreach_bb!(position.arrow_attacks(sq), sq2, {
            our_effects[sq2 as usize] += 1;
        });
    });
    foreach_bb!(opp_archer, sq, {
        foreach_bb!(position.arrow_attacks(sq), sq2, {
            opp_effects[sq2 as usize] += 1;
        });
    });

    foreach_bb!(position.heavy_attacks(position.side), sq, {
        our_effects[sq as usize] += 1;
    });
    foreach_bb!(position.heavy_attacks(!position.side), sq, {
        opp_effects[sq as usize] += 1;
    });

    for i in 0..SQUARE_NB {
        let (pt, side) = position.grid[i].split();
        let our = our_effects[i] as Value;
        let opp = opp_effects[i] as Value;
        value += EFFECT * our;
        value -= EFFECT * opp;
        if pt != PieceType::None {
            if side == position.side {
                value += OUR_EFFECT * our;
                value -= OPP_EFFECT * opp;
            } else {
                value += OPP_EFFECT * our;
                value -= OUR_EFFECT * opp;
            }
        }
    }

    value
}
