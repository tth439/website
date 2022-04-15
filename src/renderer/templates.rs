use axum::response::Html;

markup::define! {
    Header<'a>(title: Option<&'a str>) {
        header {
            nav {
                a [ href = "/"] { "/home" } " * " 
                a [ href = "/blog"] { "/blog" } " * "
                a [ href = "/contact"] { "/contact" } " * "
            }
            h1 { 
                @if let Some(title_string) = title {
                    @title_string
                }
            }
        }
    }

    Footer(year: u32) {
        footer {
            "(c) " @year
        }
    }

    Layout<'a>(title: &'a str, content: &'a str) {
        @markup::doctype()

        html[lang="en"] {

            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { @title }

                // script [ src = crate::statics::get_index_js(), type="text/javascript", async=""] {}

                // link [ rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()] {}
            }
            body {
                @Header { title: Some(title) }
                main {
                    {markup::raw(content)}
                }
                @Footer { year: 2022 }
            }
            
        }
    }

    ErrorPage() {
        @markup::doctype()

        html[lang="en"] {

            head {
                meta [ charset="utf-8" ] {}
                meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
                meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
                title { "404" }

                // script [ src = crate::statics::get_index_js(), type="text/javascript", async=""] {}

                // link [ rel = "stylesheet", type="text/css" , href = crate::statics::get_index_css()] {}
            }
            body {
                main {
                    p {
                        "404 - Not found"
                    }
                }
                @Footer { year: 2022 }
            }
            
        }
    }

}

pub(crate) fn render_page(title: &str, content: &str) -> Html<String> {
    let layout = Layout {
        title,
        content,
    };
    Html(layout.to_string())
}

pub(crate) fn render_404() -> Html<String> {
    let layout = ErrorPage{};
    Html(layout.to_string())
}