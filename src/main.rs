use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let service =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/not_found.html"));
    let routers = Router::new().nest_service("/", service);

    Ok(routers.into())
}
