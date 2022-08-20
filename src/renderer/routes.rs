use super::templates::{render_page, ContentType};
use axum::{
    extract,
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,    
};
//use chrono::{DateTime, Utc};
use mime_guess;
use rust_embed::RustEmbed;
use serde::{Serialize, Deserialize};

#[derive(RustEmbed)]
#[folder = "posts/"]
#[include = "*.md"]
struct Posts;

#[derive(RustEmbed)]
#[folder = "ui/dist/"]
struct Assets;

#[derive(Eq, PartialEq, Deserialize, Default, Debug, Serialize, Clone)]
struct FrontMatter {
    title: String,
    date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>
}

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
    let content = ContentType::Page("Another poorly written blog", &post);
    render_page(content)
}

async fn archive() -> Html<String> {
   let file_names: Vec<Option<String>> = Posts::iter().map(|file| {
       let file: Vec<&str> = file.split('.').collect();
       if file[0] != "index" {
            return Some(file[0].to_string())
       }
       return None
   }).collect();
   let content = ContentType::Archive("Archive", file_names);
   render_page(content)
}

//use frontmatter to extract metadata and stuff
async fn blog(extract::Path(name): extract::Path<String>) -> Html<String> {
    use comrak::{markdown_to_html, ComrakOptions};
    let post = match Posts::get(format!("{}.md", name).as_str()) {
        Some(content) => markdown_to_html(
            &String::from_utf8(content.data.to_vec())
                .unwrap()
                .to_string(),
            &ComrakOptions::default(),
        ),
        None => "".to_string(),
    };
    let content = ContentType::Page("Another poorly written blog", &post);
    render_page(content)
}

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        render_page(ContentType::Page("404",  format!("no route for: {}", uri.path()).as_str())),
    )
}

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/blog", get(archive))
        .route("/blog/:slug", get(blog))
        // .route("/me", )
        // .route("/rss",)
        // .layer(TraceLayer::new_for_http())
        .route("/static/*file", get(static_handler))
        .fallback(get(fallback_handler))
}
