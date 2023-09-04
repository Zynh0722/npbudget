use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use tower_http::services::ServeDir;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .fallback_service(ServeDir::new("dist"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> impl IntoResponse {
    let mut buf = Vec::with_capacity(512);
    templates::hello_args_html(&mut buf, "World").unwrap();
    Html(buf)
}
