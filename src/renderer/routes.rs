use axum::{
    body::{boxed, Body, Full},
    extract,
    http::{header, HeaderValue, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
//use chrono::{DateTime, Utc};
use super::errors::CustomError;
use crate::templates;
use rust_embed::{EmbeddedFile, RustEmbed};
use serde::{Deserialize, Serialize};
#[derive(RustEmbed)]
#[folder = "posts/"]
#[include = "*.md"]
struct Posts;

#[derive(Eq, PartialEq, Deserialize, Default, Debug, Serialize, Clone)]
struct FrontMatter {
    title: String,
    date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
}

impl std::fmt::Display for FrontMatter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Title => {}, date => {}", self.title, self.date)
    }
}

fn render<F>(f: F) -> Html<&'static str>
where
    F: FnOnce(&mut Vec<u8>) -> Result<(), std::io::Error>,
{
    let mut buf = Vec::new();
    f(&mut buf).expect("Error rendering template");
    let html: String = String::from_utf8_lossy(&buf).into();

    Html(Box::leak(html.into_boxed_str()))
}

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match templates::statics::StaticFile::get(path.as_str()) {
            Some(data) => {
                let body = boxed(Body::from(data.content));
                Response::builder()
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_str(data.mime.as_ref()).unwrap(),
                    )
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

async fn index<'a>() -> Html<&'a str> {
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
    render(|buf| templates::index(buf, &templates::Html(post), "タール"))
}

async fn archive<'a>() -> Html<&'a str> {
    let file_names: Vec<Option<String>> = Posts::iter()
        .map(|file| {
            let file: Vec<&str> = file.split('.').collect();
            if file[0] != "index" {
                return Some(file[0].to_string());
            }
            return None;
        })
        .collect();
    render(|buf| templates::archive(buf, file_names, "記録"))
}

fn parse_frontmatter(file_content: &mut String) -> Result<FrontMatter, Box<dyn std::error::Error>> {
    if file_content.starts_with("---\n") {
        let slice_after_marker = &file_content[4..];
        if let Some(end) = slice_after_marker.find("---\n") {
            let de_frontmatter: Result<FrontMatter, _> =
                serde_yaml::from_str(&slice_after_marker[..end]).map_err(|e| e.into());
            file_content.replace_range(0..end + 4, "");
            return de_frontmatter;
        }
    };

    Err(CustomError::ParserFailure("File does not contain frontmatter".to_string()).into())
}

//use frontmatter to extract metadata and stuff
async fn blog<'a>(extract::Path(name): extract::Path<String>) -> Html<&'a str> {
    use comrak::{markdown_to_html, ComrakOptions};
    let mut frontmatter: FrontMatter;
    let mut title = "untitled";

    let post = match Posts::get(format!("{}.md", name).as_str()) {
        Some(content) => {
            let EmbeddedFile { data, .. } = content;
            let mut data = String::from_utf8(data.to_vec()).unwrap().to_string();
            match parse_frontmatter(&mut data) {
                Ok(fm) => {
                    frontmatter = fm;
                    title = frontmatter.title.as_str();
                }
                Err(err) => (()),
            };
            markdown_to_html(&data, &ComrakOptions::default())
        }
        None => "".to_string(),
    };
    render(|buf| templates::index(buf, &templates::Html(post), title))
}

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        render(|buf| templates::error(buf, uri.path())),
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
