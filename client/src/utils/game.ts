export const PieceType = {
    None: 0,
    Light: 1,
    Heavy: 2,
    King: 3,
    Prince: 4,
    General: 5,
    Knight: 6,
    Arrow: 7,
    Archer0: 8,
    Archer1: 9,
    Archer2: 10
} as const;
export type PieceType = (typeof PieceType)[keyof typeof PieceType];

export const Side = {
    None: 0,
    Black: 1,
    White: 2
} as const;
export type Side = (typeof Side)[keyof typeof Side];

export type Piece = [PieceType, Side];

const get_pt = (c: string) => {
    switch (c) {
        case "L":
        case "l":
            return PieceType.Light;
        case "H":
        case "h":
            return PieceType.Heavy;
        case "K":
        case "k":
            return PieceType.King;
        case "P":
        case "p":
            return PieceType.Prince;
        case "G":
        case "g":
            return PieceType.General;
        case "N":
        case "n":
            return PieceType.Knight;
        case "R":
        case "r":
            return PieceType.Arrow;
        case "A":
        case "a":
            return PieceType.Archer0;
        case "B":
        case "b":
            return PieceType.Archer1;
        case "C":
        case "c":
            return PieceType.Archer2;
    }
    return PieceType.None;
};

export class Position {
    board: Piece[];
    side: Side;
    hand_black: [PieceType, number][];
    hand_white: [PieceType, number][];

    constructor() {
        this.board = Array(64)
            .fill(0)
            .map(() => [PieceType.None, Side.None]);
        this.side = Side.Black;
        this.hand_black = [];
        this.hand_white = [];
    }

    load(mfen: string): void {
        let ix = 0;
        let iy = 7;
        const s = mfen.split(" ");
        if (s.length != 3) {
            throw new Error("invalid mfen.");
        }
        for (const c of s[0]) {
            if (c == "/") {
                if (ix != 8) {
                    throw new Error("invalid row.");
                }
                iy -= 1;
                ix = 0;
                if (iy < 0) {
                    throw new Error("too many rows.");
                }
                continue;
            }
            const i = parseInt(c);
            if (!Number.isNaN(i)) {
                if (1 <= i && i <= 8) {
                    ix += i;
                } else {
                    throw new Error("invalid number.");
                }
                continue;
            }
            const pt = get_pt(c);
            if (pt == PieceType.None) {
                throw new Error("invalid character.");
            }
            if (c.toUpperCase() == c) {
                this.board[iy * 8 + ix] = [pt, Side.Black];
            } else {
                this.board[iy * 8 + ix] = [pt, Side.White];
            }
            ix += 1;
        }
        if (ix != 8 || iy != 0) {
            throw new Error("invalid rows.");
        }
        if (s[1] == "b") {
            this.side = Side.Black;
        } else if (s[1] == "w") {
            this.side = Side.White;
        } else {
            throw new Error("invalid turn.");
        }

        this.hand_black = [];
        this.hand_white = [];
        if (s[2] != "-") {
            let i = 0;
            while (i < s[2].length) {
                const c = s[2][i];
                const pt = get_pt(c);
                if (pt == PieceType.None) {
                    throw new Error("invalid character.");
                }
                i += 1;
                let n = parseInt(s[2][i]);
                if (Number.isNaN(n)) {
                    n = 1;
                } else {
                    i += 1;
                }
                if (c.toUpperCase() == c) {
                    this.hand_black.push([pt, n]);
                } else {
                    this.hand_white.push([pt, n]);
                }
            }
        }
    }

    mfen(): string {
        let res = "";
        for (let iy = 7; iy >= 0; iy--) {
            for (let ix = 0; ix < 8; ) {
                const [pt, side] = this.piece(ix, iy);
                if (side == Side.None) {
                    const x = ix;
                    ix += 1;
                    while (ix < 8) {
                        if (this.piece(ix, iy)[1] != Side.None) {
                            break;
                        }
                        ix += 1;
                    }
                    res += (ix - x).toString();
                    continue;
                }
                res += this.piece_mfen(pt, side);
                ix += 1;
            }
            if (iy > 0) {
                res += "/";
            }
        }
        res += " ";
        res += this.side == Side.Black ? "b" : "w";
        res += " ";
        let hand = "";
        for (const [pt, count] of this.hand_black) {
            hand += this.piece_mfen(pt, Side.Black);
            if (count >= 2) {
                hand += count.toString();
            }
        }
        for (const [pt, count] of this.hand_white) {
            hand += this.piece_mfen(pt, Side.White);
            if (count >= 2) {
                hand += count.toString();
            }
        }
        if (hand == "") {
            hand = "-";
        }
        res += hand;
        return res;
    }

    piece_mfen(pt: PieceType, side: Side) {
        let name = "";
        switch (pt) {
            case PieceType.None:
                name = "";
                break;
            case PieceType.Light:
                name = "l";
                break;
            case PieceType.Heavy:
                name = "h";
                break;
            case PieceType.King:
                name = "k";
                break;
            case PieceType.Prince:
                name = "p";
                break;
            case PieceType.General:
                name = "g";
                break;
            case PieceType.Knight:
                name = "n";
                break;
            case PieceType.Arrow:
                name = "r";
                break;
            case PieceType.Archer0:
                name = "a";
                break;
            case PieceType.Archer1:
                name = "b";
                break;
            case PieceType.Archer2:
                name = "c";
                break;
        }
        return side == Side.Black ? name.toUpperCase() : name;
    }

    piece(ix: number, iy: number): [PieceType, Side] {
        return this.piece_index(iy * 8 + ix);
    }

    piece_index(i: number): [PieceType, Side] {
        const [pt, side] = this.board[i];
        if (side == Side.None) {
            return [PieceType.None, side];
        }
        return [pt, side];
    }

    movable(ix: number, iy: number): number[] {
        const movables: number[] = [];
        const [pt, side] = this.board[iy * 8 + ix];
        const isNone = (x: number, y: number) => {
            if (x < 0 || 8 <= x) return false;
            if (y < 0 || 8 <= y) return false;
            const side2 = this.board[y * 8 + x][1];
            return side2 == Side.None;
        };
        const canMove = (x: number, y: number) => {
            if (x < 0 || 8 <= x) return false;
            if (y < 0 || 8 <= y) return false;
            const side2 = this.board[y * 8 + x][1];
            return side != side2;
        };
        const pushPossible = (x: number, y: number) => {
            if (canMove(x, y)) movables.push(index(x, y));
        };
        const index = (x: number, y: number) => y * 8 + x;
        if (side == Side.None) {
            return movables;
        }
        const dir = side == Side.Black ? 1 : -1;
        switch (pt) {
            case PieceType.None:
                break;
            case PieceType.Light:
                pushPossible(ix, iy + dir);
                if ((side == Side.Black && iy >= 5) || (side == Side.White && iy <= 2)) {
                    pushPossible(ix + 1, iy);
                    pushPossible(ix - 1, iy);
                }
                break;
            case PieceType.Heavy:
                pushPossible(ix, iy + dir);
                if (isNone(ix, iy + dir)) {
                    pushPossible(ix, iy + dir * 2);
                }
                if ((side == Side.Black && iy >= 5) || (side == Side.White && iy <= 2)) {
                    pushPossible(ix + 1, iy);
                    pushPossible(ix - 1, iy);
                }
                break;
            case PieceType.King:
                pushPossible(ix + 1, iy + 1);
                pushPossible(ix + 1, iy);
                pushPossible(ix + 1, iy - 1);
                pushPossible(ix, iy + 1);
                pushPossible(ix, iy - 1);
                pushPossible(ix - 1, iy + 1);
                pushPossible(ix - 1, iy);
                pushPossible(ix - 1, iy - 1);
                break;
            case PieceType.Prince:
                pushPossible(ix + 1, iy + 1);
                pushPossible(ix + 1, iy - 1);
                pushPossible(ix, iy + dir);
                pushPossible(ix - 1, iy + 1);
                pushPossible(ix - 1, iy - 1);
                break;
            case PieceType.General:
                pushPossible(ix + 1, iy + dir);
                pushPossible(ix + 1, iy);
                pushPossible(ix, iy + 1);
                pushPossible(ix, iy - 1);
                pushPossible(ix - 1, iy + dir);
                pushPossible(ix - 1, iy);
                break;
            case PieceType.Knight:
                pushPossible(ix + 2, iy + 1);
                pushPossible(ix + 2, iy - 1);
                pushPossible(ix + 1, iy + 2);
                pushPossible(ix + 1, iy - 2);
                pushPossible(ix - 1, iy + 2);
                pushPossible(ix - 1, iy - 2);
                pushPossible(ix - 2, iy + 1);
                pushPossible(ix - 2, iy - 1);
                break;
            case PieceType.Arrow:
                for (let dx = -1; dx <= 1; dx++) {
                    for (let dy = -1; dy <= 1; dy++) {
                        if (dx == 0 && dy == 0) {
                            continue;
                        }
                        for (let j = 1; j < 8; j++) {
                            const x = ix + dx * j;
                            const y = iy + dy * j;
                            if (!isNone(x, y)) {
                                if (x < 0 || 8 <= x) break;
                                if (y < 0 || 8 <= y) break;
                                const [pt2, side2] = this.board[y * 8 + x];
                                if (side == side2 && (pt2 == PieceType.Archer0 || pt2 == PieceType.Archer1)) {
                                    movables.push(index(x, y));
                                }
                                break;
                            }
                        }
                    }
                }
                break;
            case PieceType.Archer0:
                pushPossible(ix, iy + 1);
                pushPossible(ix, iy - 1);
                pushPossible(ix + 1, iy);
                pushPossible(ix - 1, iy);
                break;
            case PieceType.Archer1:
            case PieceType.Archer2:
                pushPossible(ix, iy + 1);
                pushPossible(ix, iy - 1);
                pushPossible(ix + 1, iy);
                pushPossible(ix - 1, iy);
                for (let dx = -1; dx <= 1; dx++) {
                    for (let dy = -1; dy <= 1; dy++) {
                        if (dx == 0 && dy == 0) {
                            continue;
                        }
                        for (let j = 1; j < 8; j++) {
                            pushPossible(ix + dx * j, iy + dy * j);
                            if (!isNone(ix + dx * j, iy + dy * j)) {
                                break;
                            }
                        }
                    }
                }
                break;
        }
        return movables;
    }

    square(i: number): string {
        const x = i % 8;
        const y = Math.floor(i / 8);
        return String.fromCharCode("A".charCodeAt(0) + x) + (y + 1).toString();
    }
}
