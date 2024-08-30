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

    piecename(pt: PieceType) {
        switch (pt) {
            case PieceType.None:
                return "";
            case PieceType.Light:
                return "歩";
            case PieceType.Heavy:
                return "重";
            case PieceType.King:
                return "玉";
            case PieceType.Prince:
                return "子";
            case PieceType.General:
                return "将";
            case PieceType.Knight:
                return "騎";
            case PieceType.Arrow:
                return "➶";
            case PieceType.Archer0:
                return "弓0";
            case PieceType.Archer1:
                return "弓1";
            case PieceType.Archer2:
                return "弓2";
        }
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
                            const y = ix + dy * j;
                            if (!isNone(x, y)) {
                                if (x < 0 || 8 <= x) break;
                                if (y < 0 || 8 <= y) break;
                                const pt = this.board[y * 8 + x][0];
                                if (pt == PieceType.Archer0 || pt == PieceType.Archer1) {
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
