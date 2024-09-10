use core::fmt;
use std::{str::FromStr, usize};

use num_traits::FromPrimitive;

use crate::{
    bitboard::KG_BITBOARD,
    change_bit, foreach_bb,
    types::{
        count_hand, get_capture, get_from, get_move_type, get_pt, get_to, is_demise,
        make_move_drop, make_move_normal, make_move_return, make_move_shoot, make_move_supply,
        read_file, read_rank, to_hand, Bitboard, Hand, Move, MoveType, Piece, PieceType, Side,
        Square, MOVE_DEMISE, PIECE_TYPE_NB, RANK_NB, SIDE_NB, SQUARE_NB,
    },
};

/// Count of occupation.
pub const OCC_NB: usize = 64;

#[derive(PartialEq, Eq, Clone)]
pub struct StateInfo {
    pub checkers: Bitboard,
    pub blockers_king: Bitboard,
    pub blockers_prince: Bitboard,
    pub check_bb: [Bitboard; PIECE_TYPE_NB],
}

impl StateInfo {
    fn calculate_blockers(position: &Position, sq: Square) -> Bitboard {
        if sq == Square::NONE {
            return 0;
        }
        let mut blockers = 0;
        let x = sq as usize % 8;
        let y = sq as usize / 8;
        if position.side == Side::Black
            && y <= 5
            && position.grid[(y + 2) * RANK_NB + x] == Piece::WHeavy
        {
            change_bit!(blockers, (y + 1) * RANK_NB + x);
        }
        if position.side == Side::White
            && y >= 2
            && position.grid[(y - 2) * RANK_NB + x] == Piece::BHeavy
        {
            change_bit!(blockers, (y - 1) * RANK_NB + x);
        }
        for pt in [PieceType::Archer1, PieceType::Archer2] {
            for side in [Side::Black, Side::White] {
                for sq2 in position.piece_list[!side as usize][pt as usize] {
                    if sq2 == Square::NONE {
                        break;
                    }
                    let line = KG_BITBOARD.between_bb[sq2 as usize][sq as usize];
                    let occ = position.pieces() & line;
                    if occ == 0 {
                        continue;
                    }
                    // Tests whether there is just one piece between an archer and the crown.
                    if occ & (occ - 1) == 0 {
                        blockers |= occ;
                    }
                }
            }
        }
        blockers
    }

    pub fn new(position: &Position, checkers: Bitboard) -> Self {
        let opp_crown = position.crown_sq(!position.side);
        let mut check_bb = [0; PIECE_TYPE_NB];
        for i in 1..PIECE_TYPE_NB {
            let pt = PieceType::from_usize(i).unwrap();
            let p = pt.into_piece(position.side);
            check_bb[i] = KG_BITBOARD.check_bb[p as usize][opp_crown as usize];
            match pt {
                PieceType::Heavy => {
                    let x = i % 8;
                    let y = i / 8;
                    if position.side == Side::Black
                        && y >= 2
                        && position.grid[(y - 2) * RANK_NB + x] == Piece::None
                    {
                        change_bit!(check_bb[i], (y - 2) * RANK_NB + x);
                    }
                    if position.side == Side::White
                        && y <= 5
                        && position.grid[(y + 2) * RANK_NB + x] == Piece::None
                    {
                        change_bit!(check_bb[i], (y + 2) * RANK_NB + x);
                    }
                }
                PieceType::Archer1 | PieceType::Archer2 => {
                    check_bb[i] |= position.arrow_attacks(opp_crown);
                }
                _ => {}
            }
        }

        let our_king = position.piece_list[position.side as usize][PieceType::King as usize][0];
        let our_prince = position.piece_list[position.side as usize][PieceType::Prince as usize][0];

        StateInfo {
            checkers,
            blockers_king: Self::calculate_blockers(position, our_king),
            blockers_prince: Self::calculate_blockers(position, our_prince),
            check_bb,
        }
    }
}

/// Position.
#[derive(PartialEq, Eq, Clone)]
pub struct Position {
    pub side: Side,
    /// Piece at the square.
    pub grid: [Piece; SQUARE_NB],
    /// Bitboards of the piece type.
    pub boards: [Bitboard; PIECE_TYPE_NB],
    /// Bitboard of occupied squares of sides.
    pub sides: [Bitboard; SIDE_NB],
    /// Hands of sides.
    pub hands: [Hand; SIDE_NB],
    /// Count of demise.
    pub demise: [usize; SIDE_NB],
    /// Effect.
    pub effects: [[usize; SQUARE_NB]; SIDE_NB],
    /// Count of piece.
    pub piece_count: [[usize; PIECE_TYPE_NB]; SIDE_NB],
    /// Square of piece type.
    pub piece_list: [[[Square; 8]; PIECE_TYPE_NB]; SIDE_NB],
    /// Index of piece.
    pub index: [usize; SQUARE_NB],
    /// Stack of StateInfo
    pub states: Vec<StateInfo>,
}

impl Position {
    /// Create an empty board.
    pub fn new() -> Position {
        Position {
            side: Side::Black,
            grid: [Piece::None; SQUARE_NB],
            boards: [0; PIECE_TYPE_NB],
            sides: [0, 0],
            hands: [0, 0],
            demise: [0, 0],
            effects: [[0; SQUARE_NB]; SIDE_NB],
            piece_count: [[0; PIECE_TYPE_NB]; SIDE_NB],
            piece_list: [[[Square::NONE; 8]; PIECE_TYPE_NB]; SIDE_NB],
            index: [8; SQUARE_NB],
            states: Vec::new(),
        }
    }

    pub fn pieces(&self) -> Bitboard {
        self.sides[Side::Black as usize] | self.sides[Side::White as usize]
    }

    #[allow(dead_code)]
    pub fn pieces_pt(&self, pt: PieceType) -> Bitboard {
        self.boards[pt as usize]
    }

    pub fn pieces_side(&self, side: Side) -> Bitboard {
        self.sides[side as usize]
    }

    pub fn pieces_pt_side(&self, pt: PieceType, side: Side) -> Bitboard {
        self.boards[pt as usize] & self.sides[side as usize]
    }

    pub fn heavy_attacks(&self, side: Side) -> Bitboard {
        KG_BITBOARD.heavy_attacks(
            self.pieces_pt_side(PieceType::Heavy, side),
            self.pieces(),
            side,
        )
    }

    pub fn arrow_attacks(&self, sq: Square) -> Bitboard {
        KG_BITBOARD.arrow_attacks(self.pieces(), sq)
    }

    pub fn calculate_effects(&self) -> [[usize; SQUARE_NB]; SIDE_NB] {
        let mut effects = [[0; SQUARE_NB]; SIDE_NB];
        for i in 0..SQUARE_NB {
            foreach_bb!(KG_BITBOARD.movable_sq[self.grid[i] as usize][i], sq2, {
                effects[self.grid[i].side() as usize][sq2 as usize] += 1;
            });
        }
        effects
    }

    fn add_effect(&mut self, sq: Square, p: Piece) {
        foreach_bb!(KG_BITBOARD.movable_sq[p as usize][sq as usize], sq2, {
            self.effects[p.side() as usize][sq2 as usize] += 1;
        });
    }

    fn remove_effect(&mut self, sq: Square, p: Piece) {
        foreach_bb!(KG_BITBOARD.movable_sq[p as usize][sq as usize], sq2, {
            self.effects[p.side() as usize][sq2 as usize] -= 1;
        });
    }

    pub fn count_hand(&self, side: Side, pt: PieceType) -> u32 {
        count_hand(self.hands[side as usize], pt)
    }

    pub fn add_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] += to_hand(pt);
    }

    pub fn remove_hand(&mut self, side: Side, pt: PieceType) {
        self.hands[side as usize] -= to_hand(pt);
    }

    fn add_piece(&mut self, pt: PieceType, side: Side, sq: Square) {
        change_bit!(self.boards[pt as usize], sq as usize);
        change_bit!(self.sides[side as usize], sq as usize);
        let p = pt.into_piece(side);
        self.grid[sq as usize] = p;
        self.add_effect(sq, p);
        let count = self.piece_count[side as usize][pt as usize];
        self.piece_list[side as usize][pt as usize][count] = sq;
        self.piece_count[side as usize][pt as usize] += 1;
        self.index[sq as usize] = count;
    }

    fn remove_piece(&mut self, sq: Square) {
        let p = self.grid[sq as usize];
        let (pt, side) = p.split();
        change_bit!(self.boards[pt as usize], sq as usize);
        change_bit!(self.sides[side as usize], sq as usize);
        self.remove_effect(sq, p);
        let count = self.piece_count[side as usize][pt as usize];
        let index = self.index[sq as usize];
        self.piece_list[side as usize][pt as usize][index] =
            self.piece_list[side as usize][pt as usize][count - 1];
        self.index[self.piece_list[side as usize][pt as usize][count - 1] as usize] = index;
        self.piece_list[side as usize][pt as usize][count - 1] = Square::NONE;
        self.piece_count[side as usize][pt as usize] -= 1;
        self.index[sq as usize] = 8;
        self.grid[sq as usize] = Piece::None;
    }

    fn move_piece(&mut self, from: Square, to: Square) {
        let p = self.grid[from as usize];
        let (pt, side) = p.split();
        change_bit!(self.boards[pt as usize], from as usize);
        change_bit!(self.sides[side as usize], from as usize);
        change_bit!(self.boards[pt as usize], to as usize);
        change_bit!(self.sides[side as usize], to as usize);
        self.grid[from as usize] = Piece::None;
        self.grid[to as usize] = p;
        self.remove_effect(from, p);
        self.add_effect(to, p);
        self.index[to as usize] = self.index[from as usize];
        self.index[from as usize] = 8;
        self.piece_list[side as usize][pt as usize][self.index[to as usize]] = to;
    }

    pub fn do_move(&mut self, m: Move, checkers: Option<Bitboard>) {
        if is_demise(m) {
            self.demise[self.side as usize] += 1;
            if m == MOVE_DEMISE {
                return;
            }
        }
        let to = get_to(m);
        match get_move_type(m) {
            MoveType::Normal => {
                let from = get_from(m);
                let cap = get_capture(m);

                if cap != PieceType::None {
                    self.remove_piece(to);
                    self.add_hand(self.side, cap);
                }
                self.move_piece(from, to);
            }
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                self.remove_piece(from);
                self.remove_piece(to);
                if pt == PieceType::Archer0 {
                    self.add_piece(PieceType::Archer1, side, to);
                } else if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer2, side, to);
                }
            }
            MoveType::Shoot => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let cap = get_capture(m);

                if cap != PieceType::None {
                    self.remove_piece(to);
                    self.add_hand(side, cap);
                }

                self.remove_piece(from);
                if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer0, side, from);
                } else if pt == PieceType::Archer2 {
                    self.add_piece(PieceType::Archer1, side, from);
                }
                self.add_piece(PieceType::Arrow, side, to);
            }
            MoveType::Drop => {
                let pt = get_pt(m);
                self.remove_hand(self.side, pt);
                self.add_piece(pt, self.side, to);
            }
            MoveType::Supply => {
                self.remove_hand(self.side, PieceType::Arrow);
                let pt = self.grid[to as usize].pt();
                self.remove_piece(to);
                if pt == PieceType::Archer0 {
                    self.add_piece(PieceType::Archer1, self.side, to);
                } else if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer2, self.side, to);
                }
            }
        }

        self.side = !self.side;

        if let Some(checkers) = checkers {
            self.states.push(StateInfo::new(self, checkers));
        } else {
            self.states
                .push(StateInfo::new(self, self.calculate_checkers()));
        }
    }

    pub fn undo_move(&mut self, m: Move) {
        if is_demise(m) {
            self.demise[!self.side as usize] -= 1;
            if m == MOVE_DEMISE {
                return;
            }
        }
        // Change side in advance.
        self.side = !self.side;

        let to = get_to(m);
        let side = self.side;
        match get_move_type(m) {
            MoveType::Normal => {
                let from = get_from(m);
                let cap = get_capture(m);

                self.move_piece(to, from);
                if cap != PieceType::None {
                    self.remove_hand(side, cap);
                    self.add_piece(cap, !side, to);
                }
            }
            MoveType::Return => {
                let from = get_from(m);
                let (pt, side) = self.grid[to as usize].split();

                self.add_piece(PieceType::Arrow, side, from);
                self.remove_piece(to);
                if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer0, side, to);
                } else if pt == PieceType::Archer2 {
                    self.add_piece(PieceType::Archer1, side, to);
                }
            }
            MoveType::Shoot => {
                let from = get_from(m);
                let (pt, side) = self.grid[from as usize].split();
                let cap = get_capture(m);

                self.remove_piece(to);
                if cap != PieceType::None {
                    self.remove_hand(side, cap);
                    self.add_piece(cap, !side, to);
                }
                self.remove_piece(from);
                if pt == PieceType::Archer0 {
                    self.add_piece(PieceType::Archer1, side, from);
                } else if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer2, side, from);
                }
            }
            MoveType::Drop => {
                let pt = get_pt(m);

                self.add_hand(side, pt);
                self.remove_piece(to);
            }
            MoveType::Supply => {
                let pt = self.grid[to as usize].pt();

                self.add_hand(side, PieceType::Arrow);
                self.remove_piece(to);
                if pt == PieceType::Archer1 {
                    self.add_piece(PieceType::Archer0, side, to);
                } else if pt == PieceType::Archer2 {
                    self.add_piece(PieceType::Archer1, side, to);
                }
            }
        }

        self.states.pop();
    }

    /// Make a move from mfen.
    pub fn read_move(&self, mfen: String) -> Result<Move, String> {
        if mfen == "D" {
            return Ok(MOVE_DEMISE);
        }
        let mut len = mfen.len();
        let mut demise = 0;
        if mfen.ends_with("D") {
            len -= 1;
            demise = MOVE_DEMISE;
        }
        let mfen = mfen.as_bytes();
        if len == 4 || len == 5 {
            let x1 = read_file(mfen[0])?;
            let y1 = read_rank(mfen[1])?;
            let from = Square::from_usize(y1 * RANK_NB + x1).unwrap();
            let x2 = read_file(mfen[2])?;
            let y2 = read_rank(mfen[3])?;
            let to = Square::from_usize(y2 * RANK_NB + x2).unwrap();
            let cap = self.grid[to as usize];
            if len == 5 {
                if mfen[4] == b'S' {
                    Ok(make_move_shoot(cap.pt(), from, to) | demise)
                } else {
                    Err("Invalid end character.".to_string())
                }
            } else {
                if cap.pt() != PieceType::None && cap.side() == self.side {
                    Ok(make_move_return(from, to) | demise)
                } else {
                    Ok(make_move_normal(cap.pt(), from, to) | demise)
                }
            }
        } else if len == 3 {
            let x = read_file(mfen[0])?;
            let y = read_rank(mfen[1])?;
            let to = Square::from_usize(y * RANK_NB + x).unwrap();
            let pt = PieceType::from_char(mfen[2]);
            let to_pt = self.grid[to as usize].pt();
            if to_pt == PieceType::Archer0 || to_pt == PieceType::Archer1 {
                Ok(make_move_supply(to) | demise)
            } else {
                Ok(make_move_drop(pt, to) | demise)
            }
        } else {
            Err("Invalid length.".to_string())
        }
    }

    pub fn crown_sq(&self, side: Side) -> Square {
        if self.demise[side as usize] % 2 == 0 {
            self.piece_list[side as usize][PieceType::King as usize][0]
        } else {
            self.piece_list[side as usize][PieceType::Prince as usize][0]
        }
    }

    pub fn checkers(&self) -> Bitboard {
        self.states.last().unwrap().checkers
    }

    pub fn blockers_king(&self) -> Bitboard {
        self.states.last().unwrap().blockers_king
    }

    pub fn blockers_prince(&self) -> Bitboard {
        self.states.last().unwrap().blockers_prince
    }

    pub fn aligned(&self, sq1: Square, sq2: Square, sq3: Square) -> bool {
        KG_BITBOARD.line_bb[sq1 as usize][sq2 as usize] & (1 << sq3 as usize) != 0
    }

    pub fn calculate_checkers(&self) -> Bitboard {
        let crown: Square = self.crown_sq(self.side);
        let mut checkers = 0;
        for i in 0..SQUARE_NB {
            let p = self.grid[i];
            if p == Piece::None || p.side() == self.side {
                continue;
            }
            foreach_bb!(KG_BITBOARD.movable_sq[p as usize][i], sq2, {
                if sq2 == crown {
                    checkers |= 1 << i;
                }
            });
        }

        if self.heavy_attacks(!self.side) & (1 << crown as usize) != 0 {
            if self.side == Side::Black {
                checkers |= 1 << (crown as usize + RANK_NB * 2);
            } else {
                checkers |= 1 << (crown as usize - RANK_NB * 2);
            }
        }
        let archer = self.pieces_pt_side(PieceType::Archer1, !self.side)
            | self.pieces_pt_side(PieceType::Archer2, !self.side);
        foreach_bb!(archer, sq, {
            let attacks = self.arrow_attacks(sq);
            if attacks & (1 << crown as usize) != 0 {
                checkers |= 1 << sq as usize;
            }
        });
        checkers
    }

    pub fn is_attacked(&self, sq: Square, side: Side) -> bool {
        if self.effects[!side as usize][sq as usize] != 0 {
            return true;
        }

        if self.heavy_attacks(!side) & (1 << sq as usize) != 0 {
            return true;
        }
        let archer = self.pieces_pt_side(PieceType::Archer1, !side)
            | self.pieces_pt_side(PieceType::Archer2, !side);
        foreach_bb!(archer, sq2, {
            let attacks = self.arrow_attacks(sq2);
            if attacks & (1 << sq as usize) != 0 {
                return true;
            }
        });
        false
    }
}

impl Piece {
    fn from_char(c: char) -> Result<Self, String> {
        let p = match c {
            'L' => Piece::BLight,
            'H' => Piece::BHeavy,
            'K' => Piece::BKing,
            'P' => Piece::BPrince,
            'G' => Piece::BGeneral,
            'N' => Piece::BKnight,
            'R' => Piece::BArrow,
            'A' => Piece::BArcher0,
            'B' => Piece::BArcher1,
            'C' => Piece::BArcher2,
            'l' => Piece::WLight,
            'h' => Piece::WHeavy,
            'k' => Piece::WKing,
            'p' => Piece::WPrince,
            'g' => Piece::WGeneral,
            'n' => Piece::WKnight,
            'r' => Piece::WArrow,
            'a' => Piece::WArcher0,
            'b' => Piece::WArcher1,
            'c' => Piece::WArcher2,
            _ => return Err(format!("invalid char: {}.", c)),
        };
        Ok(p)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for iy in (0..RANK_NB).rev() {
            let mut ix = 0;
            while ix < RANK_NB {
                let piece = self.grid[iy * RANK_NB + ix];
                if piece == Piece::None {
                    let x = ix;
                    ix += 1;
                    while ix < RANK_NB {
                        if self.grid[iy * RANK_NB + ix] == Piece::None {
                            ix += 1;
                        } else {
                            break;
                        }
                    }
                    write!(f, "{}", ix - x)?;
                } else {
                    ix += 1;
                    write!(f, "{}", piece)?;
                }
            }
            if iy > 0 {
                write!(f, "/")?;
            }
        }

        write!(f, " {} ", if self.side == Side::Black { 'b' } else { 'w' })?;

        if self.hands[0] == 0 && self.hands[1] == 0 {
            write!(f, "-")?;
        } else {
            let pts = [
                PieceType::Light,
                PieceType::Heavy,
                PieceType::General,
                PieceType::Knight,
                PieceType::Arrow,
                PieceType::Archer0,
            ];
            for pt in pts {
                let count = self.count_hand(Side::Black, pt);
                let piece = pt.into_piece(Side::Black);
                if count > 0 {
                    write!(f, "{}", piece)?;
                }
                if count > 1 {
                    write!(f, "{}", count)?;
                }
            }
            for pt in pts {
                let count = self.count_hand(Side::White, pt);
                let piece = pt.into_piece(Side::White);
                if count > 0 {
                    write!(f, "{}", piece)?;
                }
                if count > 1 {
                    write!(f, "{}", count)?;
                }
            }
        }

        write!(f, " {}", self.demise[0])?;
        write!(f, " {}", self.demise[1])?;
        Ok(())
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut position = Position::new();
        let mut ix = 0;
        let mut iy = RANK_NB - 1;
        let s: Vec<&str> = s.split(" ").collect();
        if s.len() != 5 {
            return Err("invalid mfen.".to_string());
        }
        for c in s[0].chars() {
            let piece = match c {
                '/' => {
                    if ix != RANK_NB {
                        return Err("invalid row.".to_string());
                    }
                    ix = 0;
                    if iy == 0 {
                        return Err("too many rows.".to_string());
                    }
                    iy -= 1;
                    continue;
                }
                c => {
                    if let Ok(p) = Piece::from_char(c) {
                        p
                    } else {
                        let i = c as i32 - 48;
                        if i < 0 || ix + i as usize > RANK_NB {
                            return Err(format!("invalid char: {}.", c));
                        }
                        ix += i as usize;
                        continue;
                    }
                }
            };
            let i = iy * RANK_NB + ix;
            let (pt, side) = piece.split();
            if pt != PieceType::None {
                position.add_piece(pt, side, Square::from_usize(i).unwrap());
            }
            ix += 1;
        }
        if ix != RANK_NB || iy != 0 {
            return Err("invalid number.".to_string());
        }

        if s[1] == "b" {
            position.side = Side::Black;
        } else if s[1] == "w" {
            position.side = Side::White;
        } else {
            return Err("invalid turn.".to_string());
        }

        if s[2] != "-" {
            let hand: Vec<char> = s[2].chars().collect();
            let mut i = 0;
            while i < hand.len() {
                let p = Piece::from_char(hand[i])?;
                i += 1;
                if i >= hand.len() || Piece::from_char(hand[i]).is_ok() {
                    position.add_hand(p.side(), p.pt());
                    break;
                }
                let count = hand[i] as i32 - 48;
                if count <= 1 {
                    return Err(format!("invalid char: {}.", hand[i]));
                }
                i += 1;
                for _ in 0..count {
                    position.add_hand(p.side(), p.pt());
                }
            }
        }

        if let Ok(count) = s[3].parse() {
            position.demise[0] = count;
        } else {
            return Err(format!("invalid demise: {}", s[3]));
        }

        if let Ok(count) = s[4].parse() {
            position.demise[1] = count;
        } else {
            return Err(format!("invalid demise: {}", s[4]));
        }

        position.effects = position.calculate_effects();

        position
            .states
            .push(StateInfo::new(&position, position.calculate_checkers()));

        Ok(position)
    }
}
