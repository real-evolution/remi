use std::{io, net, task};

use remi_core::io::{Acceptor, AcceptorItem};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

#[derive(Debug)]
pub struct TcpAcceptor(TcpListener);

impl TcpAcceptor {
    #[inline]
    pub async fn bind<A: ToSocketAddrs>(addrs: A) -> io::Result<Self> {
        let inner = TcpListener::bind(addrs).await?;

        Ok(Self(inner))
    }

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
