use remi_core::{
    edge::{Acceptor, AcceptorState},
    error::RemiError,
};

use std::{io, net, pin, task};
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::connection::RemiTcpConnection;

#[derive(Debug)]
pub struct RemiTcpAcceptor {
    listener: Option<TcpListener>,
    address: net::SocketAddr,
}

impl RemiTcpAcceptor {
    pub fn new(address: net::SocketAddr) -> Self {
        Self {
            listener: None,
            address,
        }
    }

    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        let address = listener.local_addr()?;

        Ok(Self {
            listener: Some(listener),
            address,
        })
    }

    pub async fn start(&mut self) -> io::Result<()> {
        if self.listener.is_some() {
            return Ok(());
        };

        self.listener = Some(TcpListener::bind(self.address).await?);

        Ok(())
    }
}

impl Acceptor for RemiTcpAcceptor {
    type Conn = RemiTcpConnection;
    type Error = RemiError;

    // Required method
    fn poll_accept(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Result<Self::Conn, Self::Error>>> {
        let Some(ref listener) = self.listener else {
            return task::Poll::Ready(None);
        };

        let task::Poll::Ready(item) = listener.poll_accept(cx) else {
            return task::Poll::Pending;
        };

        let item = match item {
            | Ok(item) => Ok(item.into()),
            | Err(e) => {
                self.listener = None;

                Err(e.into())
            }
        };

        task::Poll::Ready(Some(item))
    }

    #[inline(always)]
    fn state(&self) -> AcceptorState {
        if self.listener.is_some() {
            AcceptorState::Running
        } else {
            AcceptorState::Stopped
        }
    }
}
