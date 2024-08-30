import { useState } from "react";
import { Position, Side } from "../utils/game";

type BoardProps = {
    position: Position;
    setCount: React.Dispatch<React.SetStateAction<number>>
};

class State {
    selected: number;
    movables: number[];
    history: number[];

    constructor(selected: number, movables: number[], history: number[]) {
        this.selected = selected;
        this.movables = movables;
        this.history = history;
    }
}

function Board(props: BoardProps) {
    const { position, setCount } = props;

    const [state, setState] = useState<State>(new State(-1, [], []));

    const board = Array(64)
        .fill(0)
        .map((_, i) => {
            const iy = Math.floor((63 - i) / 8);
            const ix = i % 8;
            const j = iy * 8 + ix;
            const [name, side] = position.piece(ix, iy);
            const onClick = () => {
                if (state.movables.includes(j)) {
                    const from = position.square(state.selected);
                    const to = position.square(j);
                    fetch("http://127.0.0.1:3001/api/move", {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: JSON.stringify({ mfen: from + to })
                    }).then(() => {
                        setState(new State(-1, [], [state.selected, j]));
                        setCount((c) => c + 1);
                    });
                    return;
                }
                if (side != position.side) {
                    setState(new State(-1, [], []));
                } else {
                    const movables = position.movable(ix, iy);
                    setState(new State(j, movables, []));
                }
            };
            return (
                <div key={j} className="box-border border-b-2 border-r-2 border-black bg-[bisque]">
                    <div
                        onClick={onClick}
                        data-rev={side == Side.White}
                        data-piece={side == position.side || state.movables.includes(j)}
                        data-selected={state.selected == j}
                        data-movable={state.movables.includes(j)}
                        data-history={state.history.includes(j)}
                        className="flex h-full w-full select-none items-center justify-center border-red-500 text-[20px]
                            data-[rev=true]:rotate-180 data-[piece=true]:cursor-pointer data-[selected=true]:border-2
                            data-[movable=true]:bg-[darksalmon] data-[history=true]:bg-[lightsalmon] sm:text-[30px]">
                        {name}
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
    return (
        <>
            <div className="flex flex-col">
                <div
                    data-turn={position.side == Side.White}
                    className="w-full rotate-180 p-2 data-[turn=true]:text-red-600 data-[turn=true]:underline">
                    後手
                </div>
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
                <div
                    data-turn={position.side == Side.Black}
                    className="w-full p-2 data-[turn=true]:text-red-600 data-[turn=true]:underline">
                    先手
                </div>
            </div>
        </>
    );
}

export default Board;
