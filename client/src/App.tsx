import { useEffect, useRef, useState } from "react";
import Board from "./components/Board";
import { getMoveSquares, Position } from "./utils/game";
import { get, post } from "./utils/connect";
import { setHistory } from "./utils/slices/board";
import { useDispatch } from "react-redux";

function App() {
    const dispatch = useDispatch();

    const [count, setCount] = useState(0);
    const [board, setBoard] = useState(new Position());
    useEffect(() => {
        (async () => {
            const res = await get("board");
            const data = await res.text();
            const position = new Position();
            position.load(data);
            setBoard(position);
        })();
    }, [count]);

    const reset = async () => {
        await post("board", { mfen: "bngkpgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB b - 0 0" });
        dispatch(setHistory([]));
        setCount((c) => c + 1);
    };

    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        if (inputRef.current == undefined) {
            return;
        }
        inputRef.current.value = "3";
    }, [inputRef]);

    const [score, setScore] = useState(0);
    const [depth, setDepth] = useState(0);
    const [rootMoves, setRootMoves] = useState([]);

    const getBestMove = async () => {
        if (inputRef.current == undefined) {
            return;
        }
        const value = Number.parseFloat(inputRef.current.value);
        if (Number.isNaN(value)) {
            alert("考慮時間が数値ではありません。");
            return;
        }
        if (value < 0 || 100 < value) {
            alert("考慮時間が適当ではありません。");
            return;
        }
        setCountdown(value);
        setScore(0);
        setDepth(0);
        setRootMoves([]);
        const res = await post("bestmove", {
            mfen: board.mfen(),
            time: value
        });
        const data = await res.json();
        await post("move", { mfen: data.mfen });
        setScore(data.value);
        setDepth(data.depth);
        const moves = data.root_moves.sort((a: [string, number], b: [string, number]) => b[1] - a[1]);
        setRootMoves(moves);
        setCountdown(0);
        dispatch(setHistory(getMoveSquares(data.mfen)));
        setCount((c) => c + 1);
    };

    const [countdown, setCountdown] = useState(0);
    useEffect(() => {
        if (countdown <= 0) {
            return;
        }
        const id = setInterval(() => {
            setCountdown((c) => c - 0.1);
        }, 100);
        return () => clearInterval(id);
    }, [countdown]);

    const root = (
        <div>
            {rootMoves.map((move: [string, number], i: number) => (
                <div key={i}>
                    {move[0]}:{move[1]}
                </div>
            ))}
        </div>
    );

    return (
        <div className="grid h-full grid-cols-[360px_1fr] sm:grid-cols-[520px_1fr]">
            <div className="flex h-full items-center justify-center border-r border-black">
                <Board position={board} setCount={setCount} />
            </div>
            <div className="flex flex-col gap-2 p-2">
                <button
                    onClick={reset}
                    className="h-12 w-24 border border-black bg-gray-200 p-2 hover:bg-[lightsalmon]">
                    リセット
                </button>
                <button
                    onClick={getBestMove}
                    className="h-12 w-24 border border-black bg-gray-200 p-2 hover:bg-[lightsalmon]">
                    最善手
                </button>
                <label>考慮時間(秒)</label>
                <input ref={inputRef} className="w-16 border text-right" />
                <div>{countdown.toFixed(1)}</div>
                <div>評価値: {score}</div>
                <div>読んだ手: {depth}</div>
                <div className="overflow-y-scroll h-80 border border-black w-24">{root}</div>
            </div>
        </div>
    );
}

export default App;
