#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num_traits::FromPrimitive;
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256StarStar;

    use crate::engine::{
        movegen::{is_legal, is_pseudo_legal, GenType, MoveList},
        position::Position,
        util::{bit, move_to_mfen, PieceType, Square, PIECE_TYPE_NB, RANK_NB, SIDE_NB, SQUARE_NB},
    };

    fn count_piece(position: &Position) -> [[usize; PIECE_TYPE_NB]; SIDE_NB] {
        let mut pieces = [[0; PIECE_TYPE_NB]; SIDE_NB];
        for i in 0..SQUARE_NB {
            let (pt, side) = position.grid[i].split();
            if pt == PieceType::None {
                continue;
            }
            pieces[side as usize][pt as usize] += 1;
        }
        pieces
    }

    fn check_grid(position: &Position) -> bool {
        if count_piece(&position) != position.piece_count {
            println!("grid  : {:?}", count_piece(&position));
            println!("pieces: {:?}", position.piece_count);
            println!("Pieces mismatch");
            return false;
        }
        for i in 0..SQUARE_NB {
            let (pt, side) = position.grid[i].split();
            if pt == PieceType::None {
                if bit(position.pieces_side(side), i) != 0 {
                    println!("Sides failed at {}", Square::from_usize(i).unwrap());
                    return false;
                }
                continue;
            }
            if bit(position.pieces_side(side), i) != 1 {
                println!("Sides failed at {}", Square::from_usize(i).unwrap());
                return false;
            }
            if position.piece_list[side as usize][pt as usize][position.index[i] as usize] as usize
                != i
            {
                println!("Piece list failed at {}", Square::from_usize(i).unwrap());
                return false;
            }
            for j in 1..PIECE_TYPE_NB {
                let pt2 = PieceType::from_usize(j).unwrap();
                if pt == pt2 {
                    if bit(position.pieces_pt(pt2), i) != 1 {
                        println!("{}", pt2);
                        println!("Boards failed at {}", Square::from_usize(i).unwrap());
                        return false;
                    }
                } else {
                    if bit(position.pieces_pt(pt2), i) != 0 {
                        println!("{}:{}", pt, pt2);
                        println!("Boards failed at {}", Square::from_usize(i).unwrap());
                        return false;
                    }
                }
            }
        }
        true
    }

    #[allow(dead_code)]
    fn print_index(position: &Position) {
        for iy in (0..RANK_NB).rev() {
            for ix in 0..RANK_NB {
                print!("{} ", position.index[iy * RANK_NB + ix]);
            }
            println!();
        }
        println!();
        for pt in 1..PIECE_TYPE_NB {
            let pt = PieceType::from_usize(pt).unwrap();
            print!("{},Black:", pt);
            for i in &position.piece_list[0][pt as usize] {
                print!("{},", i);
            }
            println!();
            print!("{},White:", pt);
            for i in &position.piece_list[1][pt as usize] {
                print!("{},", i);
            }
            println!();
        }
    }

    fn equals(pos1: &Position, pos2: &Position) -> bool {
        if pos1.side != pos2.side {
            return false;
        }
        if pos1.grid != pos2.grid {
            return false;
        }
        if pos1.boards != pos2.boards {
            return false;
        }
        if pos1.sides != pos2.sides {
            return false;
        }
        if pos1.hands != pos2.hands {
            return false;
        }
        if pos1.demise != pos2.demise {
            return false;
        }
        if pos1.effects != pos2.effects {
            return false;
        }
        if pos1.effects != pos2.effects {
            return false;
        }
        if pos1.piece_count != pos2.piece_count {
            return false;
        }
        true
    }

    fn random_move_once(rng: &mut Xoshiro256StarStar, count: usize) {
        let mut moves = Vec::new();
        let mut position =
            Position::from_str("bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB b - 0 0").unwrap();
        if !check_grid(&position) {
            println!("board: {}", position);
            panic!("Init check failed");
        }
        for i in 1..10001 {
            if position.effects != position.calculate_effects() {
                println!("board: {}", position);
                panic!("Effects failed");
            }

            let mut list = MoveList::new();
            list.generate(&position, GenType::Legal);
            if list.size == 0 {
                println!("Cannot move.");
                break;
            }
            let mut illegal = false;
            for i in 0..list.size {
                let mv = list.at(i).mv;
                if !is_pseudo_legal(&position, mv) {
                    println!("mv: {}", move_to_mfen(mv, position.side));
                    illegal = true;
                }
                if !is_legal(&position, mv) {
                    println!("mv: {}", move_to_mfen(mv, position.side));
                    illegal = true;
                }
            }
            if illegal {
                println!("board: {}", position);
                panic!("illegal move.");
            }
            let index = rng.gen_range(0..list.size);
            let mv = list.at(index).mv;
            let mv_mfen = move_to_mfen(mv, position.side);
            println!("Try {}; {}: {}; ", count, i, mv_mfen);

            let temp = position.clone();
            position.do_move(mv, None);
            if position.is_attacked(position.crown_sq(!position.side), !position.side) {
                println!("board: {}", position);
                panic!("illegal move.");
            }
            if !check_grid(&position) {
                println!("old: {}", temp);
                println!("new: {}", position);
                println!("mv: {}", mv_mfen);
                panic!("Check failed");
            }
            position.undo_move(mv);
            if !equals(&position, &temp) {
                println!("old: {}", temp);
                println!("new: {}", position);
                if temp.effects != position.effects {
                    println!("old effects: {:?}", temp.effects);
                    println!("new effects: {:?}", position.effects);
                }
                if temp.piece_count != position.piece_count {
                    println!("old pieces: {:?}", temp.piece_count);
                    println!("new pieces: {:?}", position.piece_count);
                }
                println!("mv: {}", mv_mfen);
                panic!("Undo failed");
            }
            println!("Success");

            if let Some(last) = moves.last() {
                if rng.gen_ratio(2, 3) {
                    position.do_move(mv, None);
                    moves.push(mv);
                } else {
                    position.undo_move(*last);
                    moves.pop();
                }
            } else {
                position.do_move(mv, None);
            }
        }
    }

    #[test]
    fn random_move() {
        let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(32);
        for i in 1..101 {
            random_move_once(&mut rng, i);
        }
    }
}
