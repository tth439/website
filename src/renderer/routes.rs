use axum::{
    extract,
    body::{boxed, Full, Body},
    http::{header, HeaderValue, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,    
};
//use chrono::{DateTime, Utc};
use rust_embed::RustEmbed;
use serde::{Serialize, Deserialize};
use crate::templates;

#[derive(RustEmbed)]
#[folder = "posts/"]
#[include = "*.md"]
struct Posts;


#[derive(Eq, PartialEq, Deserialize, Default, Debug, Serialize, Clone)]
struct FrontMatter {
    title: String,
    date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>
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
                    .header(header::CONTENT_TYPE, HeaderValue::from_str(data.mime.as_ref()).unwrap())
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
    render(|buf| {templates::index(buf, &templates::Html(post), "Another poorly written blog")})
}

async fn archive<'a>() -> Html<&'a str> {
   let file_names: Vec<Option<String>> = Posts::iter().map(|file| {
       let file: Vec<&str> = file.split('.').collect();
       if file[0] != "index" {
            return Some(file[0].to_string())
       }
       return None
   }).collect();
   render(|buf| {templates::archive(buf, file_names, "Archive")})
}

//use frontmatter to extract metadata and stuff
async fn blog<'a>(extract::Path(name): extract::Path<String>) -> Html<&'a str> {
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
    render(|buf| {templates::index(buf, &templates::Html(post), "efrgerg")})
}

async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        render(|buf| {templates::error(buf, uri.path())}),
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
