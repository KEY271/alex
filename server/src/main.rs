use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
