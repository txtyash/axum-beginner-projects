use axum::{response::Html, routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let routers = Router::new().route("/", get(root));

    Ok(routers.into())
}

async fn root() -> Html<&'static str> {
    include_str!("index.html").into()
}
