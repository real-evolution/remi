use std::{io, net};

use remi_core::{edge::Acceptor, error::RemiResult};
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::connection::RemiTcpConnection;

#[derive(Debug)]
pub struct RemiTcpAcceptor {
    inner: TcpListener,
}

impl RemiTcpAcceptor {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let inner = TcpListener::bind(addr).await?;

        Ok(Self { inner })
    }
}

#[remi::async_trait]
impl Acceptor<RemiTcpConnection> for RemiTcpAcceptor {
    #[inline(always)]
    async fn next(&self) -> RemiResult<RemiTcpConnection> {
        Ok(self.inner.accept().await?.into())
    }
}

impl From<TcpListener> for RemiTcpAcceptor {
    fn from(inner: TcpListener) -> Self {
        Self { inner }
    }
}

impl TryFrom<net::TcpListener> for RemiTcpAcceptor {
    type Error = io::Error;

    fn try_from(inner: net::TcpListener) -> io::Result<Self> {
        Ok(Self {
            inner: TcpListener::from_std(inner)?,
        })
    }
}
