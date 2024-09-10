use crate::{
    movegen::{GenType, MoveList},
    position::Position,
    types::move_to_mfen,
};

pub fn perft(position: &mut Position, depth: usize, debug: bool) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    let mut moves = MoveList::new();
    moves.generate(position, GenType::Legal);
    for i in 0..moves.size {
        let mv = moves.at(i).mv;
        if debug {
            for _ in 0..depth - 1 {
                print!("  ");
            }
            println!("{}", move_to_mfen(mv, position.side));
        }
        position.do_move(mv, None);
        nodes += perft(position, depth - 1, debug);
        position.undo_move(mv);
    }

    nodes
}
