import { useEffect, useState } from "react";

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
            <p className="border">{text}</p>
        </>
    );
}

export default App;
