#[cfg(unix)]
pub(crate) mod unix {
    use axum::{
        extract::connect_info::{self, ConnectInfo},
    };
    use futures::ready;
    use hyper::{
        client::connect::{Connected, Connection},
        server::accept::Accept,
    };
    use std::{
        io,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    };
    use tokio::{
        io::{AsyncRead, AsyncWrite},
        net::{unix::UCred, UnixListener, UnixStream},
    };
    use tower::BoxError;

    pub(crate) struct ServerAccept {
        pub(crate) uds: UnixListener,
    }

    impl Accept for ServerAccept {
        type Conn = UnixStream;
        type Error = BoxError;

        fn poll_accept(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
            let (stream, _addr) = ready!(self.uds.poll_accept(cx))?;
            Poll::Ready(Some(Ok(stream)))
        }
    }

    struct ClientConnection {
        stream: UnixStream,
    }

    impl AsyncWrite for ClientConnection {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, io::Error>> {
            Pin::new(&mut self.stream).poll_write(cx, buf)
        }

        fn poll_flush(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), io::Error>> {
            Pin::new(&mut self.stream).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), io::Error>> {
            Pin::new(&mut self.stream).poll_shutdown(cx)
        }
    }

    impl AsyncRead for ClientConnection {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            Pin::new(&mut self.stream).poll_read(cx, buf)
        }
    }

    impl Connection for ClientConnection {
        fn connected(&self) -> Connected {
            Connected::new()
        }
    }

    #[derive(Clone, Debug)]
    #[allow(dead_code)]
    pub(crate) struct UdsConnectInfo {
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
}
