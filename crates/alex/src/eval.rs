use std::sync::LazyLock;

use num_traits::FromPrimitive;

use crate::{
    foreach_bb,
    types::{Piece, Side, PIECE_NB},
};

use super::{
    position::Position,
    types::{PieceType, Value, PIECE_TYPE_NB, SQUARE_NB},
};

//                                                 NONE  L    H    K    P    G    N    R    A0   A1   A2
// Value of a piece.
#[rustfmt::skip]
const PIECE_VALUES: [Value; PIECE_TYPE_NB]      = [0,    100, 200, 800, 600, 400, 400, 400, 400, 800, 1200];

const PARAM_OUR_EFFECT_VALUE: i32 = 70;
const PARAM_OPP_EFFECT_VALUE: i32 = 100;
const PARAM_MULTI_EFFECT_VALUE: i32 = 1800;
const PARAM_OUR_EFFECT_PIECE_VALUE_1: i32 = 30;
const PARAM_OUR_EFFECT_PIECE_VALUE_2: i32 = 30;
const PARAM_OPP_EFFECT_PIECE_VALUE_1: i32 = 30;
const PARAM_OPP_EFFECT_PIECE_VALUE_2: i32 = 30;
const PARAM_HAND_PIECE_VALUE: i32 = 200;

static KKPEE: LazyLock<Vec<Value>> = LazyLock::new(|| init_kkpee());

fn index_kkpee(bking: usize, wking: usize, sq: usize, pc: Piece, m1: usize, m2: usize) -> usize {
    bking * SQUARE_NB * SQUARE_NB * PIECE_NB * 3 * 3
        + wking * SQUARE_NB * PIECE_NB * 3 * 3
        + sq * PIECE_NB * 3 * 3
        + pc as usize * 3 * 3
        + m1 * 3
        + m2
}

fn dist(sq1: usize, sq2: usize) -> usize {
    (sq1 / 8).abs_diff(sq2 / 8).max((sq1 % 8).abs_diff(sq2 % 8))
}

fn init_kkpee() -> Vec<Value> {
    let mut our_eff = [0; 9];
    let mut opp_eff = [0; 9];
    for d in 0..9 {
        our_eff[d] = PARAM_OUR_EFFECT_VALUE * 1024 / (d as i32 + 1);
        opp_eff[d] = PARAM_OPP_EFFECT_VALUE * 1024 / (d as i32 + 1);
    }
    let multi_eff = [0, 1024, PARAM_MULTI_EFFECT_VALUE];
    let mut our_eff_table = [[[0.0; 3]; SQUARE_NB]; SQUARE_NB];
    let mut opp_eff_table = [[[0.0; 3]; SQUARE_NB]; SQUARE_NB];
    for king in 0..SQUARE_NB {
        for sq in 0..SQUARE_NB {
            for m in 0..3 {
                let d = dist(king, sq);
                our_eff_table[king][sq][m] =
                    f64::from(our_eff[d] * multi_eff[m]) / (1024.0 * 1024.0);
                opp_eff_table[king][sq][m] =
                    f64::from(opp_eff[d] * multi_eff[m]) / (1024.0 * 1024.0);
            }
        }
    }
    let our_eff_to_piece = [
        0,
        PARAM_OUR_EFFECT_PIECE_VALUE_1,
        PARAM_OUR_EFFECT_PIECE_VALUE_2,
    ];
    let opp_eff_to_piece = [
        0,
        PARAM_OPP_EFFECT_PIECE_VALUE_1,
        PARAM_OPP_EFFECT_PIECE_VALUE_2,
    ];

    let mut kkpee = vec![0; SQUARE_NB * SQUARE_NB * SQUARE_NB * PIECE_NB * 3 * 3];

    for pc in 0..PIECE_NB {
        if Piece::PAD1 as usize <= pc && pc <= Piece::PAD6 as usize {
            continue;
        }
        let pc = Piece::from_usize(pc).unwrap();
        for bking in 0..SQUARE_NB {
            for wking in 0..SQUARE_NB {
                for sq in 0..SQUARE_NB {
                    for m1 in 0..3 {
                        for m2 in 0..3 {
                            let mut score = 0.0;
                            score += our_eff_table[bking][sq][m1];
                            score += opp_eff_table[wking][sq][m1];
                            score -= our_eff_table[wking][sq][m2];
                            score -= opp_eff_table[bking][sq][m2];
                            if pc != Piece::None {
                                let s = PIECE_VALUES[pc.pt() as usize] as i32
                                    * PARAM_HAND_PIECE_VALUE
                                    / 1024;
                                if pc.side() == Side::Black {
                                    score += our_eff_to_piece[m1] as f64;
                                    score -= opp_eff_to_piece[m2] as f64;
                                    score -= s as f64;
                                } else {
                                    score -= our_eff_to_piece[m2] as f64;
                                    score += opp_eff_to_piece[m1] as f64;
                                    score += s as f64;
                                }
                            }
                            kkpee[index_kkpee(bking, wking, sq, pc, m1, m2)] = score as Value;
                        }
                    }
                }
            }
        }
    }

    kkpee
}

const PARAM_DEMISE_VALUE: Value = 400;

/// Returns a static evaluation of the position from the point of view of the side to move.
pub fn eval(position: &Position) -> Value {
    let mut material = 0;
    let our_pieces = position.piece_count[position.side as usize];
    let opp_pieces = position.piece_count[!position.side as usize];
    for i in 1..PIECE_TYPE_NB {
        let pt = PieceType::from_usize(i).unwrap();
        material += PIECE_VALUES[i] * (our_pieces[i] as Value - opp_pieces[i] as Value);
        if pt != PieceType::King
            && pt != PieceType::Prince
            && pt != PieceType::Archer1
            && pt != PieceType::Archer2
        {
            material += PIECE_VALUES[i] * position.count_hand(position.side, pt) as Value;
            material -= PIECE_VALUES[i] * position.count_hand(!position.side, pt) as Value;
        }
    }

    let mut black_effects = position.effects[Side::Black as usize];
    let mut white_effects = position.effects[Side::White as usize];
    let black_archer: u64 = position.pieces_pt_side(PieceType::Archer1, Side::Black)
        | position.pieces_pt_side(PieceType::Archer2, Side::Black);
    let white_archer: u64 = position.pieces_pt_side(PieceType::Archer1, Side::White)
        | position.pieces_pt_side(PieceType::Archer2, Side::White);
    foreach_bb!(black_archer, sq, {
        foreach_bb!(position.arrow_attacks(sq), sq2, {
            black_effects[sq2 as usize] += 1;
        });
    });
    foreach_bb!(white_archer, sq, {
        foreach_bb!(position.arrow_attacks(sq), sq2, {
            white_effects[sq2 as usize] += 1;
        });
    });
    foreach_bb!(position.heavy_attacks(Side::Black), sq, {
        black_effects[sq as usize] += 1;
    });
    foreach_bb!(position.heavy_attacks(Side::White), sq, {
        white_effects[sq as usize] += 1;
    });

    let mut value = 0;
    let bking = position.crown_sq(Side::Black) as usize;
    let wking = position.crown_sq(Side::White) as usize;
    for sq in 0..SQUARE_NB {
        value += KKPEE[index_kkpee(
            bking,
            wking,
            sq,
            position.grid[sq],
            black_effects[sq].min(2),
            white_effects[sq].min(2),
        )];
    }

    value -= position.demise[Side::Black as usize] as Value * PARAM_DEMISE_VALUE;
    value += position.demise[Side::White as usize] as Value * PARAM_DEMISE_VALUE;
    material
        + if position.side == Side::Black {
            value
        } else {
            -value
        }
}
