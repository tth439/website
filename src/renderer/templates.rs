use std::borrow::Cow;

use axum::response::Html;

pub enum ContentType<'a> {
    Page(&'a str, &'a str),
    Archive(&'a str, Vec<String>),
}

markup::define! {
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
            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { "Muh bloog" }
                script [ src = "static/js/main.js", type="text/javascript", async="" ] {}
                link [ rel = "stylesheet", type="text/css" , href = "static/css/main.css" ] {}
            }
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
                                    li { @markup::raw(p) }
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
            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { "404" }
                script [ src = "static/js/main.js", type="text/javascript", async="" ] {}
                link [ rel = "stylesheet", type="text/css" , href = "static/css/main.css" ] {}
            }
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

pub(crate) fn render_page<'a>(title: &'a str, content: &'a str) -> Html<String> {
    let layout = Layout {
        page: ContentType::Page(title, content),
    };
    Html(layout.to_string())
}
