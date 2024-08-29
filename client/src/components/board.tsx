function Board() {
    const board = Array(64)
        .fill(0)
        .map((_, i) => {
            return (
                <div key={i} className="border-b-2 border-r-2 border-black bg-[bisque] box-border">
                </div>
            );
        });
    const file = [..."abcdefgh"].map((v, i) => {
        return (
            <div key={i} className="flex h-[20px] w-[40px] items-center justify-center sm:w-[60px]">
                {v}
            </div>
        );
    });
    const rank = Array(8)
        .fill(0)
        .map((_, i) => {
            return (
                <div key={i} className="flex h-[40px] w-[20px] items-center justify-center sm:h-[60px]">
                    {i + 1}
                </div>
            );
        });
    return (
        <>
            <div
                className="grid grid-cols-[20px_320px_20px] grid-rows-[20px_320px_20px] sm:grid-cols-[20px_480px_20px]
                    sm:grid-rows-[20px_480px_20px]">
                <div
                    className="col-[2] row-[2] grid grid-cols-[repeat(8,40px)] grid-rows-[repeat(8,40px)] border-l-2
                        border-t-2 w-[322px] sm:w-[482px] h-[322px] sm:h-[482px] border-black sm:grid-cols-[repeat(8,60px)] sm:grid-rows-[repeat(8,60px)] box-border">
                    {board}
                </div>
                <div className="col-[2] row-[3] flex flex-row">{file}</div>
                <div className="col-[2] row-[1] flex rotate-180 flex-row-reverse">{file}</div>
                <div className="col-[1] row-[2] flex flex-col-reverse">{rank}</div>
                <div className="col-[3] row-[2] flex rotate-180 flex-col">{rank}</div>
            </div>
        </>
    );
}

export default Board;
