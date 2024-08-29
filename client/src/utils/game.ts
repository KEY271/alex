const PieceType = {
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
type PieceType = (typeof PieceType)[keyof typeof PieceType];

export const Side = {
    None: 0,
    Black: 1,
    White: 2
} as const;
type Side = (typeof Side)[keyof typeof Side];

type Piece = [PieceType, Side];

export class Position {
    board: Piece[];

    constructor() {
        this.board = Array(64)
            .fill(0)
            .map(() => [PieceType.None, Side.None]);
    }

    load(mfen: string): void {
        let ix = 0;
        let iy = 7;
        for (const c of mfen) {
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
            let pt: PieceType = PieceType.None;
            switch (c) {
                case "L":
                case "l":
                    pt = PieceType.Light;
                    break;
                case "H":
                case "h":
                    pt = PieceType.Heavy;
                    break;
                case "K":
                case "k":
                    pt = PieceType.King;
                    break;
                case "P":
                case "p":
                    pt = PieceType.Prince;
                    break;
                case "G":
                case "g":
                    pt = PieceType.General;
                    break;
                case "N":
                case "n":
                    pt = PieceType.Knight;
                    break;
                case "R":
                case "r":
                    pt = PieceType.Arrow;
                    break;
                case "A":
                case "a":
                    pt = PieceType.Archer0;
                    break;
                case "B":
                case "b":
                    pt = PieceType.Archer1;
                    break;
                case "C":
                case "c":
                    pt = PieceType.Archer2;
                    break;
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
    }

    piece(ix: number, iy: number): [string, Side] {
        const [pt, side] = this.board[iy * 8 + ix];
        if (side == Side.None) {
            return ["", side];
        }
        switch (pt) {
            case PieceType.None:
                return ["", side];
            case PieceType.Light:
                return ["歩", side];
            case PieceType.Heavy:
                return ["重", side];
            case PieceType.King:
                return ["玉", side];
            case PieceType.Prince:
                return ["子", side];
            case PieceType.General:
                return ["将", side];
            case PieceType.Knight:
                return ["騎", side];
            case PieceType.Arrow:
                return ["矢", side];
            case PieceType.Archer0:
                return ["弓0", side];
            case PieceType.Archer1:
                return ["弓1", side];
            case PieceType.Archer2:
                return ["弓2", side];
        }
    }

    movable(ix: number, iy: number): number[] {
        const movables: number[] = [];
        const [pt, side] = this.board[iy * 8 + ix];
        const isNone = (x: number, y: number) => {
            if (x < 0 || 8 <= x) return false;
            if (y < 0 || 8 <= y) return false;
            const side2 = this.board[y * 8 + x][1];
            return side2 == Side.None;
        }
        const canMove = (x: number, y: number) => {
            if (x < 0 || 8 <= x) return false;
            if (y < 0 || 8 <= y) return false;
            const side2 = this.board[y * 8 + x][1];
            return side != side2;
        };
        const pushPossible = (x: number, y: number) => {
            if (canMove(x, y)) movables.push(index(x, y));
        }
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
                if (side == Side.Black && iy <= 4 || side == Side.White && iy >= 3) {
                    pushPossible(ix + 1, iy);
                    pushPossible(ix - 1, iy);
                }
                break;
            case PieceType.Heavy:
                pushPossible(ix, iy + dir);
                if (isNone(ix, iy + dir)) {
                    pushPossible(ix, iy + dir * 2);
                }
                if (side == Side.Black && iy <= 4 || side == Side.White && iy >= 3) {
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
                        pushPossible(ix + dx, iy + dy);
                        if (!isNone(ix + dx, iy + dy)) {
                            continue;
                        }
                    }
                }
                break;
        }
        return movables;
    }
}
