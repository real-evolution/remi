use std::{io, net, pin, task};

use remi_core::io::Connection;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::TcpStream;

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
}

impl AsyncRead for RemiTcpConnection {
    #[inline(always)]
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> task::Poll<io::Result<()>> {
        self.project().stream.poll_read(cx, buf)
    }
}

impl AsyncWrite for RemiTcpConnection {
    #[inline(always)]
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<io::Result<usize>> {
        self.project().stream.poll_write(cx, buf)
    }

    #[inline(always)]
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<io::Result<()>> {
        self.project().stream.poll_flush(cx)
    }

    #[inline(always)]
    fn poll_shutdown(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<io::Result<()>> {
        self.project().stream.poll_shutdown(cx)
    }

    fn poll_write_vectored(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> task::Poll<Result<usize, io::Error>> {
        let buf = bufs
            .iter()
            .find(|b| !b.is_empty())
            .map_or(&[][..], |b| &**b);
        self.poll_write(cx, buf)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.stream.is_write_vectored()
    }
}

impl From<(TcpStream, net::SocketAddr)> for RemiTcpConnection {
    fn from((stream, address): (TcpStream, net::SocketAddr)) -> Self {
        Self { stream, address }
    }
}
