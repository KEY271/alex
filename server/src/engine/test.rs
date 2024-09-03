#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num_traits::FromPrimitive;
    use rand::{Rng, SeedableRng};

    use crate::engine::{
        board::Board,
        movegen::{is_pseudo_legal, GenType, MoveList},
        util::{bit, move_to_mfen, PieceType, PIECE_TYPE_NB, SIDE_NB, SQUARE_NB},
    };

    fn count_piece(board: &Board) -> [[usize; PIECE_TYPE_NB]; SIDE_NB] {
        let mut pieces = [[0; PIECE_TYPE_NB]; SIDE_NB];
        for i in 0..SQUARE_NB {
            let (pt, side) = board.grid[i].split();
            if pt == PieceType::None {
                continue;
            }
            pieces[side as usize][pt as usize] += 1;
        }
        pieces
    }

    fn check_grid(board: &Board) -> bool {
        let mut exist_king = [false; SIDE_NB];
        let mut exist_prince = [false; SIDE_NB];
        for i in 0..SQUARE_NB {
            let (pt, side) = board.grid[i].split();
            if pt == PieceType::King {
                exist_king[side as usize] = true;
                if board.king_sq[side as usize].is_none()
                    || board.king_sq[side as usize].unwrap() as usize != i
                {
                    println!("King failed at {}.", i);
                }
            }
            if pt == PieceType::Prince {
                exist_prince[side as usize] = true;
                if board.prince_sq[side as usize].is_none()
                    || board.prince_sq[side as usize].unwrap() as usize != i
                {
                    println!("Prince failed at {}.", i);
                }
            }
            for j in 1..PIECE_TYPE_NB {
                let pt2 = PieceType::from_usize(j).unwrap();
                if pt == pt2 {
                    if bit(board.pieces_pt(pt2), i) != 1 {
                        println!("{}", pt2);
                        println!("Boards failed at {}", i);
                        return false;
                    }
                } else {
                    if bit(board.pieces_pt(pt2), i) != 0 {
                        println!("{}:{}", pt, pt2);
                        println!("Boards failed at {}", i);
                        return false;
                    }
                }
                if pt != PieceType::None {
                    if bit(board.pieces_side(side), i) != 1 {
                        println!("Sides failed at {}", i);
                        return false;
                    }
                } else {
                    if bit(board.pieces_side(side), i) != 0 {
                        println!("Sides failed at {}", i);
                        return false;
                    }
                }
            }
        }
        if exist_king[0] != board.king_sq[0].is_some() {
            println!("King failed");
            return false;
        }
        if exist_king[1] != board.king_sq[1].is_some() {
            println!("King failed");
            return false;
        }
        if exist_prince[0] != board.prince_sq[0].is_some() {
            println!("Prince failed");
            return false;
        }
        if exist_prince[1] != board.prince_sq[1].is_some() {
            println!("Prince failed");
            return false;
        }
        true
    }

    #[test]
    fn random_move() {
        let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(32);
        let mut moves = Vec::new();
        let mut board =
            Board::from_str("bngpkgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB b - 0 0").unwrap();
        if !check_grid(&board) {
            println!("board: {}", board);
            panic!("Init check failed");
        }
        for i in 0..100000 {
            if board.effects != board.calculate_effects() {
                println!("board: {}", board);
                panic!("Effects failed");
            }

            let mut list = MoveList::new();
            list.generate(&board, GenType::All);
            if list.size == 0 {
                println!("Cannot move.");
                break;
            }
            let mut illegal = false;
            for i in 0..list.size {
                let mv = list.at(i).mv;
                if !is_pseudo_legal(&board, mv) {
                    println!("mv: {}", move_to_mfen(mv, board.side));
                    illegal = true;
                }
            }
            if illegal {
                println!("board: {}", board);
                panic!("illegal move.");
            }
            let index = rng.gen_range(0..list.size);
            let mv = list.at(index).mv;

            let mv_mfen = move_to_mfen(mv, board.side);
            let temp = board.clone();
            board.do_move(mv);
            if count_piece(&board) != board.pieces {
                println!("old: {}", temp);
                println!("new: {}", board);
                println!("mv: {}", mv_mfen);
                println!("grid  : {:?}", count_piece(&board));
                println!("pieces: {:?}", board.pieces);
                panic!("Pieces mismatch");
            }
            if !check_grid(&board) {
                println!("old: {}", temp);
                println!("new: {}", board);
                println!("mv: {}", mv_mfen);
                panic!("Check failed");
            }
            board.undo_move(mv);
            if board != temp {
                println!("old: {}", temp);
                println!("new: {}", board);
                if temp.effects != board.effects {
                    println!("old effects: {:?}", temp.effects);
                    println!("new effects: {:?}", board.effects);
                }
                if temp.pieces != board.pieces {
                    println!("old pieces: {:?}", temp.pieces);
                    println!("new pieces: {:?}", board.pieces);
                }
                println!("mv: {}", mv_mfen);
                panic!("Undo failed");
            }
            println!("Success {}: {}", i, mv_mfen);

            if let Some(last) = moves.last() {
                if rng.gen_ratio(2, 3) {
                    board.do_move(mv);
                    moves.push(mv);
                } else {
                    board.undo_move(*last);
                    moves.pop();
                }
            } else {
                board.do_move(mv);
            }
        }
    }
}
