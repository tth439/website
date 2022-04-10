use askama::Template;
use axum::{
    extract::{rejection::PathRejection, Path},
    http::StatusCode,
    response::{IntoResponse, Headers, Html},
};
use http::header::{HeaderName, HeaderValue};
use percent_encoding::utf8_percent_encode;
extern crate comrak;

#[derive(Template)]
#[template(path = "index.partial.html")]
pub(crate) struct IndexTmpl;

#[derive(Template)]
#[template(path = "blog.partial.html")]
pub(crate) struct BlogTmpl {
    posts: Vec<String>,
}

#[derive(Template)]
#[template(path = "post.partial.html")]
pub(crate) struct BlogPostTmpl<'a> {
    slug: Cow<'static, str>,
    content: Option<Cow<'static, str>>,
}

#[derive(Template)]
#[template(path = "404.html")]
pub(crate) struct NotFoundTmpl;

pub(crate) struct SiteTemplate<T>(pub(crate) T);

pub(crate) impl<'a> BlogPostTmpl<'a> {
    pub(crate) fn new(slug: String) -> Self {
        use std::fs;
        use comrak::{markdown_to_html, ComrakOptions};
        let contents = fs::read_to_string(filename).expect("Couldn't find your blog post");
        Self {slug, content: markdown_to_html(String::from_utf8(contents), &ComrakOptions::default())}
    }
}

impl<T> SiteTemplate<T> where T: Template 
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            ).into_response(),
        }
    }
}

