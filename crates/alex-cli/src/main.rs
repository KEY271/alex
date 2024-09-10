use std::str::FromStr;

use alex::{position::Position, search::search, types::move_to_mfen};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{space0, space1, u32},
    combinator::{opt, value},
    multi::separated_list0,
    number::complete::double,
    IResult,
};

enum Command {
    UMI,
    IsReady,
    NewGame,
    Position(String, Vec<String>),
    Go(f64),
    Perft(usize, bool),
}

fn umi(s: &str) -> IResult<&str, Command> {
    let (s, _) = tag("umi")(s)?;
    Ok((s, Command::UMI))
}

fn isready(s: &str) -> IResult<&str, Command> {
    let (s, _) = tag("isready")(s)?;
    Ok((s, Command::IsReady))
}

fn new_game(s: &str) -> IResult<&str, Command> {
    let (s, _) = tag("uminewgame")(s)?;
    Ok((s, Command::NewGame))
}

fn mfen(s: &str) -> IResult<&str, String> {
    let (s, _) = tag("mfen")(s)?;
    let (s, _) = space1(s)?;
    let (s, board) = is_a("12345678/LHKPGNRABClhkpgnrabc")(s)?;
    let (s, _) = space1(s)?;
    let (s, side) = alt((tag("b"), tag("w")))(s)?;
    let (s, _) = space1(s)?;
    let (s, hand) = is_a("12345678-LHKPGNRAlhkpgnra")(s)?;
    let (s, _) = space1(s)?;
    let (s, demise_black) = alt((tag("0"), tag("1"), tag("2")))(s)?;
    let (s, _) = space1(s)?;
    let (s, demise_white) = alt((tag("0"), tag("1"), tag("2")))(s)?;
    Ok((
        s,
        format!(
            "{} {} {} {} {}",
            board, side, hand, demise_black, demise_white
        ),
    ))
}

fn position(s: &str) -> IResult<&str, Command> {
    let startpos = "bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB b - 0 0".to_string();
    let (s, _) = tag("position")(s)?;
    let (s, _) = space1(s)?;
    let (s, mfen) = alt((value(startpos, tag("startpos")), mfen))(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = tag("moves")(s)?;
    let (s, _) = space0(s)?;
    let (s, moves) = separated_list0(space1, is_a("123456789ABCDEFGHS"))(s)?;
    Ok((
        s,
        Command::Position(mfen, moves.iter().map(|s| s.to_string()).collect()),
    ))
}

fn go(s: &str) -> IResult<&str, Command> {
    let (s, _) = tag("go")(s)?;
    let (s, _) = space1(s)?;
    let (s, time) = double(s)?;
    Ok((s, Command::Go(time)))
}

fn perft(s: &str) -> IResult<&str, Command> {
    let (s, _) = tag("perft")(s)?;
    let (s, _) = space1(s)?;
    let (s, depth) = u32(s)?;
    let (s, _) = space0(s)?;
    let (s, debug) = opt(tag("debug"))(s)?;
    Ok((s, Command::Perft(depth as usize, debug.is_some())))
}

fn command(s: &str) -> IResult<&str, Command> {
    alt((umi, isready, new_game, position, go, perft))(s)
}

fn main() {
    let mut position = None;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if let Ok((_, cmd)) = command(input) {
            match cmd {
                Command::UMI => println!("umiok"),
                Command::IsReady => println!("readyok"),
                Command::NewGame => {}
                Command::Position(mfen, moves) => {
                    let mut temp = Position::from_str(&mfen).unwrap();
                    for m in moves {
                        let mv = temp.read_move(m.clone()).unwrap();
                        if temp.is_pseudo_legal(mv) {
                            temp.do_move(mv, None);
                        } else {
                            println!("illegal move: {}", m);
                            break;
                        }
                    }
                    position = Some(temp);
                }
                Command::Go(time) => {
                    if let Some(position) = &mut position {
                        let mv = search(position, time);
                        if let Some(info) = mv {
                            println!("info depth {}", info.depth);
                            println!("info score cp {}", info.value);
                            println!("bestmove {}", move_to_mfen(info.mv, position.side));
                        } else {
                            println!("bestmove resign");
                        }
                    }
                }
                Command::Perft(depth, debug) => {
                    if let Some(position) = &mut position {
                        let nodes = alex::perft::perft(position, depth, debug);
                        println!("nodes: {}", nodes);
                    }
                }
            }
        } else {
            println!("unknown command: {}", input);
        };
    }
}
