use axum::{
    extract::connect_info::{self},
    routing::get,
    Router,
};
use futures::ready;
use hyper::server::accept::Accept;
use std::{
    path::PathBuf,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::net::{unix::UCred, UnixListener, UnixStream};
use tower::BoxError;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct UdsConnectInfo {
    peer_addr: Arc<tokio::net::unix::SocketAddr>,
    peer_cred: UCred,
}

impl connect_info::Connected<&UnixStream> for UdsConnectInfo {
    fn connect_info(target: &UnixStream) -> Self {
        let peer_addr = target.peer_addr().unwrap();
        let peer_cred = target.peer_cred().unwrap();

        Self {
            peer_addr: Arc::new(peer_addr),
            peer_cred,
        }
    }
}
struct ServerAccept {
    uds: UnixListener,
}

impl Accept for ServerAccept {
    type Conn = UnixStream;
    type Error = BoxError;

    fn poll_accept(self: Pin<&mut Self>, cx: &mut Context<'_>)
     -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        let (stream, _addr) = ready!(self.uds.poll_accept(cx))?;
        Poll::Ready(Some(Ok(stream)))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    let path = PathBuf::from("/tmp/blog-site");

    let _ = tokio::fs::remove_file(&path).await;
    tokio::fs::create_dir_all(path.parent().unwrap())
        .await
        .unwrap();

    let uds = UnixListener::bind(path.clone()).unwrap();
    tokio::spawn(async {
        let app = Router::new().route("/", get(handler));

        axum::Server::builder(ServerAccept { uds })
            .serve(app.into_make_service_with_connect_info::<UdsConnectInfo>())
            .await
            .unwrap();
    });
}
