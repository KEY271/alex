import { useEffect, useState } from "react";
import Board from "./components/Board";
import { Position } from "./utils/game";

function App() {
    const [board, setBoard] = useState(new Position());
    useEffect(() => {
        fetch("http://127.0.0.1:3001/api/board", { method: "GET" })
            .then((res) => res.text())
            .then((data) => {
                const position = new Position();
                position.load(data);
                setBoard(position);
            });
    }, []);

    return (
        <>
            <div className="flex h-full items-center justify-center">
                <Board position={board} />
            </div>
        </>
    );
}

export default App;
