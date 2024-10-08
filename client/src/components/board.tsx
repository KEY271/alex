import { useState } from "react";
import { PieceType, Position, Side } from "../utils/game";
import Dialog from "./Dialog";
import Piece from "./Piece";
import { post } from "../utils/connect";
import { useDispatch } from "react-redux";
import { setHistory } from "../utils/slices/board";
import { useSelector } from "../utils/store";

type BoardProps = {
    position: Position;
    setCount: React.Dispatch<React.SetStateAction<number>>;
};

class State {
    selected: number;
    movables: number[];
    side: number;
    hand: number;

    constructor(selected: number, movables: number[], side: number, hand: number) {
        this.selected = selected;
        this.movables = movables;
        this.side = side;
        this.hand = hand;
    }
}

function Board(props: BoardProps) {
    const { position, setCount } = props;

    const dispatch = useDispatch();
    const history = useSelector((state) => state.board.history);

    const [isShootDialogOpen, setShootDialogOpen] = useState(false);
    const [shootAction, setShootAction] = useState<{ fn: (res: boolean) => () => void }>({ fn: () => () => {} });
    const [putCandidates, setPutCandidates] = useState<[string, number][]>([]);
    const [putAction, setPutAction] = useState<{ fn: (res: number) => () => void }>({ fn: () => () => {} });
    const [isDemiseDialogOpen, setDemiseDialogOpen] = useState(false);
    const [isPutDialogOpen, setPutDialogOpen] = useState(false);
    const demiseAction = (res: boolean) => async () => {
        setDemiseDialogOpen(false);
        if (res) {
            await post("move", { mfen: "D" });
            setState(new State(-1, [], Side.None, 0));
            setCount((c) => c + 1);
        }
    };

    const [state, setState] = useState<State>(new State(-1, [], Side.None, 0));

    const board = Array(64)
        .fill(0)
        .map((_, i) => {
            const iy = Math.floor((63 - i) / 8);
            const ix = i % 8;
            const j = iy * 8 + ix;
            const [pt, side] = position.piece(ix, iy);
            let puttable =
                side == Side.None && ((state.side == Side.Black && iy <= 4) || (state.side == Side.White && iy >= 3));
            if (state.side != Side.None) {
                const hand =
                    position.side == Side.Black
                        ? position.hand_black[state.hand][0]
                        : position.hand_white[state.hand][0];
                if (
                    hand == PieceType.Arrow &&
                    side == state.side &&
                    (pt == PieceType.Archer0 || pt == PieceType.Archer1)
                ) {
                    puttable = true;
                }
            }
            const onClick = async () => {
                if (state.movables.includes(j)) {
                    const from = position.square(state.selected);
                    const to = position.square(j);
                    let mfen = from + to;
                    const pt2 = position.piece_index(state.selected)[0];
                    if (pt2 == PieceType.Archer1 || pt2 == PieceType.Archer2) {
                        if (state.movables.filter((v) => v == j).length == 2) {
                            setShootAction({
                                fn: (res: boolean) => async () => {
                                    mfen += res ? "S" : "";
                                    setShootDialogOpen(false);
                                    await post("move", { mfen: mfen });
                                    dispatch(setHistory([state.selected, j]));
                                    setState(new State(-1, [], Side.None, 0));
                                    setCount((c) => c + 1);
                                }
                            });
                            setShootDialogOpen(true);
                            return;
                        }
                        mfen += "S";
                    }
                    await post("move", { mfen: mfen });
                    dispatch(setHistory([state.selected, j]));
                    setState(new State(-1, [], Side.None, 0));
                    setCount((c) => c + 1);
                    return;
                }
                if (puttable) {
                    const to = position.square(j);
                    const hand = position.side == Side.Black ? position.hand_black : position.hand_white;
                    const typ = position.piece_mfen(hand[state.hand][0], position.side);
                    if (hand[state.hand][0] == PieceType.Archer0) {
                        const arrow = hand.filter((v) => v[0] == PieceType.Arrow);
                        if (arrow.length == 1) {
                            if (arrow[0][1] >= 2) {
                                setPutCandidates([
                                    ["0本", 0],
                                    ["1本", 1],
                                    ["2本", 2]
                                ]);
                            } else {
                                setPutCandidates([
                                    ["0本", 0],
                                    ["1本", 1]
                                ]);
                            }
                            setPutAction({
                                fn: (res) => async () => {
                                    setPutDialogOpen(false);
                                    await post("move", { mfen: to + String.fromCharCode(typ.charCodeAt(0) + res) });
                                    dispatch(setHistory([j]));
                                    setState(new State(-1, [], Side.None, 0));
                                    setCount((c) => c + 1);
                                }
                            });
                            setPutDialogOpen(true);
                            return;
                        }
                    }
                    await post("move", { mfen: to + typ });
                    dispatch(setHistory([j]));
                    setState(new State(-1, [], Side.None, 0));
                    setCount((c) => c + 1);
                    return;
                }
                dispatch(setHistory([]));
                if (side != position.side) {
                    setState(new State(-1, [], Side.None, 0));
                } else {
                    const movables = position.movable(ix, iy);
                    setState(new State(j, movables, Side.None, 0));
                }
            };
            const demise = side == Side.None ? 0 : position.demise[side - 1];
            return (
                <div
                    key={j}
                    data-border={iy == 3 || iy == 5}
                    className="box-border border-b-2 border-r-2 border-black bg-[bisque]
                        data-[border=true]:border-b-red-600">
                    <div
                        onClick={onClick}
                        data-rev={side == Side.White}
                        data-piece={side == position.side || state.movables.includes(j) || puttable}
                        data-selected={state.selected == j}
                        data-movable={state.movables.includes(j) || puttable}
                        data-history={history.includes(j)}
                        className="flex h-full w-full select-none items-center justify-center border-red-500
                            data-[rev=true]:rotate-180 data-[piece=true]:cursor-pointer data-[selected=true]:border-2
                            data-[history=true]:bg-[lightsalmon] data-[movable=true]:bg-[darksalmon]">
                        {pt == PieceType.None ? <></> : <Piece pt={pt} demise={demise} />}
                    </div>
                </div>
            );
        });
    const file = [..."ABCDEFGH"].map((v, i) => {
        return (
            <div key={i} className="flex h-[20px] w-[40px] select-none items-center justify-center sm:w-[60px]">
                {v}
            </div>
        );
    });
    const rank = Array(8)
        .fill(0)
        .map((_, i) => {
            return (
                <div key={i} className="flex h-[40px] w-[20px] select-none items-center justify-center sm:h-[60px]">
                    {i + 1}
                </div>
            );
        });
    const hand = (side: Side) => (v: [PieceType, number], i: number) => {
        const [pt, n] = v;
        const count = n == 1 ? "" : n.toString();
        const onClick = () => {
            if (position.side == side) {
                dispatch(setHistory([]));
                setState(new State(-1, [], side, i));
            }
        };
        return (
            <div
                key={i}
                data-turn={position.side == side}
                className="flex select-none data-[turn=true]:cursor-pointer"
                onClick={onClick}>
                <div
                    data-selected={side == state.side && i == state.hand}
                    className="flex h-[40px] w-[40px] items-center justify-center border-2 border-transparent
                        data-[selected=true]:border-red-500 sm:h-[60px] sm:w-[60px]">
                    {pt == PieceType.None ? <></> : <Piece pt={pt} demise={0} />}
                </div>
                <div className="ml-[-4px] text-xs sm:ml-[-8px] sm:text-base">{count}</div>
            </div>
        );
    };
    const hand_black = position.hand_black.map(hand(Side.Black));
    const hand_white = position.hand_white.map(hand(Side.White));
    const indicator = (side: Side) => {
        let disabled = true;
        if (position.side == side) {
            const search = position.demise[side - 1] % 2 == 0 ? PieceType.Prince : PieceType.King;
            for (let i = 0; i < 64; i++) {
                const [pt, s] = position.piece_index(i);
                if (s != side) {
                    continue;
                }
                if (pt == search) {
                    disabled = false;
                    break;
                }
            }
        }
        return (
            <div
                data-rev={side == Side.White}
                className="flex w-full select-none items-center justify-between gap-2 p-2 data-[rev=true]:rotate-180">
                <div
                    data-turn={position.side == side}
                    className="data-[turn=true]:text-red-600 data-[turn=true]:underline">
                    {side == Side.Black ? "先手" : "後手"}
                </div>
                <div>譲位回数: {position.demise[side - 1]}</div>
                <button
                    disabled={disabled}
                    className="border border-black bg-gray-200 p-1 hover:[&:not([disabled])]:bg-[lightsalmon]"
                    onClick={() => {
                        setDemiseDialogOpen(true);
                    }}>
                    譲位
                </button>
            </div>
        );
    };
    return (
        <>
            <Dialog
                text="矢を打ちますか？"
                isOpen={isShootDialogOpen}
                onClose={() => {
                    setShootDialogOpen(false);
                }}
                candidates={[
                    ["はい", true],
                    ["いいえ", false]
                ]}
                action={shootAction.fn}
            />
            <Dialog
                text="何本の矢を装填しますか？"
                isOpen={isPutDialogOpen}
                onClose={() => {
                    setPutDialogOpen(false);
                }}
                candidates={putCandidates}
                action={putAction.fn}
            />
            <Dialog
                text="本当に譲位しますか？"
                isOpen={isDemiseDialogOpen}
                onClose={() => {
                    setDemiseDialogOpen(false);
                }}
                candidates={[
                    ["はい", true],
                    ["いいえ", false]
                ]}
                action={demiseAction}
            />
            <div className="flex flex-col">
                <div className="flex h-[40px] w-full rotate-180 gap-4">{hand_white}</div>
                {indicator(Side.White)}
                <div
                    className="grid grid-cols-[20px_320px_20px] grid-rows-[20px_320px_20px]
                        sm:grid-cols-[20px_480px_20px] sm:grid-rows-[20px_480px_20px]">
                    <div
                        className="col-[2] row-[2] box-border grid h-[322px] w-[322px] grid-cols-[repeat(8,40px)]
                            grid-rows-[repeat(8,40px)] border-l-2 border-t-2 border-black sm:h-[482px] sm:w-[482px]
                            sm:grid-cols-[repeat(8,60px)] sm:grid-rows-[repeat(8,60px)]">
                        {board}
                    </div>
                    <div className="col-[2] row-[3] flex flex-row">{file}</div>
                    <div className="col-[2] row-[1] flex rotate-180 flex-row-reverse">{file}</div>
                    <div className="col-[1] row-[2] flex flex-col-reverse">{rank}</div>
                    <div className="col-[3] row-[2] flex rotate-180 flex-col">{rank}</div>
                </div>
                {indicator(Side.Black)}
                <div className="flex h-[40px] w-full gap-4 text-xl">{hand_black}</div>
            </div>
        </>
    );
}

export default Board;
