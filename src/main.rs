use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let service =
        ServeDir::new("assets/root").not_found_service(ServeFile::new("assets/not_found.html"));
    let routers = Router::new().nest_service("/", service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, routers).await.unwrap();
}
