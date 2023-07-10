use std::io::{self, ErrorKind};
use std::{net, task};

use remi_core::error::RemiError;
use remi_core::io::{Acceptor, AcceptorState};
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
        &mut self,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<Self::Conn, Self::Error>> {
        let Some(ref listener) = self.listener else {
            return task::Poll::Ready(Err(io::Error::new(
                ErrorKind::NotConnected,
                "listener not started",
            )
            .into()));
        };

        let task::Poll::Ready(item) = listener.poll_accept(cx) else {
            return task::Poll::Pending;
        };

        let item = match item {
            | Ok(item) => RemiTcpConnection::from(item),
            | Err(e) => {
                self.listener = None;

                return task::Poll::Ready(Err(e.into()));
            }
        };

        task::Poll::Ready(Ok(item))
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
