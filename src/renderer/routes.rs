use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri},
};
use super::templates::{render_page, render_404};
use rust_embed::RustEmbed;
use std::fmt;

#[derive(RustEmbed)]
#[folder = "posts/"]
#[include = "*.md"]
struct Posts;

#[derive(Debug)]
pub enum CustomError {
    ParserFailure(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::ParserFailure(ref cause) => write!(f, "Parsing Error: {}", cause),
        }
    }
}

async fn index() -> Html<String> {
    use comrak::{markdown_to_html, ComrakOptions};
    let post = match Posts::get("index.md") {
        Some(content) => markdown_to_html(&String::from_utf8(content.data.to_vec()).unwrap().to_string(), &ComrakOptions::default()),
        None => "".to_string()
    };
    render_page("home", &post)
}

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(index))
        // .route("/blog", )
        // .route("/blog/:slug",)
        // .route("/me", )
        // .layer(TraceLayer::new_for_http())
        .fallback(get(fallback_handler))
}