use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::sse::sse_handler;

mod sse;

// const INDEX_HTML: &str = include_str!("../index.html");
const INDEX_HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../chat_server/index.html"
));

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/events", get(sse_handler))
}

pub async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}
