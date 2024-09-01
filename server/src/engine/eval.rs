use num_traits::FromPrimitive;

use super::{
    board::Board,
    util::{PieceType, PIECE_TYPE_NB, SQUARE_NB},
};

pub type Value = i16;

// Value of a piece.
const PIECE_VALUES: [Value; PIECE_TYPE_NB] = [0, 100, 200, 800, 600, 400, 400, 400, 400, 800, 1200];

// Value of an effect.
const EFFECT: Value = 20;
// Value of an effect on ally.
const OUR_EFFECT: Value = 5;
// Value of an effect on opponent.
const OPP_EFFECT: Value = 40;

/// Returns a static evaluation of the board from the point of view of the side to move.
pub fn eval(board: &Board) -> Value {
    let mut value = 0;
    let our_pieces = board.pieces[board.side as usize];
    let opp_pieces = board.pieces[!board.side as usize];
    for i in 1..PIECE_TYPE_NB {
        let pt = PieceType::from_usize(i).unwrap();
        value += PIECE_VALUES[i] * (our_pieces[i] as Value - opp_pieces[i] as Value);
        if pt != PieceType::King
            && pt != PieceType::Prince
            && pt != PieceType::Archer1
            && pt != PieceType::Archer2
        {
            value += PIECE_VALUES[i] * board.count_hand(board.side, pt) as Value;
            value -= PIECE_VALUES[i] * board.count_hand(!board.side, pt) as Value;
        }
    }
    let our_effects = board.effects[board.side as usize];
    let opp_effects = board.effects[!board.side as usize];
    let our_arrow_effects = board.calculate_arrow_effect(board.side);
    let opp_arrow_effects = board.calculate_arrow_effect(!board.side);

    for i in 0..SQUARE_NB {
        let (pt, side) = board.grid[i].split();
        let our = (our_effects[i] + our_arrow_effects[i]) as Value;
        let opp = (opp_effects[i] + opp_arrow_effects[i]) as Value;
        value += EFFECT * our;
        value -= EFFECT * opp;
        if pt != PieceType::None {
            if side == board.side {
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
