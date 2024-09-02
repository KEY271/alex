// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function post(entry: string, json: any) {
    return fetch("http://127.0.0.1:3001/api/" + entry, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(json)
    });
}

export function get(entry: string) {
    return fetch("http://127.0.0.1:3001/api/" + entry, { method: "GET" });
}
