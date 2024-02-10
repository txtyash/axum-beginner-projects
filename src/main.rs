use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let service =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/not_found.html"));
    let routers = Router::new().nest_service("/", service);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, routers).await.unwrap();
}
