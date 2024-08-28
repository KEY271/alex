use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let static_dir = ServeDir::new("static");
    let app = Router::new().nest_service("/", static_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
