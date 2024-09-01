import { useEffect, useState } from "react";
import Board from "./components/Board";
import { Position } from "./utils/game";

function App() {
    const [count, setCount] = useState(0);
    const [board, setBoard] = useState(new Position());
    useEffect(() => {
        fetch("http://127.0.0.1:3001/api/board", { method: "GET" })
            .then((res) => res.text())
            .then((data) => {
                const position = new Position();
                position.load(data);
                setBoard(position);
            });
    }, [count]);

    const reset = () => {
        fetch("http://127.0.0.1:3001/api/board", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ mfen: "bngpkgnb/llhhhhll/8/8/8/8/LLHHHHLL/BNGPKGNB b - 0 0" })
        }).then(() => {
            setCount((c) => c + 1);
        });
    };

    const getBestMove = () => {
        fetch("http://127.0.0.1:3001/api/bestmove", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ mfen: board.mfen() })
        })
            .then((res) => res.text())
            .then((data) => {
                console.log(data);
                fetch("http://127.0.0.1:3001/api/move", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({ mfen: data })
                }).then(() => {
                    setCount((c) => c + 1);
                });
            });
    };

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
            </div>
        </div>
    );
}

export default App;
