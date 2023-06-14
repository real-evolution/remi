use std::{net, pin, task};

use remi_core::{edge::Connection, error::RemiResult};
use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf},
    net::TcpStream,
};

#[derive(Debug)]
#[pin_project::pin_project]
pub struct RemiTcpConnection {
    #[pin]
    stream: TcpStream,
    address: net::SocketAddr,
}

#[remi::async_trait]
impl Connection for RemiTcpConnection {
    type Id = net::SocketAddr;

    fn id(&self) -> Option<Self::Id> {
        Some(self.address)
    }

    async fn close(mut self) -> RemiResult<()> {
        Ok(self.stream.shutdown().await?)
    }
}

impl AsyncRead for RemiTcpConnection {
    #[inline(always)]
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        self.project().stream.poll_read(cx, buf)
    }
}

impl AsyncWrite for RemiTcpConnection {
    #[inline(always)]
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, std::io::Error>> {
        self.project().stream.poll_write(cx, buf)
    }

    #[inline(always)]
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        self.project().stream.poll_flush(cx)
    }

    #[inline(always)]
    fn poll_shutdown(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        self.project().stream.poll_shutdown(cx)
    }
}

impl From<(TcpStream, net::SocketAddr)> for RemiTcpConnection {
    fn from((stream, address): (TcpStream, net::SocketAddr)) -> Self {
        Self { stream, address }
    }
}
