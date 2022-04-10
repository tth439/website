use axum::{extract::Extension, handler::Handler, routing::get, Router};

async fn index() -> Impl Response {

}

pub fn build_router() -> Router {
    Router::new(
        .route("/", )
        .route("/blog", )
        .route("/blog/:slug",)
        .route("/me", )
        .route("/static/", )
        .layer(TraceLayer::new_for_http())
        .fallback(handle_404.into_service())
}