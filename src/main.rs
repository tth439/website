mod renderer;
use renderer::routes::build_router;
use std::net::SocketAddr;
use tokio::net::UnixListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod domainsocket;
use domainsocket::unix::{ServerAccept, UdsConnectInfo};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use http::{Request, Response};
use tracing::Span;
use std::time::Duration;

#[cfg(unix)]
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = build_router()
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &Span| {
                    tracing::debug!("started {} {}", request.method(), request.uri().path())
                })
                .on_response(|_response: &Response<_>, latency: Duration, _span: &Span| {
                    tracing::debug!("response generated in {:?}", latency)
                })
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        let err_str = format!("something went wrong: {}", error);
                        tracing::debug!(err_str)
                    },
                ),
        );

    match std::env::var("SOCKPATH") {
        Ok(path) => {
            let _ = tokio::fs::remove_file(&path).await;

            let uds = UnixListener::bind(path.clone()).unwrap();
            axum::Server::builder(ServerAccept { uds })
                .serve(app.into_make_service_with_connect_info::<UdsConnectInfo>())
                .await
                .unwrap();
        }
        Err(_) => {
            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            println!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
}

#[cfg(not(unix))]
fn main() {
    println!("This example requires unix")
}
