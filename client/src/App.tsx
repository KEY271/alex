import { useEffect, useState } from "react";
import Board from "./components/Board";

function App() {
    const [text, setText] = useState("");
    useEffect(() => {
        fetch("http://127.0.0.1:3001/text", { method: "GET" })
            .then((res) => res.text())
            .then((data) => {
                setText(data);
            });
    }, []);

    return (
        <>
            <div className="flex justify-center items-center h-full">
                <Board />
            </div>
        </>
    );
}

export default App;
