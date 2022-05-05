use super::templates::render_page;
use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use mime_guess;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "posts/"]
#[include = "*.md"]
struct Posts;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Assets::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.starts_with("static/") {
        path = path.replace("static/", "");
    }
    StaticFile(path)
}

async fn index() -> Html<String> {
    use comrak::{markdown_to_html, ComrakOptions};
    let post = match Posts::get("index.md") {
        Some(content) => markdown_to_html(
            &String::from_utf8(content.data.to_vec())
                .unwrap()
                .to_string(),
            &ComrakOptions::default(),
        ),
        None => "".to_string(),
    };
    render_page("Another poorly written blog", &post)
}

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        render_page("404", format!("no route for: {}", uri.path()).as_str()),
    )
}

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(index))
        // .route("/blog", )
        // .route("/blog/:slug",)
        // .route("/me", )
        // .route("/rss",)
        // .layer(TraceLayer::new_for_http())
        .route("/static/*file", get(static_handler))
        .fallback(get(fallback_handler))
}
