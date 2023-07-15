use std::{io, net, task};

use remi_core::io::{Acceptor, AcceptorItem};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

/// An acceptor type to accept TCP connections.
#[derive(Debug)]
pub struct TcpAcceptor(TcpListener);

impl TcpAcceptor {
    /// Creates a new [`TcpAcceptor`] with the given address.
    ///
    /// # Parameters
    /// * `addrs` - The address to bind to.
    ///
    /// # Returns
    /// A [`TcpAcceptor`] instance bound to `addrs`.
    #[inline]
    pub async fn bind<A: ToSocketAddrs>(addrs: A) -> io::Result<Self> {
        let inner = TcpListener::bind(addrs).await?;

        tracing::info!(addr = ?inner.local_addr(), "TcpAcceptor bound");

        Ok(Self(inner))
    }

    /// Consumes this [`TcpAcceptor`] and returns the inner [`TcpListener`].
    #[inline]
    pub fn into_inner(self) -> TcpListener {
        self.0
    }
}

impl Acceptor for TcpAcceptor {
    type Address = net::SocketAddr;
    type Connection = TcpStream;
    type Error = io::Error;

    fn poll_accept(
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<AcceptorItem<Self>, Self::Error>> {
        let (stream, addr) = task::ready!(self.0.poll_accept(cx))?;

        Ok(AcceptorItem::<Self>::new(stream, addr)).into()
    }
}
