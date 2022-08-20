use axum::response::Html;

pub enum ContentType<'a> {
    Page(&'a str, &'a str),
    Archive(&'a str, Vec<Option<String>>),
}

markup::define! {
    Head() {
        head {
            meta [ charset="utf-8" ] {}
            base [ href=std::env::var("BASE_URL").unwrap_or("http://localhost:3000".to_string())] {}
            meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
            meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
            title { "☢" }
            script [ src = "static/index.js", type="text/javascript", async="" ] {}
            link [ rel = "stylesheet", type="text/css" , href = "static/index.css" ] {}
        }
    }

    Header<'a>(title: Option<&'a str>) {
        header {
            nav {
                div {
                    h1 {
                        @if let Some(title_string) = title {
                            @title_string
                        }
                    }
                    a [ href = "/"] { "/home" } " * "
                    a [ href = "/blog"] { "/blog" } " * "
                    a [ href = "/contact"] { "/contact" }
                    button [ id = "darkmode-btn" ] { "☀" }
                }
            }
        }
    }

    Footer(year: u32) {
        footer {
            "Copyright (c) " @year
        }
    }

    Layout<'a>(page: ContentType<'a>) {
        @markup::doctype()
        html[lang="en"] {
            @Head{}
            body {
                @match &page {
                    ContentType::Page(ref title, _) | ContentType::Archive(ref title, _) =>  {
                       @Header {title: Some(title) }
                    }
                }
                main {
                    @match &page {
                        ContentType::Page(_, content) => {
                            p {
                                @markup::raw(content)
                            }
                        }
                        ContentType::Archive(_, posts) =>  {
                            ul {
                                @for p in posts.iter() {
                                        @if let Some(slug) =  p {
                                           li {
                                               a [ href = format!("/blog/{}", slug) ] { @markup::raw(slug) }
                                           } 
                                        }
                                }
                            }
                        }
                    }
                }
                hr {}
                @Footer { year: 2022 }
            }
        }
    }

    ErrorPage<'a>(uri: &'a str) {
        @markup::doctype()
        html[lang="en"] {
            @Head{}

            body {
                main {
                    p { "404 - No route for: " @uri }
                }
                hr {}
                @Footer { year: 2022 }
            }
        }
    }
}

pub(crate) fn render_page<'a>(content: ContentType) -> Html<String> {
    let layout = Layout {page: content};
    Html(layout.to_string())
}
